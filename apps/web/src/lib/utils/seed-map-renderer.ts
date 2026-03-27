import type { MapState, ChunkData } from '$lib/stores/seed-map.svelte';
import { getBiomeColor, getVisibleChunkRange, getResolution } from '$lib/stores/seed-map.svelte';

// ===== Coordinate transforms =====

export function worldToCanvas(
	worldX: number,
	worldZ: number,
	state: MapState
): { x: number; y: number } {
	const cx = state.canvasWidth / 2;
	const cy = state.canvasHeight / 2;
	return {
		x: cx + (worldX - state.centerX) * state.zoom,
		y: cy + (worldZ - state.centerZ) * state.zoom,
	};
}

export function canvasToWorld(
	canvasX: number,
	canvasY: number,
	state: MapState
): { x: number; z: number } {
	const cx = state.canvasWidth / 2;
	const cy = state.canvasHeight / 2;
	return {
		x: state.centerX + (canvasX - cx) / state.zoom,
		z: state.centerZ + (canvasY - cy) / state.zoom,
	};
}

// ===== Main render =====

export function renderFrame(ctx: CanvasRenderingContext2D, state: MapState) {
	ctx.clearRect(0, 0, state.canvasWidth, state.canvasHeight);

	// Background
	ctx.fillStyle = '#1a1a2e';
	ctx.fillRect(0, 0, state.canvasWidth, state.canvasHeight);

	if (!state.seedValid) {
		renderPlaceholder(ctx, state);
		return;
	}

	if (state.showBiomes) {
		renderBiomes(ctx, state);
	}

	if (state.showSlimeChunks) {
		renderSlimeChunks(ctx, state);
	}

	if (state.showGrid) {
		renderGrid(ctx, state);
	}

	renderCrosshair(ctx, state);

	if (state.showCoordinates) {
		renderCoordinateOverlay(ctx, state);
	}
}

function renderPlaceholder(ctx: CanvasRenderingContext2D, state: MapState) {
	ctx.fillStyle = '#ffffff40';
	ctx.font = '16px system-ui, sans-serif';
	ctx.textAlign = 'center';
	ctx.fillText(
		'Entrez une seed pour commencer',
		state.canvasWidth / 2,
		state.canvasHeight / 2
	);
}

// ===== Biome rendering =====

// Color cache to avoid re-converting hex each frame
const colorCache = new Map<number, string>();

function biomeColorStr(id: number): string {
	let c = colorCache.get(id);
	if (!c) {
		const rgb = getBiomeColor(id);
		c = `#${rgb.toString(16).padStart(6, '0')}`;
		colorCache.set(id, c);
	}
	return c;
}

function renderBiomes(ctx: CanvasRenderingContext2D, state: MapState) {
	const range = getVisibleChunkRange();
	const blockPx = state.zoom;

	for (let cx = range.minCX; cx <= range.maxCX; cx++) {
		for (let cz = range.minCZ; cz <= range.maxCZ; cz++) {
			const key = `${cx},${cz}`;
			const chunk = state.chunkCache.get(key);
			if (!chunk) continue;

			// Use the resolution this chunk was actually computed at
			const res = chunk.resolution;
			const gridSize = Math.floor(16 / res);
			const cellPx = blockPx * res;
			const baseWorldX = cx * 16;
			const baseWorldZ = cz * 16;

			for (let dz = 0; dz < gridSize; dz++) {
				for (let dx = 0; dx < gridSize; dx++) {
					const biomeId = chunk.biomes[dz * gridSize + dx];
					const worldX = baseWorldX + dx * res;
					const worldZ = baseWorldZ + dz * res;
					const screen = worldToCanvas(worldX, worldZ, state);

					ctx.fillStyle = biomeColorStr(biomeId);
					ctx.fillRect(
						Math.floor(screen.x),
						Math.floor(screen.y),
						Math.ceil(cellPx) + 1,
						Math.ceil(cellPx) + 1
					);
				}
			}
		}
	}
}

// ===== Slime chunks =====

function renderSlimeChunks(ctx: CanvasRenderingContext2D, state: MapState) {
	const range = getVisibleChunkRange();
	const chunkPx = 16 * state.zoom;

	ctx.fillStyle = 'rgba(80, 255, 80, 0.25)';
	ctx.strokeStyle = 'rgba(80, 255, 80, 0.6)';
	ctx.lineWidth = 1;

	for (let cx = range.minCX; cx <= range.maxCX; cx++) {
		for (let cz = range.minCZ; cz <= range.maxCZ; cz++) {
			const key = `${cx},${cz}`;
			const chunk = state.chunkCache.get(key);
			if (!chunk || !chunk.slime) continue;

			const screen = worldToCanvas(cx * 16, cz * 16, state);
			ctx.fillRect(screen.x, screen.y, chunkPx, chunkPx);
			ctx.strokeRect(screen.x + 0.5, screen.y + 0.5, chunkPx - 1, chunkPx - 1);
		}
	}
}

