<script lang="ts">
  import { onMount } from 'svelte';

  let {
    color = $bindable({ r: 255, g: 0, b: 0, a: 255 }),
    showAlpha = true,
    onchange,
  }: {
    color?: { r: number; g: number; b: number; a: number };
    showAlpha?: boolean;
    onchange?: (color: { r: number; g: number; b: number; a: number }) => void;
  } = $props();

  // Internal HSL state
  let hue = $state(0);
  let sat = $state(100);
  let lit = $state(50);
  let alpha = $state(255);

  // Canvas refs
  let slCanvas: HTMLCanvasElement;
  let hueCanvas: HTMLCanvasElement;
  let alphaCanvas: HTMLCanvasElement;

  // Dragging states
  let draggingSL = $state(false);
  let draggingHue = $state(false);
  let draggingAlpha = $state(false);

  // Hex input
  let hexInput = $state('');

  // ── Color conversion ───────────────────────────────────────────────
  function hslToRgb(h: number, s: number, l: number): [number, number, number] {
    s /= 100;
    l /= 100;
    const c = (1 - Math.abs(2 * l - 1)) * s;
    const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
    const m = l - c / 2;
    let r = 0, g = 0, b = 0;
    if (h < 60) { r = c; g = x; }
    else if (h < 120) { r = x; g = c; }
    else if (h < 180) { g = c; b = x; }
    else if (h < 240) { g = x; b = c; }
    else if (h < 300) { r = x; b = c; }
    else { r = c; b = x; }
    return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)];
  }

  function rgbToHsl(r: number, g: number, b: number): [number, number, number] {
    r /= 255; g /= 255; b /= 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    const l = (max + min) / 2;
    if (max === min) return [0, 0, Math.round(l * 100)];
    const d = max - min;
    const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    let h = 0;
    if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
    else if (max === g) h = ((b - r) / d + 2) / 6;
    else h = ((r - g) / d + 4) / 6;
    return [Math.round(h * 360), Math.round(s * 100), Math.round(l * 100)];
  }

  function rgbToHex(r: number, g: number, b: number): string {
    return '#' + [r, g, b].map((v) => v.toString(16).padStart(2, '0')).join('');
  }

  function hexToRgb(hex: string): [number, number, number] | null {
    const m = hex.match(/^#?([0-9a-f]{6})$/i);
    if (!m) return null;
    return [parseInt(m[1].slice(0, 2), 16), parseInt(m[1].slice(2, 4), 16), parseInt(m[1].slice(4, 6), 16)];
  }

  // ── Sync external color → internal HSL ─────────────────────────────
  function syncFromRgb() {
    const [h, s, l] = rgbToHsl(color.r, color.g, color.b);
    hue = h;
    sat = s;
    lit = l;
    alpha = color.a;
    hexInput = rgbToHex(color.r, color.g, color.b);
  }

  // ── Emit color change ──────────────────────────────────────────────
  function emitColor() {
    const [r, g, b] = hslToRgb(hue, sat, lit);
    color = { r, g, b, a: alpha };
    hexInput = rgbToHex(r, g, b);
    onchange?.(color);
  }

  // ── Draw SL gradient canvas ────────────────────────────────────────
  function drawSL() {
    if (!slCanvas) return;
    const ctx = slCanvas.getContext('2d')!;
    const w = slCanvas.width;
    const h = slCanvas.height;

    for (let x = 0; x < w; x++) {
      for (let y = 0; y < h; y++) {
        const s = (x / (w - 1)) * 100;
        const l = 100 - (y / (h - 1)) * 100;
        const [r, g, b] = hslToRgb(hue, s, l);
        ctx.fillStyle = `rgb(${r},${g},${b})`;
        ctx.fillRect(x, y, 1, 1);
      }
    }
  }

  // ── Draw hue bar ───────────────────────────────────────────────────
  function drawHue() {
    if (!hueCanvas) return;
    const ctx = hueCanvas.getContext('2d')!;
    const w = hueCanvas.width;
    const h = hueCanvas.height;
    const grad = ctx.createLinearGradient(0, 0, w, 0);
    for (let i = 0; i <= 360; i += 60) {
      const [r, g, b] = hslToRgb(i, 100, 50);
      grad.addColorStop(i / 360, `rgb(${r},${g},${b})`);
    }
    ctx.fillStyle = grad;
    ctx.fillRect(0, 0, w, h);
  }

  // ── Draw alpha bar ─────────────────────────────────────────────────
  function drawAlpha() {
    if (!alphaCanvas) return;
    const ctx = alphaCanvas.getContext('2d')!;
    const w = alphaCanvas.width;
    const h = alphaCanvas.height;
    // Checkerboard
    const sz = 4;
    for (let y = 0; y < h; y += sz) {
      for (let x = 0; x < w; x += sz) {
        ctx.fillStyle = ((x / sz + y / sz) % 2 === 0) ? '#ccc' : '#fff';
        ctx.fillRect(x, y, sz, sz);
      }
    }
    const [r, g, b] = hslToRgb(hue, sat, lit);
    const grad = ctx.createLinearGradient(0, 0, w, 0);
    grad.addColorStop(0, `rgba(${r},${g},${b},0)`);
    grad.addColorStop(1, `rgba(${r},${g},${b},1)`);
    ctx.fillStyle = grad;
    ctx.fillRect(0, 0, w, h);
  }

  // ── Canvas interaction helpers ─────────────────────────────────────
  function handleSL(e: MouseEvent | PointerEvent) {
    const rect = slCanvas.getBoundingClientRect();
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    const y = Math.max(0, Math.min(1, (e.clientY - rect.top) / rect.height));
    sat = Math.round(x * 100);
    lit = Math.round((1 - y) * 100);
    emitColor();
  }

  function handleHue(e: MouseEvent | PointerEvent) {
    const rect = hueCanvas.getBoundingClientRect();
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    hue = Math.round(x * 360);
    emitColor();
    drawSL();
    if (showAlpha) drawAlpha();
  }

  function handleAlpha(e: MouseEvent | PointerEvent) {
    const rect = alphaCanvas.getBoundingClientRect();
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    alpha = Math.round(x * 255);
    emitColor();
  }

  function onHexCommit() {
    const rgb = hexToRgb(hexInput);
    if (rgb) {
      color = { r: rgb[0], g: rgb[1], b: rgb[2], a: alpha };
      syncFromRgb();
      emitColor();
      drawSL();
      if (showAlpha) drawAlpha();
    }
  }

  function handlePointerUp() {
    draggingSL = false;
    draggingHue = false;
    draggingAlpha = false;
  }

  function handlePointerMove(e: PointerEvent) {
    if (draggingSL) handleSL(e);
    else if (draggingHue) handleHue(e);
    else if (draggingAlpha) handleAlpha(e);
  }

  // ── SL cursor position ─────────────────────────────────────────────
  const slCursorX = $derived(`${sat}%`);
  const slCursorY = $derived(`${100 - lit}%`);
  const hueCursorX = $derived(`${(hue / 360) * 100}%`);
  const alphaCursorX = $derived(`${(alpha / 255) * 100}%`);

  const previewColor = $derived(
    `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a / 255})`
  );

  onMount(() => {
    syncFromRgb();
    drawSL();
    drawHue();
    if (showAlpha) drawAlpha();

    window.addEventListener('pointermove', handlePointerMove);
    window.addEventListener('pointerup', handlePointerUp);
    return () => {
      window.removeEventListener('pointermove', handlePointerMove);
      window.removeEventListener('pointerup', handlePointerUp);
    };
  });
</script>

<div class="color-picker">
  <!-- SL gradient area -->
  <div class="sl-area">
    <canvas
      bind:this={slCanvas}
      width={160}
      height={160}
      class="sl-canvas"
      onpointerdown={(e) => { draggingSL = true; handleSL(e); }}
    ></canvas>
    <div class="sl-cursor" style="left: {slCursorX}; top: {slCursorY}"></div>
  </div>

  <!-- Hue bar -->
  <div class="bar-wrap">
    <canvas
      bind:this={hueCanvas}
      width={160}
      height={14}
      class="bar-canvas"
      onpointerdown={(e) => { draggingHue = true; handleHue(e); }}
    ></canvas>
    <div class="bar-cursor" style="left: {hueCursorX}"></div>
  </div>

  <!-- Alpha bar -->
  {#if showAlpha}
    <div class="bar-wrap">
      <canvas
        bind:this={alphaCanvas}
        width={160}
        height={14}
        class="bar-canvas"
        onpointerdown={(e) => { draggingAlpha = true; handleAlpha(e); }}
      ></canvas>
      <div class="bar-cursor" style="left: {alphaCursorX}"></div>
    </div>
  {/if}

  <!-- Hex + preview -->
  <div class="inputs-row">
    <div class="preview-swatch" style="background: {previewColor}"></div>
    <input
      type="text"
      class="hex-input"
      bind:value={hexInput}
      onchange={onHexCommit}
      maxlength={7}
      placeholder="#FF0000"
    />
  </div>

  <!-- RGB inputs -->
  <div class="rgb-row">
    <label class="rgb-field">
      <span>R</span>
      <input type="number" min={0} max={255} bind:value={color.r}
        oninput={() => { syncFromRgb(); drawSL(); if (showAlpha) drawAlpha(); onchange?.(color); }} />
    </label>
    <label class="rgb-field">
      <span>G</span>
      <input type="number" min={0} max={255} bind:value={color.g}
        oninput={() => { syncFromRgb(); drawSL(); if (showAlpha) drawAlpha(); onchange?.(color); }} />
    </label>
    <label class="rgb-field">
      <span>B</span>
      <input type="number" min={0} max={255} bind:value={color.b}
        oninput={() => { syncFromRgb(); drawSL(); if (showAlpha) drawAlpha(); onchange?.(color); }} />
    </label>
    {#if showAlpha}
      <label class="rgb-field">
        <span>A</span>
        <input type="number" min={0} max={255} bind:value={alpha}
          oninput={() => { color = { ...color, a: alpha }; onchange?.(color); }} />
      </label>
    {/if}
  </div>
</div>

<style>
  .color-picker {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 200px;
    user-select: none;
  }

  .sl-area {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    border-radius: var(--radius-sm, 8px);
    overflow: hidden;
    cursor: crosshair;
  }

  .sl-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }

  .sl-cursor {
    position: absolute;
    width: 12px;
    height: 12px;
    border: 2px solid #fff;
    border-radius: 50%;
    box-shadow: 0 0 2px rgba(0, 0, 0, 0.6);
    transform: translate(-50%, -50%);
    pointer-events: none;
  }

  .bar-wrap {
    position: relative;
    height: 16px;
    border-radius: 8px;
    overflow: hidden;
    cursor: pointer;
  }

  .bar-canvas {
    display: block;
    width: 100%;
    height: 100%;
    border-radius: 8px;
  }

  .bar-cursor {
    position: absolute;
    top: 50%;
    width: 6px;
    height: 16px;
    background: #fff;
    border: 1px solid rgba(0, 0, 0, 0.3);
    border-radius: 3px;
    transform: translate(-50%, -50%);
    pointer-events: none;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .inputs-row {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .preview-swatch {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm, 8px);
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    flex-shrink: 0;
    background-image:
      linear-gradient(45deg, #ccc 25%, transparent 25%),
      linear-gradient(-45deg, #ccc 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #ccc 75%),
      linear-gradient(-45deg, transparent 75%, #ccc 75%);
    background-size: 8px 8px;
    background-position: 0 0, 0 4px, 4px -4px, -4px 0;
  }

  .hex-input {
    flex: 1;
    min-width: 0;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 6px 8px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa);
    color: var(--ink-0, #0f253a);
    outline: none;
    text-transform: uppercase;
  }

  .hex-input:focus {
    border-color: var(--blue-0, #5e90ff);
  }

  .rgb-row {
    display: flex;
    gap: 4px;
  }

  .rgb-field {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    flex: 1;
  }

  .rgb-field span {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .rgb-field input {
    width: 100%;
    min-width: 0;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.72rem;
    font-weight: 600;
    padding: 4px 2px;
    text-align: center;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 6px;
    background: var(--surface-1, #edf5fa);
    color: var(--ink-0, #0f253a);
    outline: none;
    -moz-appearance: textfield;
  }

  .rgb-field input::-webkit-outer-spin-button,
  .rgb-field input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .rgb-field input:focus {
    border-color: var(--blue-0, #5e90ff);
  }
</style>
