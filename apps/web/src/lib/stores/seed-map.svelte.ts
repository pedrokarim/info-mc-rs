/// Seed map store — tile-based architecture matching chunkbase's approach.
/// Multiple workers compute 64x64 RGBA tiles in parallel.

const TILE_SIZE = 64; // samples per tile axis (64x64 = 4096 samples)
const WORKER_COUNT = Math.min(8, Math.max(2, (navigator?.hardwareConcurrency ?? 4) - 1));

// ===== Tile cache =====

export interface TileData {
	rgba: Uint8Array;      // TILE_SIZE * TILE_SIZE * 4 bytes
	biomeIds: Uint8Array;  // TILE_SIZE * TILE_SIZE biome IDs
	step: number;          // block step used
}

export interface MapState {
	seedInput: string;
	seedHi: number;
	seedLo: number;
	seedValid: boolean;

	centerX: number;
	centerZ: number;
	zoom: number;

	showBiomes: boolean;
	showSlimeChunks: boolean;
	showGrid: boolean;
	showCoordinates: boolean;

	mcVersion: string;

	hoverWorldX: number;
	hoverWorldZ: number;
	hoverChunkX: number;
	hoverChunkZ: number;
	hoverBiome: string;
	hoverIsSlime: boolean;
	hoverActive: boolean;

	canvasWidth: number;
	canvasHeight: number;

	tileCache: Map<string, TileData>;
	slimeCache: Map<string, Uint8Array>; // keyed by "regionX,regionZ"
	renderGeneration: number;

	workersReady: number;
	computing: boolean;
}

export const mapState: MapState = $state({
	seedInput: '',
	seedHi: 0,
	seedLo: 0,
	seedValid: false,

	centerX: 0,
	centerZ: 0,
	zoom: 1,

	showBiomes: true,
	showSlimeChunks: true,
	showGrid: true,
	showCoordinates: true,

	mcVersion: '1.21',

	hoverWorldX: 0,
	hoverWorldZ: 0,
	hoverChunkX: 0,
	hoverChunkZ: 0,
	hoverBiome: '',
	hoverIsSlime: false,
	hoverActive: false,

	canvasWidth: 0,
	canvasHeight: 0,

	tileCache: new Map(),
	slimeCache: new Map(),
	renderGeneration: 0,

	workersReady: 0,
	computing: false,
});

// ===== Worker pool =====

let workers: Worker[] = [];
let workerBusy: boolean[] = [];
let pendingTiles: Array<{ tileX: number; tileZ: number; step: number; dist: number }> = [];
let tileIdCounter = 0;

export function initWorkers() {
	terminateWorkers();

	for (let i = 0; i < WORKER_COUNT; i++) {
		const w = new Worker(
			new URL('../workers/seed-map-worker.ts', import.meta.url),
			{ type: 'module' }
		);
		w.onmessage = (e) => handleWorkerMessage(i, e);
		w.onerror = (e) => console.error(`[seed-map-worker-${i}]`, e);
		workers.push(w);
		workerBusy.push(false);
	}
}

export function terminateWorkers() {
	for (const w of workers) w.terminate();
	workers = [];
	workerBusy = [];
	pendingTiles = [];
}

function handleWorkerMessage(workerIdx: number, e: MessageEvent) {
	const msg = e.data;

	if (msg.type === 'ready') {
		if (msg.generation === mapState.renderGeneration) {
			mapState.workersReady++;
			if (mapState.workersReady >= WORKER_COUNT) {
				requestVisibleTiles();
			}
		}
	}

	if (msg.type === 'tile-result') {
		workerBusy[workerIdx] = false;

		if (msg.generation === mapState.renderGeneration) {
			const key = `${msg.tileX},${msg.tileZ},${msg.step}`;
			mapState.tileCache.set(key, {
				rgba: msg.rgba,
				biomeIds: msg.biomeIds,
				step: msg.step,
			});
		}

		// Dispatch next pending tile to this worker
		dispatchNext(workerIdx);
	}

	if (msg.type === 'slime-result') {
		workerBusy[workerIdx] = false;
		if (msg.generation === mapState.renderGeneration) {
			const key = `${msg.chunkX},${msg.chunkZ}`;
			mapState.slimeCache.set(key, msg.slime);
		}
		dispatchNext(workerIdx);
	}

	if (msg.type === 'error') {
		workerBusy[workerIdx] = false;
		console.error(`[worker-${workerIdx}]`, msg.message);
		dispatchNext(workerIdx);
	}
}

function dispatchNext(workerIdx: number) {
	if (pendingTiles.length === 0) {
		// Check if all workers idle
		if (workerBusy.every(b => !b)) {
			mapState.computing = false;
		}
		return;
	}

	const tile = pendingTiles.shift()!;
	workerBusy[workerIdx] = true;

	workers[workerIdx].postMessage({
		type: 'tile',
		tileX: tile.tileX,
		tileZ: tile.tileZ,
		tileSize: TILE_SIZE,
		step: tile.step,
		generation: mapState.renderGeneration,
		tileId: tileIdCounter++,
	});
}

