/**
 * Centralized skin editor state using Svelte 5 runes.
 * All mutable state is inside a single $state object to allow cross-module assignment.
 */
import { getPixel as _getPixel, setPixel as _setPixel, floodFill, drawLine, drawRect, mirrorX } from '$lib/utils/skin-canvas-tools';
import { findBodyPartAt, isInAnyRegion } from '$lib/utils/skin-uv-regions';
import type { Color, PixelFilter } from '$lib/utils/skin-canvas-tools';

// ── Tool types ─────────────────────────────────────────────────────
export type Tool = 'pencil' | 'eraser' | 'eyedropper' | 'fill' | 'line' | 'rect' | 'pan';
export type RectMode = 'filled' | 'outline';
export type Layer = 'base' | 'overlay';

// ── Single reactive state object ───────────────────────────────────
export const editorState = $state({
  pixels: new Uint8ClampedArray(64 * 64 * 4),
  textureVersion: 0,

  activeTool: 'pencil' as Tool,
  mirrorMode: false,
  rectFillMode: 'filled' as RectMode,

  primaryColor: { r: 0, g: 0, b: 0, a: 255 } as Color,
  recentColors: [] as Color[],

  zoom: 8,
  panOffset: { x: 0, y: 0 },
  showGrid: true,
  showBodyPartOverlay: false,

  activeLayer: 'base' as Layer,
  selectedBodyPart: null as string | null,

  slim: false,

  // Part visibility (3D viewer)
  partVisibility: {
    head: { base: true, overlay: true },
    body: { base: true, overlay: true },
    rarm: { base: true, overlay: true },
    larm: { base: true, overlay: true },
    rleg: { base: true, overlay: true },
    lleg: { base: true, overlay: true },
  } as Record<string, { base: boolean; overlay: boolean }>,

  // 3D controls
  animationPaused: true,
  wireframeMode: false,
  cameraResetTrigger: 0,

  // History
  _history: [] as Uint8ClampedArray[],
  _redoStack: [] as Uint8ClampedArray[],
});

// ── Private helpers ────────────────────────────────────────────────
let _dirty = false;
let _dataUrl = '';
let _offscreen: HTMLCanvasElement | null = null;

function getOffscreen(): HTMLCanvasElement {
  if (!_offscreen) {
    _offscreen = document.createElement('canvas');
    _offscreen.width = 64;
    _offscreen.height = 64;
  }
  return _offscreen;
}

function markDirty() {
  _dirty = true;
  editorState.textureVersion++;
}

// ── Texture data URL ───────────────────────────────────────────────
export function regenerateDataUrl(): string {
  const canvas = getOffscreen();
  const ctx = canvas.getContext('2d')!;
  const imgData = new ImageData(new Uint8ClampedArray(editorState.pixels), 64, 64);
  ctx.putImageData(imgData, 0, 0);
  _dataUrl = canvas.toDataURL('image/png');
  _dirty = false;
  return _dataUrl;
}

export function getTextureDataUrl(): string {
  if (_dirty || !_dataUrl) return regenerateDataUrl();
  return _dataUrl;
}

// ── Recent colors ──────────────────────────────────────────────────
const MAX_RECENT = 24;

export function addRecentColor(c: Color) {
  const arr = editorState.recentColors;
  const exists = arr.findIndex(
    (rc) => rc.r === c.r && rc.g === c.g && rc.b === c.b && rc.a === c.a
  );
  if (exists >= 0) arr.splice(exists, 1);
  arr.unshift({ ...c });
  if (arr.length > MAX_RECENT) arr.pop();
}

// ── History ────────────────────────────────────────────────────────
const MAX_HISTORY = 50;

export function getHistoryLength(): number { return editorState._history.length; }
export function getRedoLength(): number { return editorState._redoStack.length; }

export function pushHistory() {
  editorState._history.push(new Uint8ClampedArray(editorState.pixels));
  if (editorState._history.length > MAX_HISTORY) editorState._history.shift();
  editorState._redoStack = [];
}

export function undo() {
  if (editorState._history.length === 0) return;
  editorState._redoStack.push(new Uint8ClampedArray(editorState.pixels));
  const prev = editorState._history.pop()!;
  editorState.pixels.set(prev);
  markDirty();
}

export function redo() {
  if (editorState._redoStack.length === 0) return;
  editorState._history.push(new Uint8ClampedArray(editorState.pixels));
  const next = editorState._redoStack.pop()!;
  editorState.pixels.set(next);
  markDirty();
}

