/// Seed map tile worker — computes biome RGBA tiles via WASM.
/// Architecture matches chunkbase: one WASM call per tile, returns RGBA directly.

import initWasm, { WorldGen } from '../wasm/mc-worldgen/mc_worldgen';

let wasmReady = false;
let worldgen: WorldGen | null = null;
let currentGeneration = 0;
let currentDimension = 'overworld';

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

			currentDimension = e.data.dimension || 'overworld';
			const edition = e.data.edition || 'java';
			worldgen = new WorldGen(e.data.seedHi, e.data.seedLo, e.data.version, currentDimension, edition);
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

			const tileBlockW = tileSize * step;

			// Slime chunks: overworld only
			let slimeData: Uint8Array | null = null;
			let chunkX0 = 0, chunkZ0 = 0, chunkW = 0, chunkH = 0;
			if (currentDimension === 'overworld') {
				chunkX0 = Math.floor(blockX / 16);
				chunkZ0 = Math.floor(blockZ / 16);
				chunkW = Math.ceil(tileBlockW / 16) + 1;
				chunkH = Math.ceil(tileBlockW / 16) + 1;
				slimeData = worldgen.get_slime_area(chunkX0, chunkZ0, chunkW, chunkH);
			}

			// Structures
			const structData = worldgen.find_structures(blockX, blockZ, tileBlockW, tileBlockW);

			// Transfer the buffers (zero-copy)
			const rgbaBuf = rgba.buffer;
			const idsBuf = biomeIds.buffer;
			const transfers: ArrayBuffer[] = [rgbaBuf, idsBuf];

			const msg: any = {
				type: 'tile-result',
				tileId,
				tileX, tileZ, tileSize, step,
				generation,
				rgba: new Uint8Array(rgbaBuf),
				biomeIds: new Uint8Array(idsBuf),
				structures: Array.from(structData),
			};

			if (slimeData) {
				const slimeBuf = slimeData.buffer;
				msg.slimeChunkX = chunkX0;
				msg.slimeChunkZ = chunkZ0;
				msg.slimeW = chunkW;
				msg.slimeH = chunkH;
				msg.slime = new Uint8Array(slimeBuf);
				transfers.push(slimeBuf);
			}

			post(msg, transfers);
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
