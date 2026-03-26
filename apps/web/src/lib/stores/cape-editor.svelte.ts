/**
 * Centralized cape editor state using Svelte 5 runes.
 */
import { getPixel as _getPixel, setPixel as _setPixel, floodFill, drawLine, drawRect } from '$lib/utils/skin-canvas-tools';
import { CAPE_DIMS, findCapeFaceAt, isInCapeFace, capeMirrorX } from '$lib/utils/cape-uv-regions';
import type { Color, PixelFilter, Dims } from '$lib/utils/skin-canvas-tools';

export type Tool = 'pencil' | 'eraser' | 'eyedropper' | 'fill' | 'line' | 'rect' | 'pan';
export type RectMode = 'filled' | 'outline';

// ── State ──────────────────────────────────────────────────────────
export const capeState = $state({
  pixels: new Uint8ClampedArray(CAPE_DIMS.w * CAPE_DIMS.h * 4),
  textureVersion: 0,
  width: CAPE_DIMS.w,
  height: CAPE_DIMS.h,

  mode: 'optifine' as 'optifine' | 'creative',

  activeTool: 'pencil' as Tool,
  mirrorMode: false,
  rectFillMode: 'filled' as RectMode,

  primaryColor: { r: 255, g: 255, b: 255, a: 255 } as Color,
  recentColors: [] as Color[],

  zoom: 8,
  panOffset: { x: 0, y: 0 },
  showGrid: true,
  showFaceOverlay: true,

  showBody: true,
  backEquipment: 'cape' as 'cape' | 'elytra',
  animationPaused: true,
  wireframeMode: false,
  cameraResetTrigger: 0,

  _history: [] as Uint8ClampedArray[],
  _redoStack: [] as Uint8ClampedArray[],
});

// ── Helpers ────────────────────────────────────────────────────────
let _dirty = false;
let _dataUrl = '';
let _offscreen: HTMLCanvasElement | null = null;

function getDims(): Dims {
  return { w: capeState.width, h: capeState.height };
}

function getOffscreen(): HTMLCanvasElement {
  if (!_offscreen || _offscreen.width !== capeState.width || _offscreen.height !== capeState.height) {
    _offscreen = document.createElement('canvas');
    _offscreen.width = capeState.width;
    _offscreen.height = capeState.height;
  }
  return _offscreen;
}

function markDirty() {
  _dirty = true;
  capeState.textureVersion++;
}

export function regenerateDataUrl(): string {
  const canvas = getOffscreen();
  const ctx = canvas.getContext('2d')!;
  const imgData = new ImageData(new Uint8ClampedArray(capeState.pixels), capeState.width, capeState.height);
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
  const arr = capeState.recentColors;
  const exists = arr.findIndex((rc) => rc.r === c.r && rc.g === c.g && rc.b === c.b && rc.a === c.a);
  if (exists >= 0) arr.splice(exists, 1);
  arr.unshift({ ...c });
  if (arr.length > MAX_RECENT) arr.pop();
}

// ── History ────────────────────────────────────────────────────────
const MAX_HISTORY = 50;

export function getHistoryLength(): number { return capeState._history.length; }
export function getRedoLength(): number { return capeState._redoStack.length; }

export function pushHistory() {
  capeState._history.push(new Uint8ClampedArray(capeState.pixels));
  if (capeState._history.length > MAX_HISTORY) capeState._history.shift();
  capeState._redoStack = [];
}

export function undo() {
  if (capeState._history.length === 0) return;
  capeState._redoStack.push(new Uint8ClampedArray(capeState.pixels));
  const prev = capeState._history.pop()!;
  capeState.pixels.set(prev);
  markDirty();
}

export function redo() {
  if (capeState._redoStack.length === 0) return;
  capeState._history.push(new Uint8ClampedArray(capeState.pixels));
  const next = capeState._redoStack.pop()!;
  capeState.pixels.set(next);
  markDirty();
}

