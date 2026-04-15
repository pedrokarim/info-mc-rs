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
  let dragMoved = false;
  let dragStartX = 0;
  let dragStartY = 0;

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
    dragMoved = false;
    lastX = e.clientX;
    lastY = e.clientY;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    canvas.setPointerCapture(e.pointerId);
  }

  /** Find the nearest visible marker within click radius (px). */
  function findMarkerAt(canvasX: number, canvasY: number) {
    if (mapState.structures.length === 0) return null;
    const clickRadius = 16;
    const halfW = mapState.canvasWidth / 2;
    const halfH = mapState.canvasHeight / 2;
    let best = null;
    let bestDist = clickRadius * clickRadius;

    for (const s of mapState.structures) {
      if (!mapState.enabledStructures.has(s.type)) continue;
      const sx = halfW + (s.x - mapState.centerX) * mapState.zoom;
      const sy = halfH + (s.z - mapState.centerZ) * mapState.zoom;
      const dx = sx - canvasX;
      const dy = sy - canvasY;
      const d = dx * dx + dy * dy;
      if (d < bestDist) {
        bestDist = d;
        best = s;
      }
    }
    return best;
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

    // Check slime chunk
    const slimeKey = `${mapState.hoverChunkX},${mapState.hoverChunkZ}`;
    mapState.hoverIsSlime = mapState.slimeCache.has(slimeKey);

    // Cursor: pointer if hovering a marker, grab/grabbing otherwise
    if (!dragging) {
      const marker = findMarkerAt(cx, cy);
      canvas.style.cursor = marker ? 'pointer' : 'grab';
    }

    if (dragging) {
      const dx = (e.clientX - lastX) / mapState.zoom;
      const dy = (e.clientY - lastY) / mapState.zoom;
      // Only count as drag if mouse moved > 4px from start
      const totalDx = e.clientX - dragStartX;
      const totalDy = e.clientY - dragStartY;
      if (totalDx * totalDx + totalDy * totalDy > 16) {
        dragMoved = true;
      }
      pan(-dx, -dy);
      lastX = e.clientX;
      lastY = e.clientY;
      scheduleRequest();
    }
  }

  function handlePointerUp(e: PointerEvent) {
    dragging = false;
    canvas.releasePointerCapture(e.pointerId);

    // Click detection: if mouse didn't move, treat as click
    if (!dragMoved) {
      const rect = canvas.getBoundingClientRect();
      const cx = e.clientX - rect.left;
      const cy = e.clientY - rect.top;
      const marker = findMarkerAt(cx, cy);
      if (marker) {
        const biome = getBiomeAtWorld(mapState, marker.x, marker.z);
        mapState.selectedMarker = {
          type: marker.type,
          name: marker.name,
          x: marker.x,
          z: marker.z,
          biome,
        };
      } else {
        mapState.selectedMarker = null;
      }
    }

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
