import type { MapState, TileData } from '$lib/stores/seed-map.svelte';
import { getVisibleTileRange, getBiomeColor, getBiomeName, TILE_SIZE } from '$lib/stores/seed-map.svelte';

// ===== Coordinate transforms =====

export function worldToCanvas(
	worldX: number, worldZ: number, state: MapState
): { x: number; y: number } {
	return {
		x: state.canvasWidth / 2 + (worldX - state.centerX) * state.zoom,
		y: state.canvasHeight / 2 + (worldZ - state.centerZ) * state.zoom,
	};
}

export function canvasToWorld(
	canvasX: number, canvasY: number, state: MapState
): { x: number; z: number } {
	return {
		x: state.centerX + (canvasX - state.canvasWidth / 2) / state.zoom,
		z: state.centerZ + (canvasY - state.canvasHeight / 2) / state.zoom,
	};
}

// ===== Main render =====

// Reusable ImageData cache to avoid allocation per tile per frame
const tileCanvasCache = new Map<string, ImageData>();

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
		renderBiomeTiles(ctx, state);
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
	ctx.fillText('Entrez une seed pour commencer', state.canvasWidth / 2, state.canvasHeight / 2);
}

// ===== Biome tile rendering =====

function renderBiomeTiles(ctx: CanvasRenderingContext2D, state: MapState) {
	const range = getVisibleTileRange();
	const { step, tileWorldSize } = range;
	const tilePx = tileWorldSize * state.zoom; // pixel size of one tile on screen

	for (let tx = range.minTX; tx <= range.maxTX; tx++) {
		for (let tz = range.minTZ; tz <= range.maxTZ; tz++) {
			const key = `${tx},${tz},${step}`;
			const tile = state.tileCache.get(key);
			if (!tile) continue;

			// World position of tile's top-left corner
			const worldX = tx * tileWorldSize;
			const worldZ = tz * tileWorldSize;
			const screen = worldToCanvas(worldX, worldZ, state);

			// Draw the tile's RGBA data scaled to screen size
			drawTile(ctx, tile, screen.x, screen.y, tilePx);
		}
	}
}

function drawTile(
	ctx: CanvasRenderingContext2D,
	tile: TileData,
	screenX: number, screenY: number,
	tilePx: number,
) {
	// Create ImageData from RGBA buffer
	const clamped = new Uint8ClampedArray(tile.rgba.buffer, tile.rgba.byteOffset, tile.rgba.byteLength);
	const imgData = new ImageData(clamped, TILE_SIZE, TILE_SIZE);

	// Use a temporary canvas to draw at native resolution then scale
	let tmpCanvas = (drawTile as any)._tmpCanvas as OffscreenCanvas | undefined;
	if (!tmpCanvas) {
		tmpCanvas = new OffscreenCanvas(TILE_SIZE, TILE_SIZE);
		(drawTile as any)._tmpCanvas = tmpCanvas;
	}
	const tmpCtx = tmpCanvas.getContext('2d')!;
	tmpCtx.putImageData(imgData, 0, 0);

	// Draw scaled to screen
	ctx.imageSmoothingEnabled = false; // pixelated look
	ctx.drawImage(
		tmpCanvas,
		Math.floor(screenX),
		Math.floor(screenY),
		Math.ceil(tilePx) + 1,
		Math.ceil(tilePx) + 1,
	);
}

// ===== Slime chunks =====

function renderSlimeChunks(ctx: CanvasRenderingContext2D, state: MapState) {
	const chunkPx = 16 * state.zoom;
	if (chunkPx < 3) return; // too small to see

	const halfW = state.canvasWidth / 2 / state.zoom;
	const halfH = state.canvasHeight / 2 / state.zoom;
	const minCX = Math.floor((state.centerX - halfW) / 16) - 1;
	const maxCX = Math.floor((state.centerX + halfW) / 16) + 1;
	const minCZ = Math.floor((state.centerZ - halfH) / 16) - 1;
	const maxCZ = Math.floor((state.centerZ + halfH) / 16) + 1;

	ctx.fillStyle = 'rgba(80, 255, 80, 0.25)';
	ctx.strokeStyle = 'rgba(80, 255, 80, 0.6)';
	ctx.lineWidth = 1;

	// Check slimeCache — we need to request slime data from worker if not cached
	// For now use the tile biomeIds (TODO: separate slime worker)
	for (let cx = minCX; cx <= maxCX; cx++) {
		for (let cz = minCZ; cz <= maxCZ; cz++) {
			const key = `${cx},${cz}`;
			const area = state.slimeCache.get(key);
			// Simple: check if we have single-chunk slime data
			if (area && area[0] === 1) {
				const screen = worldToCanvas(cx * 16, cz * 16, state);
				ctx.fillRect(screen.x, screen.y, chunkPx, chunkPx);
				ctx.strokeRect(screen.x + 0.5, screen.y + 0.5, chunkPx - 1, chunkPx - 1);
			}
		}
	}
}

