<script lang="ts">
  import { onMount } from 'svelte';
  import {
    capeState,
    capeSetPixel, capeGetPixel, capeFloodFill, capeDrawLine, capeDrawRect,
    pushHistory, addRecentColor,
  } from '$lib/stores/cape-editor.svelte';
  import { CAPE_FACES } from '$lib/utils/cape-uv-regions';
  import type { Color } from '$lib/utils/skin-canvas-tools';

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let animId: number;

  let isDrawing = $state(false);
  let startPixel = $state<{ x: number; y: number } | null>(null);
  let lastPixel = $state<{ x: number; y: number } | null>(null);
  let previewEnd = $state<{ x: number; y: number } | null>(null);
  let isPanning = $state(false);
  let panStart = $state({ x: 0, y: 0 });
  let panOffsetStart = $state({ x: 0, y: 0 });

  function canvasToPixel(clientX: number, clientY: number): { x: number; y: number } | null {
    if (!canvas) return null;
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const cx = (clientX - rect.left) * scaleX;
    const cy = (clientY - rect.top) * scaleY;
    const px = Math.floor((cx - capeState.panOffset.x) / capeState.zoom);
    const py = Math.floor((cy - capeState.panOffset.y) / capeState.zoom);
    if (px < 0 || px >= capeState.width || py < 0 || py >= capeState.height) return null;
    return { x: px, y: py };
  }

  function onPointerDown(e: PointerEvent) {
    if (e.button === 1 || (e.button === 0 && capeState.activeTool === 'pan')) {
      isPanning = true;
      panStart = { x: e.clientX, y: e.clientY };
      panOffsetStart = { x: capeState.panOffset.x, y: capeState.panOffset.y };
      canvas.setPointerCapture(e.pointerId);
      return;
    }
    const p = canvasToPixel(e.clientX, e.clientY);
    if (!p) return;
    canvas.setPointerCapture(e.pointerId);

    if (capeState.activeTool === 'eyedropper') {
      const c = capeGetPixel(p.x, p.y);
      capeState.primaryColor = { ...c };
      return;
    }
    if (capeState.activeTool === 'fill') {
      capeFloodFill(p.x, p.y, { ...capeState.primaryColor });
      addRecentColor({ ...capeState.primaryColor });
      return;
    }

    isDrawing = true;
    startPixel = p;
    lastPixel = p;
    previewEnd = p;

    if (capeState.activeTool === 'pencil' || capeState.activeTool === 'eraser') {
      pushHistory();
      const color: Color = capeState.activeTool === 'eraser' ? { r: 0, g: 0, b: 0, a: 0 } : { ...capeState.primaryColor };
      capeSetPixel(p.x, p.y, color);
      if (capeState.activeTool === 'pencil') addRecentColor({ ...capeState.primaryColor });
    } else if (capeState.activeTool === 'line' || capeState.activeTool === 'rect') {
      pushHistory();
    }
  }

  function onPointerMove(e: PointerEvent) {
    if (isPanning) {
      const dx = e.clientX - panStart.x;
      const dy = e.clientY - panStart.y;
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const scaleY = canvas.height / rect.height;
      capeState.panOffset.x = panOffsetStart.x + dx * scaleX;
      capeState.panOffset.y = panOffsetStart.y + dy * scaleY;
      return;
    }
    if (!isDrawing) return;
    const p = canvasToPixel(e.clientX, e.clientY);
    if (!p) return;
    previewEnd = p;

    if (capeState.activeTool === 'pencil' || capeState.activeTool === 'eraser') {
      const color: Color = capeState.activeTool === 'eraser' ? { r: 0, g: 0, b: 0, a: 0 } : { ...capeState.primaryColor };
      if (lastPixel) {
        const dx = Math.abs(p.x - lastPixel.x);
        const dy = Math.abs(p.y - lastPixel.y);
        if (dx > 1 || dy > 1) {
          const steps = Math.max(dx, dy);
          for (let i = 0; i <= steps; i++) {
            const t = steps === 0 ? 0 : i / steps;
            capeSetPixel(Math.round(lastPixel.x + (p.x - lastPixel.x) * t), Math.round(lastPixel.y + (p.y - lastPixel.y) * t), color);
          }
        } else {
          capeSetPixel(p.x, p.y, color);
        }
      }
      lastPixel = p;
    }
  }

  function onPointerUp(e: PointerEvent) {
    if (isPanning) { isPanning = false; return; }
    if (!isDrawing || !startPixel) { isDrawing = false; return; }
    const p = canvasToPixel(e.clientX, e.clientY) ?? previewEnd ?? startPixel;

    if (capeState.activeTool === 'line') {
      capeDrawLine(startPixel.x, startPixel.y, p.x, p.y, { ...capeState.primaryColor });
      addRecentColor({ ...capeState.primaryColor });
    } else if (capeState.activeTool === 'rect') {
      capeDrawRect(startPixel.x, startPixel.y, p.x, p.y, { ...capeState.primaryColor }, capeState.rectFillMode === 'filled');
      addRecentColor({ ...capeState.primaryColor });
    }
    isDrawing = false;
    startPixel = null;
    lastPixel = null;
    previewEnd = null;
  }

  function onWheel(e: WheelEvent) {
    e.preventDefault();
    const oldZoom = capeState.zoom;
    const delta = e.deltaY > 0 ? -1 : 1;
    capeState.zoom = Math.max(1, Math.min(48, capeState.zoom + delta));
    const p = canvasToPixel(e.clientX, e.clientY);
    if (p) {
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const cx = (e.clientX - rect.left) * scaleX;
      const cy = (e.clientY - rect.top) * scaleX;
      capeState.panOffset.x = cx - (cx - capeState.panOffset.x) * (capeState.zoom / oldZoom);
      capeState.panOffset.y = cy - (cy - capeState.panOffset.y) * (capeState.zoom / oldZoom);
    }
  }

  function render() {
    animId = requestAnimationFrame(render);
    if (!ctx) return;

    const cw = canvas.width;
    const ch = canvas.height;
    const z = capeState.zoom;
    const ox = capeState.panOffset.x;
    const oy = capeState.panOffset.y;
    const pixels = capeState.pixels;
    const tw = capeState.width;
    const th = capeState.height;

    ctx.clearRect(0, 0, cw, ch);

    // Checkerboard
    for (let y = 0; y < th; y++) {
      for (let x = 0; x < tw; x++) {
        const sx = ox + x * z;
        const sy = oy + y * z;
        if (sx + z < 0 || sy + z < 0 || sx > cw || sy > ch) continue;
        ctx.fillStyle = ((x + y) % 2 === 0) ? '#e8e8e8' : '#ffffff';
        ctx.fillRect(sx, sy, z, z);
      }
    }

    // Pixels
    for (let y = 0; y < th; y++) {
      for (let x = 0; x < tw; x++) {
        const i = (y * tw + x) * 4;
        const a = pixels[i + 3];
        if (a === 0) continue;
        const sx = ox + x * z;
        const sy = oy + y * z;
        if (sx + z < 0 || sy + z < 0 || sx > cw || sy > ch) continue;
        ctx.fillStyle = `rgba(${pixels[i]},${pixels[i+1]},${pixels[i+2]},${a / 255})`;
        ctx.fillRect(sx, sy, z, z);
      }
    }

    // Face overlay
    if (capeState.showFaceOverlay) {
      for (const face of CAPE_FACES) {
        const r = face.rect;
        ctx.fillStyle = face.color + '30';
        ctx.fillRect(ox + r.x * z, oy + r.y * z, r.w * z, r.h * z);
        ctx.strokeStyle = face.color + 'AA';
        ctx.lineWidth = 1.5;
        ctx.strokeRect(ox + r.x * z, oy + r.y * z, r.w * z, r.h * z);
        if (z >= 8) {
          ctx.fillStyle = face.color;
          ctx.font = `bold ${Math.min(z * 1.2, 11)}px 'Chakra Petch', sans-serif`;
          ctx.fillText(face.label, ox + r.x * z + 2, oy + r.y * z + Math.min(z * 1.5, 12));
        }
      }
    }

    // Grid
    if (capeState.showGrid && z >= 4) {
      ctx.strokeStyle = 'rgba(0,0,0,0.12)';
      ctx.lineWidth = 0.5;
      ctx.beginPath();
      for (let x = 0; x <= tw; x++) { const sx = ox + x * z; ctx.moveTo(sx, oy); ctx.lineTo(sx, oy + th * z); }
      for (let y = 0; y <= th; y++) { const sy = oy + y * z; ctx.moveTo(ox, sy); ctx.lineTo(ox + tw * z, sy); }
      ctx.stroke();
    }

    // Ghost preview
    if (isDrawing && startPixel && previewEnd && (capeState.activeTool === 'line' || capeState.activeTool === 'rect')) {
      const pc = capeState.primaryColor;
      ctx.strokeStyle = `rgba(${pc.r},${pc.g},${pc.b},0.6)`;
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 2]);
      if (capeState.activeTool === 'line') {
        ctx.beginPath();
        ctx.moveTo(ox + (startPixel.x + 0.5) * z, oy + (startPixel.y + 0.5) * z);
        ctx.lineTo(ox + (previewEnd.x + 0.5) * z, oy + (previewEnd.y + 0.5) * z);
        ctx.stroke();
      } else {
        const rx = Math.min(startPixel.x, previewEnd.x);
        const ry = Math.min(startPixel.y, previewEnd.y);
        const rw = Math.abs(previewEnd.x - startPixel.x) + 1;
        const rh = Math.abs(previewEnd.y - startPixel.y) + 1;
        ctx.strokeRect(ox + rx * z, oy + ry * z, rw * z, rh * z);
      }
      ctx.setLineDash([]);
    }
  }

  onMount(() => {
    ctx = canvas.getContext('2d')!;
    ctx.imageSmoothingEnabled = false;
    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        const dpr = window.devicePixelRatio || 1;
        canvas.width = width * dpr;
        canvas.height = height * dpr;
      }
    });
    ro.observe(canvas.parentElement!);

    capeState.panOffset.x = 40;
    capeState.panOffset.y = 40;
    render();

    return () => { cancelAnimationFrame(animId); ro.disconnect(); };
  });
</script>

<div class="cape-canvas-container">
  <canvas
    bind:this={canvas}
    class="cape-canvas"
    class:tool-paint={capeState.activeTool !== 'pan'}
    class:tool-pan={capeState.activeTool === 'pan' || isPanning}
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onwheel={onWheel}
  ></canvas>
  <div class="canvas-info"><span class="zoom-badge">{capeState.zoom}x</span></div>
</div>

<style>
  .cape-canvas-container { position: relative; width: 100%; height: 100%; min-height: 200px; background: var(--surface-0, #dbe8f1); border-radius: var(--radius-md, 12px); overflow: hidden; }
  .cape-canvas { display: block; width: 100%; height: 100%; image-rendering: pixelated; }
  .cape-canvas.tool-paint { cursor: crosshair; }
  .cape-canvas.tool-pan { cursor: grab; }
  .cape-canvas.tool-pan:active { cursor: grabbing; }
  .canvas-info { position: absolute; bottom: 8px; right: 8px; pointer-events: none; }
  .zoom-badge { background: rgba(0,0,0,0.5); color: #fff; font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; font-weight: 600; padding: 2px 8px; border-radius: 4px; }
</style>
