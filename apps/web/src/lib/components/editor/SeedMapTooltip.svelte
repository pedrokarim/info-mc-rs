<script lang="ts">
  import { mapState } from '$lib/stores/seed-map.svelte';
</script>

{#if mapState.hoverActive && mapState.seedValid && mapState.hoverBiome}
  <div class="tooltip">
    <span class="tooltip-biome">{mapState.hoverBiome}</span>
    <span class="tooltip-coords">{mapState.hoverWorldX}, {mapState.hoverWorldZ}</span>
    {#if mapState.hoverIsSlime}
      <span class="tooltip-slime">Slime Chunk</span>
    {/if}
  </div>
{/if}

<style>
  .tooltip {
    position: absolute;
    top: 8px;
    right: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    padding: 0.4rem 0.6rem;
    background: rgba(0, 0, 0, 0.8);
    border-radius: 6px;
    pointer-events: none;
    z-index: 10;
  }

  .tooltip-biome {
    font-size: 0.82rem;
    font-weight: 600;
    color: #fff;
  }

  .tooltip-coords {
    font-size: 0.72rem;
    font-family: monospace;
    color: #aaa;
  }

  .tooltip-slime {
    font-size: 0.7rem;
    font-weight: 600;
    color: #6de68a;
  }
</style>
