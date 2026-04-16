<script lang="ts">
  import { onMount } from 'svelte';
  import SeedMapCanvas from './SeedMapCanvas.svelte';
  import SeedMapControls from './SeedMapControls.svelte';
  import SeedMapTooltip from './SeedMapTooltip.svelte';
  import SeedMapMarkerInfo from './SeedMapMarkerInfo.svelte';
  import {
    mapState, initWorkers, terminateWorkers, pan, zoomIn, zoomOut,
    requestVisibleTiles, setSeed, persistState, restoreState, randomSeed,
  } from '$lib/stores/seed-map.svelte';

  function handleKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement)?.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    const step = 16 / mapState.zoom;

    switch (e.key) {
      case 'ArrowLeft':
        e.preventDefault();
        pan(-step, 0);
        requestVisibleTiles();
        break;
      case 'ArrowRight':
        e.preventDefault();
        pan(step, 0);
        requestVisibleTiles();
        break;
      case 'ArrowUp':
        e.preventDefault();
        pan(0, -step);
        requestVisibleTiles();
        break;
      case 'ArrowDown':
        e.preventDefault();
        pan(0, step);
        requestVisibleTiles();
        break;
      case '+':
      case '=':
        zoomIn();
        requestVisibleTiles();
        break;
      case '-':
        zoomOut();
        requestVisibleTiles();
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

  // Persist state on every meaningful change (debounced)
  let persistTimeout: ReturnType<typeof setTimeout> | null = null;
  function schedulePersist() {
    if (persistTimeout) clearTimeout(persistTimeout);
    persistTimeout = setTimeout(persistState, 300);
  }

  // Watch all persisted fields
  $effect(() => {
    // Touch all reactive fields we want to persist
    void mapState.seedInput;
    void mapState.mcVersion;
    void mapState.dimension;
    void mapState.edition;
    void mapState.centerX;
    void mapState.centerZ;
    void mapState.zoom;
    void mapState.showBiomes;
    void mapState.showSlimeChunks;
    void mapState.showStructures;
    void mapState.showGrid;
    void mapState.showCoordinates;
    void mapState.enabledStructures;
    schedulePersist();
  });

  onMount(() => {
    initWorkers();

    // Restore from URL or localStorage
    const restored = restoreState();
    if (restored && mapState.seedInput) {
      setSeed(mapState.seedInput);
    } else if (!mapState.seedInput) {
      // No seed persisted — start with a random one
      const seed = randomSeed();
      mapState.seedInput = seed;
      setSeed(seed);
    }

    window.addEventListener('keydown', handleKeydown);
    return () => {
      window.removeEventListener('keydown', handleKeydown);
      if (persistTimeout) clearTimeout(persistTimeout);
      terminateWorkers();
    };
  });
</script>

<div class="seed-map-viewer">
  <SeedMapControls />
  <div class="map-main">
    <SeedMapCanvas />
    <SeedMapTooltip />
    <SeedMapMarkerInfo />
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