// ── Pixel filter ───────────────────────────────────────────────────
function getFilter(): PixelFilter {
  return (px: number, py: number) => isInCapeFace(px, py);
}

// ── Pixel operations ───────────────────────────────────────────────
export function capeSetPixel(x: number, y: number, color: Color) {
  const filter = getFilter();
  const dims = getDims();
  _setPixel(capeState.pixels, x, y, color, filter, dims);
  if (capeState.mirrorMode) {
    const face = findCapeFaceAt(x, y);
    if (face) {
      const mx = capeMirrorX(x, face);
      _setPixel(capeState.pixels, mx, y, color, filter, dims);
    }
  }
  markDirty();
}

export function capeGetPixel(x: number, y: number): Color {
  return _getPixel(capeState.pixels, x, y, getDims());
}

export function capeFloodFill(x: number, y: number, color: Color) {
  pushHistory();
  const filter = getFilter();
  const dims = getDims();
  floodFill(capeState.pixels, x, y, color, 0, filter, dims);
  if (capeState.mirrorMode) {
    const face = findCapeFaceAt(x, y);
    if (face) {
      const mx = capeMirrorX(x, face);
      floodFill(capeState.pixels, mx, y, color, 0, filter, dims);
    }
  }
  markDirty();
}

export function capeDrawLine(x0: number, y0: number, x1: number, y1: number, color: Color) {
  const filter = getFilter();
  const dims = getDims();
  drawLine(capeState.pixels, x0, y0, x1, y1, color, filter, dims);
  if (capeState.mirrorMode) {
    const face = findCapeFaceAt(x0, y0);
    if (face) {
      const mx0 = capeMirrorX(x0, face);
      const mx1 = capeMirrorX(x1, face);
      drawLine(capeState.pixels, mx0, y0, mx1, y1, color, filter, dims);
    }
  }
  markDirty();
}

export function capeDrawRect(x0: number, y0: number, x1: number, y1: number, color: Color, filled: boolean) {
  const filter = getFilter();
  const dims = getDims();
  drawRect(capeState.pixels, x0, y0, x1, y1, color, filled, filter, dims);
  if (capeState.mirrorMode) {
    const face = findCapeFaceAt(x0, y0);
    if (face) {
      const mx0 = capeMirrorX(x0, face);
      const mx1 = capeMirrorX(x1, face);
      drawRect(capeState.pixels, mx0, y0, mx1, y1, color, filled, filter, dims);
    }
  }
  markDirty();
}

// ── Import / Export ────────────────────────────────────────────────
export function loadFromImage(img: HTMLImageElement) {
  const canvas = document.createElement('canvas');
  canvas.width = capeState.width;
  canvas.height = capeState.height;
  const ctx = canvas.getContext('2d')!;
  ctx.clearRect(0, 0, capeState.width, capeState.height);
  ctx.drawImage(img, 0, 0, capeState.width, capeState.height);
  const imgData = ctx.getImageData(0, 0, capeState.width, capeState.height);
  capeState.pixels.set(imgData.data);
  pushHistory();
  markDirty();
}

export async function loadFromUrl(url: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => { loadFromImage(img); resolve(); };
    img.onerror = () => reject(new Error('Failed to load cape from URL'));
    img.src = url;
  });
}

export function exportAsPng(): void {
  const canvas = getOffscreen();
  const ctx = canvas.getContext('2d')!;
  const imgData = new ImageData(new Uint8ClampedArray(capeState.pixels), capeState.width, capeState.height);
  ctx.putImageData(imgData, 0, 0);
  const dataUrl = canvas.toDataURL('image/png');
  const a = document.createElement('a');
  a.href = dataUrl;
  a.download = 'cape.png';
  a.click();
}

export function clearCanvas() {
  pushHistory();
  capeState.pixels.fill(0);
  markDirty();
}