// ===== Grid =====

function renderGrid(ctx: CanvasRenderingContext2D, state: MapState) {
	const halfW = state.canvasWidth / 2 / state.zoom;
	const halfH = state.canvasHeight / 2 / state.zoom;
	const minCX = Math.floor((state.centerX - halfW) / 16);
	const maxCX = Math.floor((state.centerX + halfW) / 16) + 1;
	const minCZ = Math.floor((state.centerZ - halfH) / 16);
	const maxCZ = Math.floor((state.centerZ + halfH) / 16) + 1;

	const chunkPx = 16 * state.zoom;

	if (chunkPx >= 8) {
		ctx.strokeStyle = 'rgba(255, 255, 255, 0.08)';
		ctx.lineWidth = 0.5;
		ctx.beginPath();
		for (let cx = minCX; cx <= maxCX + 1; cx++) {
			const s = worldToCanvas(cx * 16, minCZ * 16, state);
			const e = worldToCanvas(cx * 16, (maxCZ + 1) * 16, state);
			ctx.moveTo(Math.floor(s.x) + 0.5, s.y);
			ctx.lineTo(Math.floor(e.x) + 0.5, e.y);
		}
		for (let cz = minCZ; cz <= maxCZ + 1; cz++) {
			const s = worldToCanvas(minCX * 16, cz * 16, state);
			const e = worldToCanvas((maxCX + 1) * 16, cz * 16, state);
			ctx.moveTo(s.x, Math.floor(s.y) + 0.5);
			ctx.lineTo(e.x, Math.floor(e.y) + 0.5);
		}
		ctx.stroke();
	}

	// Region borders (every 512 blocks)
	ctx.strokeStyle = 'rgba(255, 200, 50, 0.25)';
	ctx.lineWidth = 1;
	const rMinX = Math.floor(minCX / 32) * 32;
	const rMaxX = Math.ceil(maxCX / 32) * 32;
	const rMinZ = Math.floor(minCZ / 32) * 32;
	const rMaxZ = Math.ceil(maxCZ / 32) * 32;
	ctx.beginPath();
	for (let rx = rMinX; rx <= rMaxX; rx += 32) {
		const s = worldToCanvas(rx * 16, minCZ * 16, state);
		const e = worldToCanvas(rx * 16, (maxCZ + 1) * 16, state);
		ctx.moveTo(Math.floor(s.x) + 0.5, s.y);
		ctx.lineTo(Math.floor(e.x) + 0.5, e.y);
	}
	for (let rz = rMinZ; rz <= rMaxZ; rz += 32) {
		const s = worldToCanvas(minCX * 16, rz * 16, state);
		const e = worldToCanvas((maxCX + 1) * 16, rz * 16, state);
		ctx.moveTo(s.x, Math.floor(s.y) + 0.5);
		ctx.lineTo(e.x, Math.floor(e.y) + 0.5);
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
	const biome = state.hoverBiome || '...';
	const text = `X: ${x}  Z: ${z}  |  Chunk: ${state.hoverChunkX}, ${state.hoverChunkZ}  |  ${biome}`;

	ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
	const w = ctx.measureText(text).width + 16;
	ctx.fillRect(8, state.canvasHeight - 28, w, 22);
	ctx.fillStyle = '#fff';
	ctx.font = '12px monospace';
	ctx.textAlign = 'left';
	ctx.fillText(text, 16, state.canvasHeight - 12);
}

// ===== Biome lookup at world position (for tooltip) =====

export function getBiomeAtWorld(state: MapState, worldX: number, worldZ: number): string {
	const range = getVisibleTileRange();
	const { step, tileWorldSize } = range;

	const tx = Math.floor(worldX / tileWorldSize);
	const tz = Math.floor(worldZ / tileWorldSize);
	const key = `${tx},${tz},${step}`;
	const tile = state.tileCache.get(key);
	if (!tile) return '';

	// Position within the tile
	const localX = Math.floor((worldX - tx * tileWorldSize) / step);
	const localZ = Math.floor((worldZ - tz * tileWorldSize) / step);

	if (localX < 0 || localX >= TILE_SIZE || localZ < 0 || localZ >= TILE_SIZE) return '';

	const biomeId = tile.biomeIds[localZ * TILE_SIZE + localX];
	return getBiomeName(biomeId);
}
