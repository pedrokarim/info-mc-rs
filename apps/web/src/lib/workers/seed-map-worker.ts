/// Seed map tile worker — computes biome RGBA tiles via WASM.
/// Architecture matches chunkbase: one WASM call per tile, returns RGBA directly.

import initWasm, { WorldGen } from '../wasm/mc-worldgen/mc_worldgen';

let wasmReady = false;
let worldgen: WorldGen | null = null;
let currentGeneration = 0;

function post(msg: unknown, transfer?: Transferable[]) {
	(self as unknown as Worker).postMessage(msg, transfer ?? []);
}

self.onmessage = async (e: MessageEvent) => {
	try {
		const { type } = e.data;

		if (type === 'init') {
			if (!wasmReady) {
				await initWasm();
				wasmReady = true;
			}
			if (worldgen) worldgen.free();

			worldgen = new WorldGen(e.data.seedHi, e.data.seedLo, e.data.version);
			currentGeneration = e.data.generation;
			post({ type: 'ready', generation: e.data.generation });
		}

		if (type === 'tile') {
			if (!worldgen || e.data.generation !== currentGeneration) return;

			const { tileX, tileZ, tileSize, step, generation, tileId } = e.data;

			// One WASM call: compute biome RGBA for the entire tile
			const blockX = tileX * step * tileSize;
			const blockZ = tileZ * step * tileSize;
			const rgba = worldgen.get_biome_area_rgba(blockX, blockZ, tileSize, tileSize, step);

			// Also get biome IDs for tooltip lookups
			const biomeIds = worldgen.get_biome_area(blockX, blockZ, tileSize, tileSize, step);

			// Transfer the buffers (zero-copy)
			const rgbaBuf = rgba.buffer;
			const idsBuf = biomeIds.buffer;

			post({
				type: 'tile-result',
				tileId,
				tileX, tileZ, tileSize, step,
				generation,
				rgba: new Uint8Array(rgbaBuf),
				biomeIds: new Uint8Array(idsBuf),
			}, [rgbaBuf, idsBuf]);
		}

		if (type === 'slime-area') {
			if (!worldgen || e.data.generation !== currentGeneration) return;

			const { chunkX, chunkZ, width, height, generation } = e.data;
			const slime = worldgen.get_slime_area(chunkX, chunkZ, width, height);
			const buf = slime.buffer;

			post({
				type: 'slime-result',
				chunkX, chunkZ, width, height,
				generation,
				slime: new Uint8Array(buf),
			}, [buf]);
		}

	} catch (err) {
		post({
			type: 'error',
			message: err instanceof Error ? err.message : String(err),
		});
	}
};