// ── Presets (real PNG textures) ──────────────────────────────────────
export interface Preset {
  id: string;
  label: string;
  file: string; // path relative to /images/skins/capes/
}

export const PRESETS: Preset[] = [
  { id: 'mojang',              label: 'Mojang',              file: 'mojang.png' },
  { id: 'mojang-studios',      label: 'Mojang Studios',      file: 'mojang-studios.png' },
  { id: 'minecon-2011',        label: 'Minecon 2011',        file: 'minecon-2011.png' },
  { id: 'minecon-2012',        label: 'Minecon 2012',        file: 'minecon-2012.png' },
  { id: 'minecon-2013',        label: 'Minecon 2013',        file: 'minecon-2013.png' },
  { id: 'minecon-2015',        label: 'Minecon 2015',        file: 'minecon-2015.png' },
  { id: 'minecon-2016',        label: 'Minecon 2016',        file: 'minecon-2016.png' },
  { id: 'translator',          label: 'Translator',           file: 'translator.png' },
  { id: 'translator-japanese', label: 'Translator JP',        file: 'translator_japanese.png' },
  { id: 'migrator',            label: 'Migrator',             file: 'migrator.png' },
  { id: 'cherry-blossom',      label: 'Cherry Blossom',       file: 'cherry-blossom.png' },
  { id: 'vanilla',             label: 'Vanilla',              file: 'vanilla.png' },
  { id: '15th-anniversary',    label: '15th Anniversary',     file: '15th-Anniversary.png' },
  { id: 'birthday',            label: 'Birthday',             file: 'birthday.png' },
  { id: 'cherry-blossom',      label: 'Cherry Blossom',       file: 'cherry-blossom.png' },
  { id: 'copper',              label: 'Copper',               file: 'copper.png' },
  { id: 'follower',            label: 'Follower',             file: 'follower_s.png' },
  { id: 'founder',             label: 'Founder',              file: 'founder_s.png' },
  { id: 'home',                label: 'Home',                 file: 'home.png' },
  { id: 'mcc-15th',            label: 'MCC 15th Year',        file: 'mcc-15th-year.png' },
  { id: 'menace',              label: 'Menace',               file: 'menace.png' },
  { id: 'mc-experience',       label: 'MC Experience',        file: 'minecraft-experience.png' },
  { id: 'mojang-office',       label: 'Mojang Office',        file: 'mojang-office.png' },
  { id: 'millionth-customer',  label: 'Millionth Customer',   file: 'millionth-customer.png' },
  { id: 'oxeye',               label: 'Oxeye',               file: 'oxeye.png' },
  { id: 'pan',                 label: 'Pan',                  file: 'pan.png' },
  { id: 'prismarine',          label: 'Prismarine',           file: 'prismarine.png' },
  { id: 'purple-heart',        label: 'Purple Heart',         file: 'purple-heart.png' },
  { id: 'realms-mapmaker',     label: 'Realms Mapmaker',      file: 'realms-mapmaker.png' },
  { id: 'snowman',             label: 'Snowman',              file: 'snowman.png' },
  { id: 'spade',               label: 'Spade',               file: 'spade.png' },
  { id: 'turtle',              label: 'Turtle',               file: 'turtle.png' },
  { id: 'valentine',           label: 'Valentine',            file: 'valentine.png' },
  { id: 'yearn',               label: 'Yearn',                file: 'yearn.png' },
  { id: 'zombie-horse',        label: 'Zombie Horse',         file: 'zombie-horse.png' },
  { id: 'blank',               label: 'Vierge',               file: '' },
];

export async function loadPreset(presetId: string) {
  const preset = PRESETS.find((p) => p.id === presetId);
  if (!preset) return;

  if (presetId === 'blank') {
    pushHistory();
    capeState.pixels.fill(0);
    markDirty();
    return;
  }

  await loadFromUrl(`/images/skins/capes/${preset.file}`);
}
