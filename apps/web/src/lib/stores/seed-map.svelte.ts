export interface ChunkData {
	biomes: Uint8Array;
	slime: boolean;
	resolution: number; // block step size used when computing this chunk
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

	chunkCache: Map<string, ChunkData>;
	renderGeneration: number;

	workerReady: boolean;
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

	chunkCache: new Map(),
	renderGeneration: 0,

	workerReady: false,
	computing: false,
});

// ===== Worker management =====

let worker: Worker | null = null;

export function initWorker() {
	if (worker) worker.terminate();

	worker = new Worker(
		new URL('../workers/seed-map-worker.ts', import.meta.url),
		{ type: 'module' }
	);

	worker.onmessage = handleWorkerMessage;
	worker.onerror = (e) => console.error('[seed-map-worker] error:', e);
}

export function terminateWorker() {
	if (worker) {
		worker.terminate();
		worker = null;
	}
}

function handleWorkerMessage(e: MessageEvent) {
	const msg = e.data;

	if (msg.type === 'ready') {
		if (msg.generation === mapState.renderGeneration) {
			mapState.workerReady = true;
			mapState.computing = false;
			requestVisibleChunks();
		}
	}

	if (msg.type === 'chunk-batch') {
		if (msg.generation !== mapState.renderGeneration) return;

		for (const chunk of msg.chunks) {
			const key = `${chunk.cx},${chunk.cz}`;
			mapState.chunkCache.set(key, {
				biomes: chunk.biomes,
				slime: chunk.slime,
				resolution: msg.resolution,
			});
		}
		mapState.computing = false;
		// Request more chunks if there are still uncached ones visible
		requestVisibleChunks();
	}

	if (msg.type === 'error') {
		console.error('[seed-map-worker]', msg.message);
		mapState.computing = false;
	}
}

// ===== Seed parsing (pure JS, no WASM needed) =====

/** Java's String.hashCode() */
function javaStringHashCode(s: string): number {
	let hash = 0;
	for (const ch of s) {
		hash = (Math.imul(hash, 31) + ch.charCodeAt(0)) | 0;
	}
	return hash;
}

/** Parse seed string → {hi, lo} i32 pair for WASM. */
export function parseSeed(input: string): { hi: number; lo: number } {
	let seed: bigint;

	// Try parsing as number first
	try {
		seed = BigInt(input);
	} catch {
		// Text seed — use Java hashCode (returns i32, which is also a valid i64 seed)
		seed = BigInt(javaStringHashCode(input));
	}

	// Split i64 into two i32s
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
		mapState.chunkCache = new Map();
		mapState.renderGeneration++;
		mapState.workerReady = false;
		mapState.computing = false;

		worker?.postMessage({
			type: 'init',
			seedHi: hi,
			seedLo: lo,
			version: mapState.mcVersion,
			generation: mapState.renderGeneration,
		});
	} catch (err) {
		console.error('[seed-map] setSeed error:', err);
		mapState.seedValid = false;
	}
}

export function setVersion(version: string) {
	mapState.mcVersion = version;
	if (mapState.seedValid) {
		mapState.chunkCache = new Map();
		mapState.renderGeneration++;
		mapState.workerReady = false;
		mapState.computing = false;

		worker?.postMessage({
			type: 'init',
			seedHi: mapState.seedHi,
			seedLo: mapState.seedLo,
			version,
			generation: mapState.renderGeneration,
		});
	}
}

// ===== Viewport =====

const ZOOM_LEVELS = [0.125, 0.25, 0.5, 1, 2, 4, 8, 16];

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

// ===== Chunk requests =====

export function getVisibleChunkRange() {
	const halfW = mapState.canvasWidth / 2 / mapState.zoom;
	const halfH = mapState.canvasHeight / 2 / mapState.zoom;

	return {
		minCX: Math.floor((mapState.centerX - halfW) / 16) - 1,
		maxCX: Math.floor((mapState.centerX + halfW) / 16) + 1,
		minCZ: Math.floor((mapState.centerZ - halfH) / 16) - 1,
		maxCZ: Math.floor((mapState.centerZ + halfH) / 16) + 1,
	};
}

export function getResolution(): number {
	if (mapState.zoom <= 0.25) return 8;
	if (mapState.zoom <= 1) return 4;
	return 1;
}

export function requestVisibleChunks() {
	if (!worker || !mapState.workerReady || !mapState.seedValid) return;
	if (mapState.computing) return;

	const range = getVisibleChunkRange();
	const centerCX = Math.floor(mapState.centerX / 16);
	const centerCZ = Math.floor(mapState.centerZ / 16);

	const pending: Array<{ cx: number; cz: number; dist: number }> = [];

	const currentRes = getResolution();

	for (let cx = range.minCX; cx <= range.maxCX; cx++) {
		for (let cz = range.minCZ; cz <= range.maxCZ; cz++) {
			const key = `${cx},${cz}`;
			const cached = mapState.chunkCache.get(key);
			// Re-compute if not cached or cached at a coarser resolution than needed
			if (!cached || cached.resolution > currentRes) {
				const dx = cx - centerCX;
				const dz = cz - centerCZ;
				pending.push({ cx, cz, dist: dx * dx + dz * dz });
			}
		}
	}

	if (pending.length === 0) return;

	pending.sort((a, b) => a.dist - b.dist);

	const chunks: number[] = [];
	const batch = pending.slice(0, 256);
	for (const p of batch) {
		chunks.push(p.cx, p.cz);
	}

	mapState.computing = true;
	worker.postMessage({
		type: 'compute',
		chunks,
		generation: mapState.renderGeneration,
		resolution: getResolution(),
	});
}

// ===== Biome info (pure JS lookup, no WASM on main thread) =====

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