// ── Layer filter ───────────────────────────────────────────────────
function getLayerFilter(): PixelFilter {
  const layer = editorState.activeLayer;
  return (px: number, py: number) => isInAnyRegion(px, py, layer);
}

// ── Pixel operations ───────────────────────────────────────────────
export function editorSetPixel(x: number, y: number, color: Color) {
  const filter = getLayerFilter();
  _setPixel(editorState.pixels, x, y, color, filter);
  if (editorState.mirrorMode) {
    const part = findBodyPartAt(x, y, editorState.activeLayer);
    if (part) {
      const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
      const mx = mirrorX(x, region.x, region.w);
      _setPixel(editorState.pixels, mx, y, color, filter);
    }
  }
  markDirty();
}

export function editorGetPixel(x: number, y: number): Color {
  return _getPixel(editorState.pixels, x, y);
}

export function editorFloodFill(x: number, y: number, color: Color) {
  pushHistory();
  const filter = getLayerFilter();
  floodFill(editorState.pixels, x, y, color, 0, filter);
  if (editorState.mirrorMode) {
    const part = findBodyPartAt(x, y, editorState.activeLayer);
    if (part) {
      const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
      const mx = mirrorX(x, region.x, region.w);
      floodFill(editorState.pixels, mx, y, color, 0, filter);
    }
  }
  markDirty();
}

export function editorDrawLine(x0: number, y0: number, x1: number, y1: number, color: Color) {
  const filter = getLayerFilter();
  drawLine(editorState.pixels, x0, y0, x1, y1, color, filter);
  if (editorState.mirrorMode) {
    const part = findBodyPartAt(x0, y0, editorState.activeLayer);
    if (part) {
      const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
      const mx0 = mirrorX(x0, region.x, region.w);
      const mx1 = mirrorX(x1, region.x, region.w);
      drawLine(editorState.pixels, mx0, y0, mx1, y1, color, filter);
    }
  }
  markDirty();
}

export function editorDrawRect(x0: number, y0: number, x1: number, y1: number, color: Color, filled: boolean) {
  const filter = getLayerFilter();
  drawRect(editorState.pixels, x0, y0, x1, y1, color, filled, filter);
  if (editorState.mirrorMode) {
    const part = findBodyPartAt(x0, y0, editorState.activeLayer);
    if (part) {
      const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
      const mx0 = mirrorX(x0, region.x, region.w);
      const mx1 = mirrorX(x1, region.x, region.w);
      drawRect(editorState.pixels, mx0, y0, mx1, y1, color, filled, filter);
    }
  }
  markDirty();
}

// ── Import / Export ────────────────────────────────────────────────
export function loadFromImage(img: HTMLImageElement) {
  const canvas = document.createElement('canvas');
  canvas.width = 64;
  canvas.height = 64;
  const ctx = canvas.getContext('2d')!;
  ctx.clearRect(0, 0, 64, 64);
  ctx.drawImage(img, 0, 0, 64, 64);
  const imgData = ctx.getImageData(0, 0, 64, 64);
  editorState.pixels.set(imgData.data);
  pushHistory();
  markDirty();
}

export async function loadFromUrl(url: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => { loadFromImage(img); resolve(); };
    img.onerror = () => reject(new Error('Failed to load skin from URL'));
    img.src = url;
  });
}

export function exportAsPng(): void {
  const canvas = getOffscreen();
  const ctx = canvas.getContext('2d')!;
  const imgData = new ImageData(new Uint8ClampedArray(editorState.pixels), 64, 64);
  ctx.putImageData(imgData, 0, 0);
  const dataUrl = canvas.toDataURL('image/png');
  const a = document.createElement('a');
  a.href = dataUrl;
  a.download = 'skin.png';
  a.click();
}

export function clearCanvas() {
  pushHistory();
  editorState.pixels.fill(0);
  markDirty();
}

// ── Templates ──────────────────────────────────────────────────────
const TEMPLATE_URLS: Record<string, { url: string; slim: boolean }> = {
  steve:      { url: '/images/skins/steve.png', slim: false },
  alex:       { url: '/images/skins/alex.png', slim: true },
  pedrokarim: { url: '/images/skins/PedroKarim64.png', slim: false },
};

export async function loadTemplate(type: 'steve' | 'alex' | 'pedrokarim' | 'blank') {
  if (type === 'blank') {
    pushHistory();
    editorState.pixels.fill(0);
    editorState.slim = false;
    markDirty();
    return;
  }

  const tpl = TEMPLATE_URLS[type];
  if (!tpl) return;
  editorState.slim = tpl.slim;
  await loadFromUrl(tpl.url);
}
