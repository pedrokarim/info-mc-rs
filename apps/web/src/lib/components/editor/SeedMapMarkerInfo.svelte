<script lang="ts">
  import { mapState, setCenter } from '$lib/stores/seed-map.svelte';

  function close() {
    mapState.selectedMarker = null;
  }

  function copyCoords() {
    if (!mapState.selectedMarker) return;
    const text = `${mapState.selectedMarker.x} ${mapState.selectedMarker.z}`;
    navigator.clipboard?.writeText(text);
  }

  function teleport() {
    if (!mapState.selectedMarker) return;
    setCenter(mapState.selectedMarker.x, mapState.selectedMarker.z);
  }
</script>

{#if mapState.selectedMarker}
  <div class="marker-info">
    <button class="close-btn" onclick={close} title="Fermer">×</button>
    <div class="header">
      <span class="name">{mapState.selectedMarker.name}</span>
    </div>
    <div class="grid">
      <span class="k">Bloc</span>
      <span class="v">{mapState.selectedMarker.x}, {mapState.selectedMarker.z}</span>
      <span class="k">Chunk</span>
      <span class="v">{Math.floor(mapState.selectedMarker.x / 16)}, {Math.floor(mapState.selectedMarker.z / 16)}</span>
      {#if mapState.selectedMarker.biome}
        <span class="k">Biome</span>
        <span class="v">{mapState.selectedMarker.biome}</span>
      {/if}
    </div>
    <div class="actions">
      <button class="action-btn" onclick={copyCoords}>📋 Copier</button>
      <button class="action-btn" onclick={teleport}>🎯 Centrer</button>
    </div>
  </div>
{/if}

<style>
  .marker-info {
    position: absolute;
    top: 12px;
    right: 12px;
    min-width: 200px;
    max-width: 280px;
    padding: 12px 14px 10px;
    background: rgba(15, 25, 40, 0.95);
    border: 1px solid var(--blue-0, #5e90ff);
    border-radius: 10px;
    color: #fff;
    font-size: 0.78rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(6px);
    z-index: 20;
  }

  .close-btn {
    position: absolute;
    top: 4px;
    right: 6px;
    background: transparent;
    border: none;
    color: #fff;
    font-size: 1.3rem;
    cursor: pointer;
    line-height: 1;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .close-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .header {
    margin-bottom: 8px;
    padding-right: 24px;
  }

  .name {
    font-weight: 700;
    font-size: 0.9rem;
    color: var(--blue-0, #7eaaff);
  }

  .grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 3px 12px;
    margin-bottom: 10px;
  }

  .k {
    color: rgba(255, 255, 255, 0.5);
    font-weight: 600;
    font-size: 0.7rem;
    text-transform: uppercase;
  }

  .v {
    font-family: monospace;
    color: #fff;
  }

  .actions {
    display: flex;
    gap: 6px;
  }

  .action-btn {
    flex: 1;
    padding: 5px 8px;
    background: rgba(94, 144, 255, 0.15);
    border: 1px solid rgba(94, 144, 255, 0.4);
    border-radius: 6px;
    color: #fff;
    font-size: 0.72rem;
    cursor: pointer;
    transition: background 120ms ease;
  }
  .action-btn:hover {
    background: rgba(94, 144, 255, 0.3);
  }
</style>
