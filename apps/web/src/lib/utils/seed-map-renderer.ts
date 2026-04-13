import type { MapState, TileData } from '$lib/stores/seed-map.svelte';
import { getVisibleTileRange, getBiomeColor, getBiomeName, getStructureName, TILE_SIZE } from '$lib/stores/seed-map.svelte';

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

	// Background — changes per dimension
	const bgColors: Record<string, string> = {
		overworld: '#1a1a2e',
		nether: '#1a0a0a',
		end: '#0a0a14',
	};
	ctx.fillStyle = bgColors[state.dimension] ?? '#1a1a2e';
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

	if (state.showStructures) {
		renderStructures(ctx, state);
	}

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

	ctx.fillStyle = 'rgba(80, 255, 80, 0.3)';
	ctx.strokeStyle = 'rgba(80, 255, 80, 0.7)';
	ctx.lineWidth = 1;

	for (let cx = minCX; cx <= maxCX; cx++) {
		for (let cz = minCZ; cz <= maxCZ; cz++) {
			const key = `${cx},${cz}`;
			if (state.slimeCache.has(key)) {
				const screen = worldToCanvas(cx * 16, cz * 16, state);
				ctx.fillRect(screen.x, screen.y, chunkPx, chunkPx);
				if (chunkPx >= 8) {
					ctx.strokeRect(screen.x + 0.5, screen.y + 0.5, chunkPx - 1, chunkPx - 1);
				}
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

// ===== Structures =====

// Sprite coordinates for each structure type (from chunkbase spritesheet)
const STRUCT_SPRITES: Record<number, { sx: number; sy: number; sw: number; sh: number }> = {
	0:  { sx: 190, sy: 77,  sw: 20, sh: 26 }, // village
	1:  { sx: 112, sy: 81,  sw: 26, sh: 26 }, // desert-temple
	2:  { sx: 27,  sy: 81,  sw: 26, sh: 26 }, // jungle-temple
	3:  { sx: 31,  sy: 0,   sw: 26, sh: 26 }, // witch-hut
	4:  { sx: 54,  sy: 81,  sw: 26, sh: 26 }, // igloo
	5:  { sx: 0,   sy: 135, sw: 26, sh: 23 }, // ocean-monument
	6:  { sx: 50,  sy: 159, sw: 24, sh: 24 }, // mansion
	7:  { sx: 54,  sy: 54,  sw: 26, sh: 26 }, // pillager-outpost
	9:  { sx: 85,  sy: 0,   sw: 26, sh: 26 }, // ocean-ruin
	10: { sx: 190, sy: 131, sw: 21, sh: 24 }, // shipwreck
	12: { sx: 27,  sy: 27,  sw: 26, sh: 26 }, // ruined-portal
	13: { sx: 139, sy: 81,  sw: 26, sh: 26 }, // ancient-city
	14: { sx: 0,   sy: 27,  sw: 26, sh: 26 }, // trail-ruin
	15: { sx: 0,   sy: 159, sw: 24, sh: 24 }, // trial-chamber
	18: { sx: 166, sy: 0,   sw: 23, sh: 26 }, // mineshaft
	19: { sx: 75,  sy: 159, sw: 24, sh: 24 }, // dungeon
	20: { sx: 190, sy: 51,  sw: 22, sh: 25 }, // desert-well
	21: { sx: 139, sy: 27,  sw: 26, sh: 25 }, // fossil
	23: { sx: 112, sy: 27,  sw: 26, sh: 26 }, // end-city
	24: { sx: 112, sy: 0,   sw: 26, sh: 26 }, // end-gateway
};

// Lazy-loaded spritesheet image
let spriteImg: HTMLImageElement | null = null;
let spriteLoaded = false;

function getSpriteImg(): HTMLImageElement | null {
	if (!spriteImg) {
		spriteImg = new Image();
		spriteImg.src = '/images/ui/seed-map-pois-sprite.png';
		spriteImg.onload = () => { spriteLoaded = true; };
	}
	return spriteLoaded ? spriteImg : null;
}

function renderStructures(ctx: CanvasRenderingContext2D, state: MapState) {
	if (state.structures.length === 0) return;

	const img = getSpriteImg();
	const iconScale = Math.max(0.7, Math.min(1.5, state.zoom * 0.5));

	for (const s of state.structures) {
		if (!state.enabledStructures.has(s.type)) continue;

		const screen = worldToCanvas(s.x, s.z, state);

		if (screen.x < -30 || screen.x > state.canvasWidth + 30 ||
			screen.y < -30 || screen.y > state.canvasHeight + 30) continue;

		const sprite = STRUCT_SPRITES[s.type];

		if (img && sprite) {
			// Draw sprite icon from spritesheet
			const dw = sprite.sw * iconScale;
			const dh = sprite.sh * iconScale;
			ctx.imageSmoothingEnabled = false;
			ctx.drawImage(
				img,
				sprite.sx, sprite.sy, sprite.sw, sprite.sh,
				screen.x - dw / 2, screen.y - dh, dw, dh,
			);
		} else {
			// Fallback: colored dot
			ctx.fillStyle = '#fff';
			ctx.strokeStyle = '#000';
			ctx.lineWidth = 1.5;
			ctx.beginPath();
			ctx.arc(screen.x, screen.y, 5 * iconScale, 0, Math.PI * 2);
			ctx.fill();
			ctx.stroke();
		}

		// Label when zoomed in
		if (state.zoom >= 2) {
			const labelY = screen.y - (sprite ? sprite.sh * iconScale + 2 : 10);
			ctx.fillStyle = '#fff';
			ctx.strokeStyle = 'rgba(0,0,0,0.7)';
			ctx.lineWidth = 2.5;
			ctx.font = 'bold 10px system-ui';
			ctx.textAlign = 'center';
			ctx.strokeText(s.name, screen.x, labelY);
			ctx.fillText(s.name, screen.x, labelY);
		}
	}
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
