<script lang="ts">
  import { onMount } from 'svelte';
  import {
    mapState, zoomToward, pan, requestVisibleTiles,
    getBiomeName,
  } from '$lib/stores/seed-map.svelte';
  import { renderFrame, canvasToWorld, getBiomeAtWorld } from '$lib/utils/seed-map-renderer';

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let container: HTMLDivElement;
  let animId = 0;

  // Pan state
  let dragging = false;
  let lastX = 0;
  let lastY = 0;

  // Debounce tile requests during pan
  let requestTimeout: ReturnType<typeof setTimeout> | null = null;

  function scheduleRequest() {
    if (requestTimeout) clearTimeout(requestTimeout);
    requestTimeout = setTimeout(() => {
      requestVisibleTiles();
    }, 50);
  }

  function loop() {
    if (ctx) {
      renderFrame(ctx, mapState);
    }
    animId = requestAnimationFrame(loop);
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    zoomToward(x, y, e.deltaY);
    scheduleRequest();
  }

  function handlePointerDown(e: PointerEvent) {
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
    canvas.setPointerCapture(e.pointerId);
  }

  function handlePointerMove(e: PointerEvent) {
    const rect = canvas.getBoundingClientRect();
    const cx = e.clientX - rect.left;
    const cy = e.clientY - rect.top;

    // Update hover info
    const world = canvasToWorld(cx, cy, mapState);
    mapState.hoverWorldX = Math.floor(world.x);
    mapState.hoverWorldZ = Math.floor(world.z);
    mapState.hoverChunkX = Math.floor(world.x / 16);
    mapState.hoverChunkZ = Math.floor(world.z / 16);
    mapState.hoverActive = true;

    // Look up biome from cached tiles
    mapState.hoverBiome = getBiomeAtWorld(mapState, Math.floor(world.x), Math.floor(world.z));

    if (dragging) {
      const dx = (e.clientX - lastX) / mapState.zoom;
      const dy = (e.clientY - lastY) / mapState.zoom;
      pan(-dx, -dy);
      lastX = e.clientX;
      lastY = e.clientY;
      scheduleRequest();
    }
  }

  function handlePointerUp(e: PointerEvent) {
    dragging = false;
    canvas.releasePointerCapture(e.pointerId);
    requestVisibleTiles();
  }

  function handlePointerLeave() {
    mapState.hoverActive = false;
  }

  onMount(() => {
    ctx = canvas.getContext('2d')!;

    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        canvas.width = width * devicePixelRatio;
        canvas.height = height * devicePixelRatio;
        ctx!.scale(devicePixelRatio, devicePixelRatio);
        mapState.canvasWidth = width;
        mapState.canvasHeight = height;
      }
      requestVisibleTiles();
    });

    ro.observe(container);
    animId = requestAnimationFrame(loop);

    return () => {
      cancelAnimationFrame(animId);
      ro.disconnect();
      if (requestTimeout) clearTimeout(requestTimeout);
    };
  });
</script>

<div class="canvas-container" bind:this={container}>
  <canvas
    bind:this={canvas}
    onwheel={handleWheel}
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={handlePointerUp}
    onpointerleave={handlePointerLeave}
  ></canvas>
</div>

<style>
  .canvas-container {
    width: 100%;
    height: 100%;
    min-height: 400px;
    position: relative;
    overflow: hidden;
    border-radius: var(--radius-lg, 16px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    background: #1a1a2e;
  }

  canvas {
    display: block;
    width: 100%;
    height: 100%;
    cursor: grab;
    touch-action: none;
  }

  canvas:active {
    cursor: grabbing;
  }
</style>
