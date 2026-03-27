<script lang="ts">
  import { onMount } from 'svelte';
  import SeedMapCanvas from './SeedMapCanvas.svelte';
  import SeedMapControls from './SeedMapControls.svelte';
  import SeedMapTooltip from './SeedMapTooltip.svelte';
  import {
    mapState, initWorker, terminateWorker, pan, zoomIn, zoomOut,
    requestVisibleChunks,
  } from '$lib/stores/seed-map.svelte';

  function handleKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement)?.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    const step = 16 / mapState.zoom;

    switch (e.key) {
      case 'ArrowLeft':
        e.preventDefault();
        pan(-step, 0);
        requestVisibleChunks();
        break;
      case 'ArrowRight':
        e.preventDefault();
        pan(step, 0);
        requestVisibleChunks();
        break;
      case 'ArrowUp':
        e.preventDefault();
        pan(0, -step);
        requestVisibleChunks();
        break;
      case 'ArrowDown':
        e.preventDefault();
        pan(0, step);
        requestVisibleChunks();
        break;
      case '+':
      case '=':
        zoomIn();
        requestVisibleChunks();
        break;
      case '-':
        zoomOut();
        requestVisibleChunks();
        break;
      case 'g':
      case 'G':
        mapState.showGrid = !mapState.showGrid;
        break;
      case 's':
      case 'S':
        mapState.showSlimeChunks = !mapState.showSlimeChunks;
        break;
      case 'b':
      case 'B':
        mapState.showBiomes = !mapState.showBiomes;
        break;
    }
  }

  onMount(() => {
    initWorker();
    window.addEventListener('keydown', handleKeydown);
    return () => {
      window.removeEventListener('keydown', handleKeydown);
      terminateWorker();
    };
  });
</script>

<div class="seed-map-viewer">
  <SeedMapControls />
  <div class="map-main">
    <SeedMapCanvas />
    <SeedMapTooltip />
  </div>
</div>

<style>
  .seed-map-viewer {
    display: flex;
    gap: 12px;
    width: 100%;
    min-height: 500px;
    height: calc(100vh - 160px);
    max-height: 800px;
  }

  .map-main {
    flex: 1;
    position: relative;
    min-width: 0;
  }

  @media (max-width: 768px) {
    .seed-map-viewer {
      flex-direction: column;
      height: auto;
    }

    .map-main {
      min-height: 400px;
    }
  }
</style>
