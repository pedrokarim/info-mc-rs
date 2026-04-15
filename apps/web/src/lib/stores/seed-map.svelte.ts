/// Seed map store — tile-based architecture matching chunkbase's approach.
/// Multiple workers compute 64x64 RGBA tiles in parallel.
/// Supports multiple independent instances via createSeedMapStore().

import { browser } from '$app/environment';

const TILE_SIZE = 64;
const WORKER_COUNT = Math.min(8, Math.max(2, (typeof navigator !== 'undefined' ? navigator.hardwareConcurrency : 4) - 1));
const LS_KEY = 'seed-map-state';

/** Default enabled structure IDs — used to auto-enable new types added after the user's saved state. */
const DEFAULT_ENABLED_STRUCTURES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24, 25];

/** Schema version — bump when adding new default structure IDs to auto-enable them on load. */
const SCHEMA_VERSION = 2;

// ===== Types =====

export interface TileData {
	rgba: Uint8Array;
	biomeIds: Uint8Array;
	step: number;
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
	showStructures: boolean;
	enabledStructures: Set<number>;
	showGrid: boolean;
	showCoordinates: boolean;

	mcVersion: string;
	dimension: 'overworld' | 'nether' | 'end';

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
	slimeCache: Map<string, Uint8Array>;
	structures: Array<{ type: number; name: string; x: number; z: number }>;
	selectedMarker: { type: number; name: string; x: number; z: number; biome: string } | null;
	renderGeneration: number;

	workersReady: number;
	computing: boolean;
}

// ===== Shared lookup tables (stateless, shared across all instances) =====

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
	60: 'Nether Wastes', 61: 'Soul Sand Valley', 62: 'Crimson Forest',
	63: 'Warped Forest', 64: 'Basalt Deltas',
	70: 'The End', 71: 'End Highlands', 72: 'End Midlands',
	73: 'End Barrens', 74: 'Small End Islands',
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
	60: 0xBF3B3B, 61: 0x5E3830, 62: 0xDD0808, 63: 0x49907B, 64: 0x403636,
	70: 0x8080FF, 71: 0xD5CE8E, 72: 0xB5AE6E, 73: 0x706848, 74: 0x000000,
};

const STRUCTURE_NAMES: Record<number, string> = {
	0: 'Village', 1: 'Desert Temple', 2: 'Jungle Temple', 3: 'Witch Hut',
	4: 'Igloo', 5: 'Ocean Monument', 6: 'Woodland Mansion', 7: 'Pillager Outpost',
	8: 'Stronghold', 9: 'Ocean Ruin', 10: 'Shipwreck', 11: 'Buried Treasure',
	12: 'Ruined Portal', 13: 'Ancient City', 14: 'Trail Ruin', 15: 'Trial Chamber',
	16: 'Nether Fortress', 17: 'Bastion', 18: 'Mineshaft', 19: 'Dungeon',
	20: 'Desert Well', 21: 'Fossil', 23: 'End City', 24: 'End Gateway',
	25: 'End City Ship',
};

const STRUCTURE_ICONS: Record<number, string> = {
	0: 'village', 1: 'desert-temple', 2: 'jungle-temple', 3: 'witch-hut',
	4: 'igloo', 5: 'ocean-monument', 6: 'mansion', 7: 'pillager-outpost',
	8: 'stronghold', 9: 'ocean-ruin', 10: 'shipwreck', 11: 'buried-treasure',
	12: 'ruined-portal', 13: 'ancient-city', 14: 'trail-ruin', 15: 'trial-chamber',
	16: 'nether-fortress', 17: 'bastion-treasure',
};

// ===== Stateless utility functions (shared) =====

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

export function randomSeed(): string {
	const hi = (Math.random() * 0x100000000) >>> 0;
	const lo = (Math.random() * 0x100000000) >>> 0;
	const big = (BigInt(hi) << 32n) | BigInt(lo);
	return BigInt.asIntN(64, big).toString();
}

export function getStructureName(typeId: number): string { return STRUCTURE_NAMES[typeId] ?? 'Unknown'; }
export function getStructureIconName(typeId: number): string { return STRUCTURE_ICONS[typeId] ?? 'unknown'; }
export function getBiomeName(id: number): string { return BIOME_NAMES[id] ?? 'Unknown'; }
export function getBiomeColor(id: number): number { return BIOME_COLORS[id] ?? 0xFF00FF; }

const ZOOM_LEVELS = [0.0625, 0.125, 0.25, 0.5, 1, 2, 4, 8, 16];

// ===== Store instance type =====

