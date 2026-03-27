/// Web Worker for seed map biome/slime chunk computation.
/// Loads the mc-worldgen WASM module and processes chunk requests off the main thread.

import initWasm, { WorldGen } from '../wasm/mc-worldgen/mc_worldgen';

let wasmReady = false;
let worldgen: WorldGen | null = null;
let currentGeneration = 0;

function post(msg: unknown) {
	(self as unknown as Worker).postMessage(msg);
}

self.onmessage = async (e: MessageEvent) => {
	try {
		if (e.data.type === 'init') {
			// Only init WASM once
			if (!wasmReady) {
				await initWasm();
				wasmReady = true;
			}

			// Free previous instance
			if (worldgen) {
				worldgen.free();
			}

			worldgen = new WorldGen(e.data.seedHi, e.data.seedLo, e.data.version);
			currentGeneration = e.data.generation;

			post({ type: 'ready', generation: e.data.generation });
		}

		if (e.data.type === 'compute') {
			if (!worldgen || e.data.generation !== currentGeneration) return;

			const { chunks, generation, resolution } = e.data;
			const numChunks = chunks.length / 2;
			const step = Math.max(1, resolution);
			const gridSize = Math.floor(16 / step);
			const biomesPerChunk = gridSize * gridSize;

			// Process in batches of 16 chunks for progressive updates
			const batchSize = 16;
			for (let batch = 0; batch < numChunks; batch += batchSize) {
				if (currentGeneration !== generation) return; // Stale

				const end = Math.min(batch + batchSize, numChunks);
				const batchCoords = chunks.slice(batch * 2, end * 2);

				const raw = worldgen.compute_chunks(new Int32Array(batchCoords), resolution);

				const results: {
					type: 'chunk-batch';
					generation: number;
					resolution: number;
					chunks: Array<{ cx: number; cz: number; biomes: Uint8Array; slime: boolean }>;
				} = {
					type: 'chunk-batch',
					generation,
					resolution,
					chunks: [],
				};

				let offset = 0;
				for (let i = batch; i < end; i++) {
					const cx = chunks[i * 2];
					const cz = chunks[i * 2 + 1];
					const biomes = raw.slice(offset, offset + biomesPerChunk);
					offset += biomesPerChunk;
					const slime = raw[offset] === 1;
					offset += 1;
					results.chunks.push({ cx, cz, biomes, slime });
				}

				post(results);
			}
		}
	} catch (err) {
		post({
			type: 'error',
			message: err instanceof Error ? err.message : String(err),
		});
	}
};