// ===== Seed / version =====

function javaStringHashCode(s: string): number {
	let hash = 0;
	for (const ch of s) {
		hash = (Math.imul(hash, 31) + ch.charCodeAt(0)) | 0;
	}
	return hash;
}

export function parseSeed(input: string): { hi: number; lo: number } {
	let seed: bigint;
	try {
		seed = BigInt(input);
	} catch {
		seed = BigInt(javaStringHashCode(input));
	}
	const lo = Number(BigInt.asIntN(32, seed));
	const hi = Number(BigInt.asIntN(32, seed >> 32n));
	return { hi, lo };
}

export function setSeed(input: string) {
	mapState.seedInput = input;

	if (!input.trim()) {
		mapState.seedValid = false;
		return;
	}

	try {
		const { hi, lo } = parseSeed(input);
		mapState.seedHi = hi;
		mapState.seedLo = lo;
		mapState.seedValid = true;
		mapState.tileCache = new Map();
		mapState.slimeCache = new Map();
		mapState.renderGeneration++;
		mapState.workersReady = 0;
		mapState.computing = false;
		pendingTiles = [];

		// Init all workers with new seed
		for (const w of workers) {
			w.postMessage({
				type: 'init',
				seedHi: hi,
				seedLo: lo,
				version: mapState.mcVersion,
				generation: mapState.renderGeneration,
			});
		}
	} catch (err) {
		console.error('[seed-map] setSeed error:', err);
		mapState.seedValid = false;
	}
}

export function setVersion(version: string) {
	mapState.mcVersion = version;
	if (mapState.seedValid) {
		mapState.tileCache = new Map();
		mapState.slimeCache = new Map();
		mapState.renderGeneration++;
		mapState.workersReady = 0;
		mapState.computing = false;
		pendingTiles = [];

		for (const w of workers) {
			w.postMessage({
				type: 'init',
				seedHi: mapState.seedHi,
				seedLo: mapState.seedLo,
				version,
				generation: mapState.renderGeneration,
			});
		}
	}
}

// ===== Viewport =====

const ZOOM_LEVELS = [0.0625, 0.125, 0.25, 0.5, 1, 2, 4, 8, 16];

export function zoomIn() {
	const idx = ZOOM_LEVELS.indexOf(mapState.zoom);
	if (idx < ZOOM_LEVELS.length - 1) {
		mapState.zoom = ZOOM_LEVELS[idx + 1];
	}
}

export function zoomOut() {
	const idx = ZOOM_LEVELS.indexOf(mapState.zoom);
	if (idx > 0) {
		mapState.zoom = ZOOM_LEVELS[idx - 1];
	}
}

export function zoomToward(canvasX: number, canvasY: number, delta: number) {
	const oldZoom = mapState.zoom;
	const idx = ZOOM_LEVELS.indexOf(oldZoom);
	const newIdx = delta < 0
		? Math.min(idx + 1, ZOOM_LEVELS.length - 1)
		: Math.max(idx - 1, 0);
	const newZoom = ZOOM_LEVELS[newIdx];

	if (newZoom === oldZoom) return;

	const cx = mapState.canvasWidth / 2;
	const cy = mapState.canvasHeight / 2;
	const worldX = mapState.centerX + (canvasX - cx) / oldZoom;
	const worldZ = mapState.centerZ + (canvasY - cy) / oldZoom;

	mapState.centerX = worldX - (canvasX - cx) / newZoom;
	mapState.centerZ = worldZ - (canvasY - cy) / newZoom;
	mapState.zoom = newZoom;
}

export function pan(dx: number, dz: number) {
	mapState.centerX += dx;
	mapState.centerZ += dz;
}

export function setCenter(x: number, z: number) {
	mapState.centerX = x;
	mapState.centerZ = z;
}

// ===== Tile system =====

/** Get the block-step for the current zoom level.
 * Like chunkbase: higher zoom = finer resolution. */
export function getStep(): number {
	// At zoom 16 (most zoomed in): step=4 (native biome res)
	// At zoom 1: step=4
	// At zoom 0.25: step=16
	// At zoom 0.0625: step=64
	if (mapState.zoom >= 1) return 4;
	if (mapState.zoom >= 0.25) return 16;
	if (mapState.zoom >= 0.0625) return 64;
	return 128;
}

/** Get visible tile range for the current viewport. */
export function getVisibleTileRange() {
	const step = getStep();
	const tileWorldSize = TILE_SIZE * step; // size of one tile in blocks

	const halfW = mapState.canvasWidth / 2 / mapState.zoom;
	const halfH = mapState.canvasHeight / 2 / mapState.zoom;

	const minBlockX = mapState.centerX - halfW;
	const maxBlockX = mapState.centerX + halfW;
	const minBlockZ = mapState.centerZ - halfH;
	const maxBlockZ = mapState.centerZ + halfH;

	return {
		minTX: Math.floor(minBlockX / tileWorldSize) - 1,
		maxTX: Math.floor(maxBlockX / tileWorldSize) + 1,
		minTZ: Math.floor(minBlockZ / tileWorldSize) - 1,
		maxTZ: Math.floor(maxBlockZ / tileWorldSize) + 1,
		step,
		tileWorldSize,
	};
}