// ===== Grid =====

function renderGrid(ctx: CanvasRenderingContext2D, state: MapState) {
	const range = getVisibleChunkRange();
	const chunkPx = 16 * state.zoom;

	// Only draw chunk grid if chunks are big enough to see
	if (chunkPx >= 8) {
		ctx.strokeStyle = 'rgba(255, 255, 255, 0.08)';
		ctx.lineWidth = 0.5;

		ctx.beginPath();
		for (let cx = range.minCX; cx <= range.maxCX + 1; cx++) {
			const screen = worldToCanvas(cx * 16, range.minCZ * 16, state);
			const screenEnd = worldToCanvas(cx * 16, (range.maxCZ + 1) * 16, state);
			ctx.moveTo(Math.floor(screen.x) + 0.5, screen.y);
			ctx.lineTo(Math.floor(screenEnd.x) + 0.5, screenEnd.y);
		}
		for (let cz = range.minCZ; cz <= range.maxCZ + 1; cz++) {
			const screen = worldToCanvas(range.minCX * 16, cz * 16, state);
			const screenEnd = worldToCanvas((range.maxCX + 1) * 16, cz * 16, state);
			ctx.moveTo(screen.x, Math.floor(screen.y) + 0.5);
			ctx.lineTo(screenEnd.x, Math.floor(screenEnd.y) + 0.5);
		}
		ctx.stroke();
	}

	// Region borders (every 32 chunks = 512 blocks)
	ctx.strokeStyle = 'rgba(255, 200, 50, 0.25)';
	ctx.lineWidth = 1;

	const regionMinX = Math.floor(range.minCX / 32) * 32;
	const regionMaxX = Math.ceil(range.maxCX / 32) * 32;
	const regionMinZ = Math.floor(range.minCZ / 32) * 32;
	const regionMaxZ = Math.ceil(range.maxCZ / 32) * 32;

	ctx.beginPath();
	for (let rx = regionMinX; rx <= regionMaxX; rx += 32) {
		const screen = worldToCanvas(rx * 16, range.minCZ * 16, state);
		const screenEnd = worldToCanvas(rx * 16, (range.maxCZ + 1) * 16, state);
		ctx.moveTo(Math.floor(screen.x) + 0.5, screen.y);
		ctx.lineTo(Math.floor(screenEnd.x) + 0.5, screenEnd.y);
	}
	for (let rz = regionMinZ; rz <= regionMaxZ; rz += 32) {
		const screen = worldToCanvas(range.minCX * 16, rz * 16, state);
		const screenEnd = worldToCanvas((range.maxCX + 1) * 16, rz * 16, state);
		ctx.moveTo(screen.x, Math.floor(screen.y) + 0.5);
		ctx.lineTo(screenEnd.x, Math.floor(screenEnd.y) + 0.5);
	}
	ctx.stroke();
}

// ===== Crosshair at origin =====

function renderCrosshair(ctx: CanvasRenderingContext2D, state: MapState) {
	const origin = worldToCanvas(0, 0, state);
	const size = 12;

	ctx.strokeStyle = 'rgba(255, 80, 80, 0.8)';
	ctx.lineWidth = 2;

	ctx.beginPath();
	ctx.moveTo(origin.x - size, origin.y);
	ctx.lineTo(origin.x + size, origin.y);
	ctx.moveTo(origin.x, origin.y - size);
	ctx.lineTo(origin.x, origin.y + size);
	ctx.stroke();

	// Small label
	ctx.fillStyle = 'rgba(255, 80, 80, 0.8)';
	ctx.font = '10px system-ui';
	ctx.textAlign = 'left';
	ctx.fillText('0, 0', origin.x + size + 4, origin.y - 4);
}

// ===== Coordinate overlay =====

function renderCoordinateOverlay(ctx: CanvasRenderingContext2D, state: MapState) {
	if (!state.hoverActive) return;

	const x = state.hoverWorldX;
	const z = state.hoverWorldZ;
	const text = `X: ${x}  Z: ${z}  |  Chunk: ${state.hoverChunkX}, ${state.hoverChunkZ}`;

	ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
	ctx.fillRect(8, state.canvasHeight - 28, ctx.measureText(text).width + 16, 22);
	ctx.fillStyle = '#fff';
	ctx.font = '12px monospace';
	ctx.textAlign = 'left';
	ctx.fillText(text, 16, state.canvasHeight - 12);
}
