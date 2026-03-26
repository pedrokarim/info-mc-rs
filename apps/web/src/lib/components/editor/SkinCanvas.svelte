<script lang="ts">
  import { onMount } from 'svelte';
  import {
    editorState,
    editorSetPixel, editorGetPixel, editorFloodFill, editorDrawLine, editorDrawRect,
    pushHistory, addRecentColor, regenerateDataUrl,
  } from '$lib/stores/skin-editor.svelte';
  import { BODY_PARTS } from '$lib/utils/skin-uv-regions';
  import type { Color } from '$lib/utils/skin-canvas-tools';

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let animId: number;

  // Drawing state
  let isDrawing = $state(false);
  let startPixel = $state<{ x: number; y: number } | null>(null);
  let lastPixel = $state<{ x: number; y: number } | null>(null);
  let previewEnd = $state<{ x: number; y: number } | null>(null);

  // Panning state
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
    const px = Math.floor((cx - editorState.panOffset.x) / editorState.zoom);
    const py = Math.floor((cy - editorState.panOffset.y) / editorState.zoom);
    if (px < 0 || px >= 64 || py < 0 || py >= 64) return null;
    return { x: px, y: py };
  }

  function onPointerDown(e: PointerEvent) {
    if (e.button === 1 || (e.button === 0 && editorState.activeTool === 'pan')) {
      isPanning = true;
      panStart = { x: e.clientX, y: e.clientY };
      panOffsetStart = { x: editorState.panOffset.x, y: editorState.panOffset.y };
      canvas.setPointerCapture(e.pointerId);
      return;
    }

    const p = canvasToPixel(e.clientX, e.clientY);
    if (!p) return;

    canvas.setPointerCapture(e.pointerId);

    if (editorState.activeTool === 'eyedropper') {
      const c = editorGetPixel(p.x, p.y);
      editorState.primaryColor = { ...c };
      return;
    }

    if (editorState.activeTool === 'fill') {
      editorFloodFill(p.x, p.y, { ...editorState.primaryColor });
      addRecentColor({ ...editorState.primaryColor });
      return;
    }

    isDrawing = true;
    startPixel = p;
    lastPixel = p;
    previewEnd = p;

    if (editorState.activeTool === 'pencil' || editorState.activeTool === 'eraser') {
      pushHistory();
      const color: Color = editorState.activeTool === 'eraser' ? { r: 0, g: 0, b: 0, a: 0 } : { ...editorState.primaryColor };
      editorSetPixel(p.x, p.y, color);
      if (editorState.activeTool === 'pencil') addRecentColor({ ...editorState.primaryColor });
    } else if (editorState.activeTool === 'line' || editorState.activeTool === 'rect') {
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
      editorState.panOffset.x = panOffsetStart.x + dx * scaleX;
      editorState.panOffset.y = panOffsetStart.y + dy * scaleY;
      return;
    }

    if (!isDrawing) return;
    const p = canvasToPixel(e.clientX, e.clientY);
    if (!p) return;

    previewEnd = p;

    if (editorState.activeTool === 'pencil' || editorState.activeTool === 'eraser') {
      const color: Color = editorState.activeTool === 'eraser' ? { r: 0, g: 0, b: 0, a: 0 } : { ...editorState.primaryColor };
      if (lastPixel) {
        const dx = Math.abs(p.x - lastPixel.x);
        const dy = Math.abs(p.y - lastPixel.y);
        if (dx > 1 || dy > 1) {
          const steps = Math.max(dx, dy);
          for (let i = 0; i <= steps; i++) {
            const t = steps === 0 ? 0 : i / steps;
            const ix = Math.round(lastPixel.x + (p.x - lastPixel.x) * t);
            const iy = Math.round(lastPixel.y + (p.y - lastPixel.y) * t);
            editorSetPixel(ix, iy, color);
          }
        } else {
          editorSetPixel(p.x, p.y, color);
        }
      }
      lastPixel = p;
    }
  }

  function onPointerUp(e: PointerEvent) {
    if (isPanning) {
      isPanning = false;
      return;
    }

    if (!isDrawing || !startPixel) {
      isDrawing = false;
      return;
    }

    const p = canvasToPixel(e.clientX, e.clientY) ?? previewEnd ?? startPixel;

    if (editorState.activeTool === 'line') {
      editorDrawLine(startPixel.x, startPixel.y, p.x, p.y, { ...editorState.primaryColor });
      addRecentColor({ ...editorState.primaryColor });
    } else if (editorState.activeTool === 'rect') {
      editorDrawRect(startPixel.x, startPixel.y, p.x, p.y, { ...editorState.primaryColor }, editorState.rectFillMode === 'filled');
      addRecentColor({ ...editorState.primaryColor });
    }

    isDrawing = false;
    startPixel = null;
    lastPixel = null;
    previewEnd = null;
  }

  function onWheel(e: WheelEvent) {
    e.preventDefault();
    const p = canvasToPixel(e.clientX, e.clientY);
    const oldZoom = editorState.zoom;
    const delta = e.deltaY > 0 ? -1 : 1;
    editorState.zoom = Math.max(1, Math.min(32, editorState.zoom + delta));

    if (p) {
      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const cx = (e.clientX - rect.left) * scaleX;
      const cy = (e.clientY - rect.top) * scaleX;
      editorState.panOffset.x = cx - (cx - editorState.panOffset.x) * (editorState.zoom / oldZoom);
      editorState.panOffset.y = cy - (cy - editorState.panOffset.y) * (editorState.zoom / oldZoom);
    }
  }

  // ── Render loop ──────────────────────────────────────────────────
  function render() {
    animId = requestAnimationFrame(render);
    if (!ctx) return;

    const cw = canvas.width;
    const ch = canvas.height;
    const z = editorState.zoom;
    const ox = editorState.panOffset.x;
    const oy = editorState.panOffset.y;
    const pixels = editorState.pixels;

    ctx.clearRect(0, 0, cw, ch);

    // Checkerboard background
    for (let y = 0; y < 64; y++) {
      for (let x = 0; x < 64; x++) {
        const sx = ox + x * z;
        const sy = oy + y * z;
        if (sx + z < 0 || sy + z < 0 || sx > cw || sy > ch) continue;
        ctx.fillStyle = ((x + y) % 2 === 0) ? '#e8e8e8' : '#ffffff';
        ctx.fillRect(sx, sy, z, z);
      }
    }

    // Draw pixels
    for (let y = 0; y < 64; y++) {
      for (let x = 0; x < 64; x++) {
        const i = (y * 64 + x) * 4;
        const a = pixels[i + 3];
        if (a === 0) continue;
        const sx = ox + x * z;
        const sy = oy + y * z;
        if (sx + z < 0 || sy + z < 0 || sx > cw || sy > ch) continue;
        ctx.fillStyle = `rgba(${pixels[i]},${pixels[i+1]},${pixels[i+2]},${a / 255})`;
        ctx.fillRect(sx, sy, z, z);
      }
    }

    // Body part overlay — show BOTH layers, active = vivid, inactive = dimmed
    if (editorState.showBodyPartOverlay) {
      for (const part of BODY_PARTS) {
        // Active layer: solid border + vivid fill
        const active = editorState.activeLayer === 'base' ? part.base : part.overlay;
        ctx.fillStyle = part.color + '30';
        ctx.fillRect(ox + active.x * z, oy + active.y * z, active.w * z, active.h * z);
        ctx.strokeStyle = part.color + 'AA';
        ctx.lineWidth = 1.5;
        ctx.strokeRect(ox + active.x * z, oy + active.y * z, active.w * z, active.h * z);

        if (z >= 4) {
          ctx.fillStyle = part.color;
          ctx.font = `bold ${Math.min(z * 1.5, 11)}px 'Chakra Petch', sans-serif`;
          ctx.fillText(part.label, ox + active.x * z + 2, oy + active.y * z + Math.min(z * 1.8, 12));
        }

        // Inactive layer: dashed border + very faded
        const inactive = editorState.activeLayer === 'base' ? part.overlay : part.base;
        ctx.fillStyle = part.color + '10';
        ctx.fillRect(ox + inactive.x * z, oy + inactive.y * z, inactive.w * z, inactive.h * z);
        ctx.strokeStyle = part.color + '40';
        ctx.lineWidth = 1;
        ctx.setLineDash([3, 3]);
        ctx.strokeRect(ox + inactive.x * z, oy + inactive.y * z, inactive.w * z, inactive.h * z);
        ctx.setLineDash([]);
      }
    }

    // Selected body part highlight
    if (editorState.selectedBodyPart) {
      const part = BODY_PARTS.find((p) => p.id === editorState.selectedBodyPart);
      if (part) {
        const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
        ctx.strokeStyle = part.color;
        ctx.lineWidth = 2;
        ctx.setLineDash([4, 2]);
        ctx.strokeRect(ox + region.x * z, oy + region.y * z, region.w * z, region.h * z);
        ctx.setLineDash([]);
      }
    }

    // Grid
    if (editorState.showGrid && z >= 4) {
      ctx.strokeStyle = 'rgba(0,0,0,0.12)';
      ctx.lineWidth = 0.5;
      ctx.beginPath();
      for (let x = 0; x <= 64; x++) {
        const sx = ox + x * z;
        ctx.moveTo(sx, oy);
        ctx.lineTo(sx, oy + 64 * z);
      }
      for (let y = 0; y <= 64; y++) {
        const sy = oy + y * z;
        ctx.moveTo(ox, sy);
        ctx.lineTo(ox + 64 * z, sy);
      }
      ctx.stroke();
    }

    // Line/rect preview ghost
    if (isDrawing && startPixel && previewEnd && (editorState.activeTool === 'line' || editorState.activeTool === 'rect')) {
      const pc = editorState.primaryColor;
      ctx.strokeStyle = `rgba(${pc.r},${pc.g},${pc.b},0.6)`;
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 2]);

      if (editorState.activeTool === 'line') {
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

  function centerCanvas() {
    if (!canvas) return;
    editorState.panOffset.x = (canvas.width - 64 * editorState.zoom) / 2;
    editorState.panOffset.y = (canvas.height - 64 * editorState.zoom) / 2;
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

    centerCanvas();
    render();

    return () => {
      cancelAnimationFrame(animId);
      ro.disconnect();
    };
  });