export interface SeedMapStore {
	state: MapState;
	initWorkers(): void;
	terminateWorkers(): void;
	setSeed(input: string): void;
	setVersion(version: string): void;
	setDimension(dim: 'overworld' | 'nether' | 'end'): void;
	zoomIn(): void;
	zoomOut(): void;
	zoomToward(canvasX: number, canvasY: number, delta: number): void;
	pan(dx: number, dz: number): void;
	setCenter(x: number, z: number): void;
	getStep(): number;
	getVisibleTileRange(): { minTX: number; maxTX: number; minTZ: number; maxTZ: number; step: number; tileWorldSize: number };
	requestVisibleTiles(): void;
	persistState(): void;
	restoreState(): boolean;
}

// ===== Factory =====

export function createSeedMapStore(): SeedMapStore {
	const state: MapState = $state({
		seedInput: '',
		seedHi: 0,
		seedLo: 0,
		seedValid: false,

		centerX: 0,
		centerZ: 0,
		zoom: 1,

		showBiomes: true,
		showSlimeChunks: true,
		showStructures: true,
		enabledStructures: new Set(DEFAULT_ENABLED_STRUCTURES),
		showGrid: true,
		showCoordinates: true,

		mcVersion: '1.21',
		dimension: 'overworld' as 'overworld' | 'nether' | 'end',

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
		structures: [],
		selectedMarker: null as { type: number; name: string; x: number; z: number; biome: string } | null,
		renderGeneration: 0,

		workersReady: 0,
		computing: false,
	});

	// ── Worker pool (private to this instance) ──

	let workers: Worker[] = [];
	let workerBusy: boolean[] = [];
	let pendingTiles: Array<{ tileX: number; tileZ: number; step: number; dist: number }> = [];
	let tileIdCounter = 0;

	function initWorkers() {
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

	function terminateWorkers() {
		for (const w of workers) w.terminate();
		workers = [];
		workerBusy = [];
		pendingTiles = [];
	}

	function handleWorkerMessage(workerIdx: number, e: MessageEvent) {
		const msg = e.data;

		if (msg.type === 'ready') {
			if (msg.generation === state.renderGeneration) {
				state.workersReady++;
				if (state.workersReady >= WORKER_COUNT) {
					requestVisibleTiles();
				}
			}
		}

		if (msg.type === 'tile-result') {
			workerBusy[workerIdx] = false;
			if (msg.generation === state.renderGeneration) {
				const key = `${msg.tileX},${msg.tileZ},${msg.step}`;
				state.tileCache.set(key, { rgba: msg.rgba, biomeIds: msg.biomeIds, step: msg.step });

				if (msg.structures) {
					const arr = msg.structures as number[];
					for (let i = 0; i < arr.length; i += 3) {
						const typeId = arr[i], bx = arr[i + 1], bz = arr[i + 2];
						if (!state.structures.find(s => s.x === bx && s.z === bz && s.type === typeId)) {
							state.structures.push({ type: typeId, name: STRUCTURE_NAMES[typeId] ?? 'unknown', x: bx, z: bz });
						}
					}
				}

				if (msg.slime) {
					for (let dz = 0; dz < msg.slimeH; dz++) {
						for (let dx = 0; dx < msg.slimeW; dx++) {
							const cx = msg.slimeChunkX + dx, cz = msg.slimeChunkZ + dz;
							if (msg.slime[dz * msg.slimeW + dx] === 1) {
								state.slimeCache.set(`${cx},${cz}`, new Uint8Array([1]));
							}
						}
					}
				}
			}
			dispatchNext(workerIdx);
		}

		if (msg.type === 'slime-result') {
			workerBusy[workerIdx] = false;
			if (msg.generation === state.renderGeneration) {
				state.slimeCache.set(`${msg.chunkX},${msg.chunkZ}`, msg.slime);
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
			if (workerBusy.every(b => !b)) state.computing = false;
			return;
		}
		const tile = pendingTiles.shift()!;
		workerBusy[workerIdx] = true;
		workers[workerIdx].postMessage({
			type: 'tile', tileX: tile.tileX, tileZ: tile.tileZ,
			tileSize: TILE_SIZE, step: tile.step,
			generation: state.renderGeneration, tileId: tileIdCounter++,
		});
	}

	// ── Helpers to reinit workers on seed/version/dimension change ──

	function reinitWorkerState() {
		state.tileCache = new Map();
		state.slimeCache = new Map();
		state.structures = [];
		state.renderGeneration++;
		state.workersReady = 0;
		state.computing = false;
		pendingTiles = [];
	}

	function postInitToWorkers() {
		for (const w of workers) {
			w.postMessage({
				type: 'init',
				seedHi: state.seedHi, seedLo: state.seedLo,
				version: state.mcVersion, dimension: state.dimension,
				generation: state.renderGeneration,
			});
		}
	}

	// ── Seed / version / dimension ──

	function setSeed(input: string) {
		state.seedInput = input;
		if (!input.trim()) { state.seedValid = false; return; }
		try {
			const { hi, lo } = parseSeed(input);
			state.seedHi = hi;
			state.seedLo = lo;
			state.seedValid = true;
			reinitWorkerState();
			postInitToWorkers();
		} catch (err) {
			console.error('[seed-map] setSeed error:', err);
			state.seedValid = false;
		}
	}

	function setVersion(version: string) {
		state.mcVersion = version;
		if (state.seedValid) { reinitWorkerState(); postInitToWorkers(); }
	}

	function setDimension(dim: 'overworld' | 'nether' | 'end') {
		state.dimension = dim;
		if (state.seedValid) { reinitWorkerState(); postInitToWorkers(); }
	}

	// ── Viewport ──

	function zoomIn() {
		const idx = ZOOM_LEVELS.indexOf(state.zoom);
		if (idx < ZOOM_LEVELS.length - 1) state.zoom = ZOOM_LEVELS[idx + 1];
	}

	function zoomOut() {
		const idx = ZOOM_LEVELS.indexOf(state.zoom);
		if (idx > 0) state.zoom = ZOOM_LEVELS[idx - 1];
	}

	function zoomToward(canvasX: number, canvasY: number, delta: number) {
		const oldZoom = state.zoom;
		const idx = ZOOM_LEVELS.indexOf(oldZoom);
		const newIdx = delta < 0 ? Math.min(idx + 1, ZOOM_LEVELS.length - 1) : Math.max(idx - 1, 0);
		const newZoom = ZOOM_LEVELS[newIdx];
		if (newZoom === oldZoom) return;
		const cx = state.canvasWidth / 2, cy = state.canvasHeight / 2;
		const worldX = state.centerX + (canvasX - cx) / oldZoom;
		const worldZ = state.centerZ + (canvasY - cy) / oldZoom;
		state.centerX = worldX - (canvasX - cx) / newZoom;
		state.centerZ = worldZ - (canvasY - cy) / newZoom;
		state.zoom = newZoom;
	}

	function pan(dx: number, dz: number) { state.centerX += dx; state.centerZ += dz; }
	function setCenter(x: number, z: number) { state.centerX = x; state.centerZ = z; }

	// ── Tile system ──

	function getStep(): number {
		if (state.zoom >= 1) return 4;
		if (state.zoom >= 0.25) return 16;
		if (state.zoom >= 0.0625) return 64;
		return 128;
	}

	function getVisibleTileRange() {
		const step = getStep();
		const tileWorldSize = TILE_SIZE * step;
		const halfW = state.canvasWidth / 2 / state.zoom;
		const halfH = state.canvasHeight / 2 / state.zoom;
		return {
			minTX: Math.floor((state.centerX - halfW) / tileWorldSize) - 1,
			maxTX: Math.floor((state.centerX + halfW) / tileWorldSize) + 1,
			minTZ: Math.floor((state.centerZ - halfH) / tileWorldSize) - 1,
			maxTZ: Math.floor((state.centerZ + halfH) / tileWorldSize) + 1,
			step, tileWorldSize,
		};
	}

	function requestVisibleTiles() {
		if (workers.length === 0 || state.workersReady < WORKER_COUNT || !state.seedValid) return;
		const range = getVisibleTileRange();
		const centerTX = Math.floor(state.centerX / range.tileWorldSize);
		const centerTZ = Math.floor(state.centerZ / range.tileWorldSize);
		pendingTiles = [];
		for (let tx = range.minTX; tx <= range.maxTX; tx++) {
			for (let tz = range.minTZ; tz <= range.maxTZ; tz++) {
				const key = `${tx},${tz},${range.step}`;
				if (!state.tileCache.has(key)) {
					const dx = tx - centerTX, dz = tz - centerTZ;
					pendingTiles.push({ tileX: tx, tileZ: tz, step: range.step, dist: dx * dx + dz * dz });
				}
			}
		}
		if (pendingTiles.length === 0) return;
		pendingTiles.sort((a, b) => a.dist - b.dist);
		state.computing = true;
		for (let i = 0; i < workers.length; i++) {
			if (!workerBusy[i] && pendingTiles.length > 0) dispatchNext(i);
		}
	}

	// ── Persistence ──

	function persistState() {
		if (!browser) return;
		const params = new URLSearchParams();
		if (state.seedInput) params.set('seed', state.seedInput);
		params.set('v', state.mcVersion);
		params.set('dim', state.dimension);
		params.set('x', Math.round(state.centerX).toString());
		params.set('z', Math.round(state.centerZ).toString());
		params.set('zoom', state.zoom.toString());
		if (!state.showBiomes) params.set('biomes', '0');
		if (!state.showSlimeChunks) params.set('slime', '0');
		if (!state.showStructures) params.set('structs', '0');
		if (!state.showGrid) params.set('grid', '0');
		if (!state.showCoordinates) params.set('coords', '0');
		params.set('st', [...state.enabledStructures].sort().join(','));
		params.set('sv', SCHEMA_VERSION.toString());
		const hash = '#' + params.toString();
		if (window.location.hash !== hash) history.replaceState(null, '', hash);
		try {
			localStorage.setItem(LS_KEY, JSON.stringify({
				schemaVersion: SCHEMA_VERSION, seed: state.seedInput,
				version: state.mcVersion, dimension: state.dimension,
				x: Math.round(state.centerX), z: Math.round(state.centerZ), zoom: state.zoom,
				showBiomes: state.showBiomes, showSlimeChunks: state.showSlimeChunks,
				showStructures: state.showStructures, showGrid: state.showGrid,
				showCoordinates: state.showCoordinates,
				enabledStructures: [...state.enabledStructures],
			}));
		} catch { /* quota exceeded */ }
	}

	function restoreState(): boolean {
		if (!browser) return false;
		const hash = window.location.hash.slice(1);
		if (hash) {
			const params = new URLSearchParams(hash);
			const seed = params.get('seed');
			if (seed) {
				state.seedInput = seed;
				state.mcVersion = params.get('v') || state.mcVersion;
				state.dimension = (params.get('dim') as typeof state.dimension) || state.dimension;
				state.centerX = parseFloat(params.get('x') || '0');
				state.centerZ = parseFloat(params.get('z') || '0');
				state.zoom = parseFloat(params.get('zoom') || '1');
				if (params.get('biomes') === '0') state.showBiomes = false;
				if (params.get('slime') === '0') state.showSlimeChunks = false;
				if (params.get('structs') === '0') state.showStructures = false;
				if (params.get('grid') === '0') state.showGrid = false;
				if (params.get('coords') === '0') state.showCoordinates = false;
				const st = params.get('st');
				const savedVersion = parseInt(params.get('sv') || '0');
				if (st) {
					const saved = new Set(st.split(',').map(Number).filter(n => !isNaN(n)));
					if (savedVersion < SCHEMA_VERSION) { for (const id of DEFAULT_ENABLED_STRUCTURES) saved.add(id); }
					state.enabledStructures = saved;
				}
				return true;
			}
		}
		try {
			const raw = localStorage.getItem(LS_KEY);
			if (raw) {
				const saved = JSON.parse(raw);
				if (saved.seed) {
					state.seedInput = saved.seed;
					state.mcVersion = saved.version || state.mcVersion;
					state.dimension = saved.dimension || state.dimension;
					state.centerX = saved.x ?? 0; state.centerZ = saved.z ?? 0; state.zoom = saved.zoom ?? 1;
					state.showBiomes = saved.showBiomes ?? true;
					state.showSlimeChunks = saved.showSlimeChunks ?? true;
					state.showStructures = saved.showStructures ?? true;
					state.showGrid = saved.showGrid ?? true;
					state.showCoordinates = saved.showCoordinates ?? true;
					if (saved.enabledStructures) {
						const restoredSet = new Set<number>(saved.enabledStructures);
						if ((saved.schemaVersion ?? 0) < SCHEMA_VERSION) {
							for (const id of DEFAULT_ENABLED_STRUCTURES) restoredSet.add(id);
						}
						state.enabledStructures = restoredSet;
					}
					return true;
				}
			}
		} catch { /* corrupt data */ }
		return false;
	}

	return {
		state, initWorkers, terminateWorkers, setSeed, setVersion, setDimension,
		zoomIn, zoomOut, zoomToward, pan, setCenter,
		getStep, getVisibleTileRange, requestVisibleTiles,
		persistState, restoreState,
	};
}

// ===== Default singleton (backward-compatible) =====

const defaultStore = createSeedMapStore();

export const mapState = defaultStore.state;
export const initWorkers = defaultStore.initWorkers;
export const terminateWorkers = defaultStore.terminateWorkers;
export const setSeed = defaultStore.setSeed;
export const setVersion = defaultStore.setVersion;
export const setDimension = defaultStore.setDimension;
export const zoomIn = defaultStore.zoomIn;
export const zoomOut = defaultStore.zoomOut;
export const zoomToward = defaultStore.zoomToward;
export const pan = defaultStore.pan;
export const setCenter = defaultStore.setCenter;
export const getStep = defaultStore.getStep;
export const getVisibleTileRange = defaultStore.getVisibleTileRange;
export const requestVisibleTiles = defaultStore.requestVisibleTiles;
export const persistState = defaultStore.persistState;
export const restoreState = defaultStore.restoreState;

export { TILE_SIZE };
