/**
 * Pure drawing functions operating on a Uint8ClampedArray (W×H RGBA).
 * All functions accept optional dims to support any texture size (default 64×64).
 * All functions accept an optional filter callback to constrain edits to specific regions.
 */

export interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface Dims {
  w: number;
  h: number;
}

export type PixelFilter = (x: number, y: number) => boolean;

const DEFAULT_DIMS: Dims = { w: 64, h: 64 };

function idx(x: number, y: number, w: number): number {
  return (y * w + x) * 4;
}

function inBounds(x: number, y: number, d: Dims): boolean {
  return x >= 0 && x < d.w && y >= 0 && y < d.h;
}

export function getPixel(pixels: Uint8ClampedArray, x: number, y: number, dims: Dims = DEFAULT_DIMS): Color {
  if (!inBounds(x, y, dims)) return { r: 0, g: 0, b: 0, a: 0 };
  const i = idx(x, y, dims.w);
  return { r: pixels[i], g: pixels[i + 1], b: pixels[i + 2], a: pixels[i + 3] };
}

export function setPixel(pixels: Uint8ClampedArray, x: number, y: number, c: Color, filter?: PixelFilter, dims: Dims = DEFAULT_DIMS): void {
  if (!inBounds(x, y, dims)) return;
  if (filter && !filter(x, y)) return;
  const i = idx(x, y, dims.w);
  pixels[i] = c.r;
  pixels[i + 1] = c.g;
  pixels[i + 2] = c.b;
  pixels[i + 3] = c.a;
}

export function colorMatch(a: Color, b: Color, tolerance = 0): boolean {
  return (
    Math.abs(a.r - b.r) <= tolerance &&
    Math.abs(a.g - b.g) <= tolerance &&
    Math.abs(a.b - b.b) <= tolerance &&
    Math.abs(a.a - b.a) <= tolerance
  );
}

/**
 * Flood fill using BFS (stack-based to avoid call stack overflow).
 */
export function floodFill(
  pixels: Uint8ClampedArray,
  x: number,
  y: number,
  fillColor: Color,
  tolerance = 0,
  filter?: PixelFilter,
  dims: Dims = DEFAULT_DIMS
): void {
  if (!inBounds(x, y, dims)) return;
  if (filter && !filter(x, y)) return;
  const target = getPixel(pixels, x, y, dims);
  if (colorMatch(target, fillColor, 0)) return;

  const stack: [number, number][] = [[x, y]];
  const visited = new Set<number>();

  while (stack.length > 0) {
    const [cx, cy] = stack.pop()!;
    const key = cy * dims.w + cx;
    if (visited.has(key)) continue;
    if (!inBounds(cx, cy, dims)) continue;
    if (filter && !filter(cx, cy)) continue;

    const current = getPixel(pixels, cx, cy, dims);
    if (!colorMatch(current, target, tolerance)) continue;

    visited.add(key);
    setPixel(pixels, cx, cy, fillColor, undefined, dims);

    stack.push([cx + 1, cy], [cx - 1, cy], [cx, cy + 1], [cx, cy - 1]);
  }
}

/**
 * Bresenham line drawing algorithm.
 */
export function drawLine(
  pixels: Uint8ClampedArray,
  x0: number,
  y0: number,
  x1: number,
  y1: number,
  color: Color,
  filter?: PixelFilter,
  dims: Dims = DEFAULT_DIMS
): void {
  let dx = Math.abs(x1 - x0);
  let dy = -Math.abs(y1 - y0);
  const sx = x0 < x1 ? 1 : -1;
  const sy = y0 < y1 ? 1 : -1;
  let err = dx + dy;

  let cx = x0;
  let cy = y0;

  while (true) {
    setPixel(pixels, cx, cy, color, filter, dims);
    if (cx === x1 && cy === y1) break;
    const e2 = 2 * err;
    if (e2 >= dy) {
      err += dy;
      cx += sx;
    }
    if (e2 <= dx) {
      err += dx;
      cy += sy;
    }
  }
}

/**
 * Draw a rectangle (filled or outline only).
 */
export function drawRect(
  pixels: Uint8ClampedArray,
  x0: number,
  y0: number,
  x1: number,
  y1: number,
  color: Color,
  filled: boolean,
  filter?: PixelFilter,
  dims: Dims = DEFAULT_DIMS
): void {
  const minX = Math.max(0, Math.min(x0, x1));
  const maxX = Math.min(dims.w - 1, Math.max(x0, x1));
  const minY = Math.max(0, Math.min(y0, y1));
  const maxY = Math.min(dims.h - 1, Math.max(y0, y1));

  if (filled) {
    for (let y = minY; y <= maxY; y++) {
      for (let x = minX; x <= maxX; x++) {
        setPixel(pixels, x, y, color, filter, dims);
      }
    }
  } else {
    for (let x = minX; x <= maxX; x++) {
      setPixel(pixels, x, minY, color, filter, dims);
      setPixel(pixels, x, maxY, color, filter, dims);
    }
    for (let y = minY; y <= maxY; y++) {
      setPixel(pixels, minX, y, color, filter, dims);
      setPixel(pixels, maxX, y, color, filter, dims);
    }
  }
}

/**
 * Compute mirrored X coordinate within a body part region.
 */
export function mirrorX(x: number, regionX: number, regionW: number): number {
  const local = x - regionX;
  const mirrored = regionW - 1 - local;
  return regionX + mirrored;
}