</script>

<div class="skin-canvas-container">
  <canvas
    bind:this={canvas}
    class="skin-canvas"
    class:tool-pencil={editorState.activeTool === 'pencil'}
    class:tool-eraser={editorState.activeTool === 'eraser'}
    class:tool-eyedropper={editorState.activeTool === 'eyedropper'}
    class:tool-fill={editorState.activeTool === 'fill'}
    class:tool-line={editorState.activeTool === 'line'}
    class:tool-rect={editorState.activeTool === 'rect'}
    class:tool-pan={editorState.activeTool === 'pan' || isPanning}
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onwheel={onWheel}
  ></canvas>

  <div class="canvas-info">
    <span class="zoom-badge">{editorState.zoom}x</span>
  </div>
</div>

<style>
  .skin-canvas-container {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 300px;
    background: var(--surface-0, #dbe8f1);
    border-radius: var(--radius-md, 12px);
    overflow: hidden;
  }

  .skin-canvas {
    display: block;
    width: 100%;
    height: 100%;
    image-rendering: pixelated;
  }

  .skin-canvas.tool-pencil { cursor: crosshair; }
  .skin-canvas.tool-eraser { cursor: crosshair; }
  .skin-canvas.tool-eyedropper { cursor: crosshair; }
  .skin-canvas.tool-fill { cursor: crosshair; }
  .skin-canvas.tool-line { cursor: crosshair; }
  .skin-canvas.tool-rect { cursor: crosshair; }
  .skin-canvas.tool-pan { cursor: grab; }
  .skin-canvas.tool-pan:active { cursor: grabbing; }

  .canvas-info {
    position: absolute;
    bottom: 8px;
    right: 8px;
    display: flex;
    gap: 6px;
    pointer-events: none;
  }

  .zoom-badge {
    background: rgba(0, 0, 0, 0.5);
    color: #fff;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.7rem;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 4px;
  }
</style>