export function requestVisibleTiles() {
	if (workers.length === 0 || mapState.workersReady < WORKER_COUNT || !mapState.seedValid) return;

	const range = getVisibleTileRange();
	const centerTX = Math.floor(mapState.centerX / range.tileWorldSize);
	const centerTZ = Math.floor(mapState.centerZ / range.tileWorldSize);

	// Clear old pending tiles
	pendingTiles = [];

	for (let tx = range.minTX; tx <= range.maxTX; tx++) {
		for (let tz = range.minTZ; tz <= range.maxTZ; tz++) {
			const key = `${tx},${tz},${range.step}`;
			if (!mapState.tileCache.has(key)) {
				const dx = tx - centerTX;
				const dz = tz - centerTZ;
				pendingTiles.push({ tileX: tx, tileZ: tz, step: range.step, dist: dx * dx + dz * dz });
			}
		}
	}

	if (pendingTiles.length === 0) return;

	// Sort by distance to center (closest first)
	pendingTiles.sort((a, b) => a.dist - b.dist);

	mapState.computing = true;

	// Dispatch to all idle workers
	for (let i = 0; i < workers.length; i++) {
		if (!workerBusy[i] && pendingTiles.length > 0) {
			dispatchNext(i);
		}
	}
}

// ===== Biome info =====

const BIOME_NAMES: Record<number, string> = {
	0: 'Ocean', 1: 'Deep Ocean', 2: 'Frozen Ocean', 3: 'Deep Frozen Ocean',
	4: 'Cold Ocean', 5: 'Deep Cold Ocean', 6: 'Lukewarm Ocean', 7: 'Deep Lukewarm Ocean',
	8: 'Warm Ocean', 9: 'Plains', 10: 'Sunflower Plains', 11: 'Snowy Plains',
	12: 'Ice Spikes', 13: 'Desert', 14: 'Swamp', 15: 'Mangrove Swamp',
	16: 'Forest', 17: 'Flower Forest', 18: 'Birch Forest', 19: 'Old Growth Birch Forest',
	20: 'Dark Forest', 21: 'Taiga', 22: 'Old Growth Pine Taiga', 23: 'Old Growth Spruce Taiga',
	24: 'Snowy Taiga', 25: 'Savanna', 26: 'Savanna Plateau', 27: 'Windswept Savanna',
	28: 'Jungle', 29: 'Sparse Jungle', 30: 'Bamboo Jungle', 31: 'Badlands',
	32: 'Eroded Badlands', 33: 'Wooded Badlands', 34: 'Meadow', 35: 'Cherry Grove',
	36: 'Grove', 37: 'Snowy Slopes', 38: 'Frozen Peaks', 39: 'Jagged Peaks',
	40: 'Stony Peaks', 41: 'Windswept Hills', 42: 'Windswept Gravelly Hills',
	43: 'Windswept Forest', 44: 'River', 45: 'Frozen River', 46: 'Beach',
	47: 'Snowy Beach', 48: 'Stony Shore', 49: 'Mushroom Fields',
};

const BIOME_COLORS: Record<number, number> = {
	0: 0x000070, 1: 0x000030, 2: 0x7070D6, 3: 0x404090,
	4: 0x2020D0, 5: 0x202080, 6: 0x0000AC, 7: 0x000050,
	8: 0x0000FF, 9: 0x8DB360, 10: 0xB5DB88, 11: 0xFFFFFF,
	12: 0xB4DCDC, 13: 0xFA9418, 14: 0x07F9B2, 15: 0x67A54A,
	16: 0x056621, 17: 0x2D8E49, 18: 0x307444, 19: 0x589C6C,
	20: 0x40511A, 21: 0x0B6659, 22: 0x596651, 23: 0x818E79,
	24: 0x31554A, 25: 0xBDB25F, 26: 0xA79D64, 27: 0xE5DA87,
	28: 0x537B09, 29: 0x628B17, 30: 0x768E14, 31: 0xD94515,
	32: 0xFF6D3D, 33: 0xB09765, 34: 0x83BB6D, 35: 0xE0A0B5,
	36: 0x537D4C, 37: 0xD2E7E7, 38: 0xC4D8E0, 39: 0xC0C0C0,
	40: 0x7E7E7E, 41: 0x606060, 42: 0x787878, 43: 0x507050,
	44: 0x0000FF, 45: 0xA0A0FF, 46: 0xFADE55, 47: 0xFAF0C0,
	48: 0xA2A284, 49: 0xFF00FF,
};

export function getBiomeName(id: number): string {
	return BIOME_NAMES[id] ?? 'Unknown';
}

export function getBiomeColor(id: number): number {
	return BIOME_COLORS[id] ?? 0xFF00FF;
}

export { TILE_SIZE };
