<script lang="ts">
  import CapeCard from '$lib/components/ui/CapeCard.svelte';
  import CapeDetailModal from '$lib/components/ui/CapeDetailModal.svelte';
  import { CAPES_CATALOG, CATEGORIES } from '$lib/data/capes-catalog';
  import type { CapeInfo } from '$lib/data/capes-catalog';

  let activeCategory = $state('all');
  let selectedCape = $state<CapeInfo | null>(null);

  const filtered = $derived(
    activeCategory === 'all'
      ? CAPES_CATALOG
      : CAPES_CATALOG.filter((c) => c.category === activeCategory)
  );
</script>

<svelte:head>
  <title>Capes Minecraft — Histoire et Collection — MCInfo</title>
  <meta name="description" content="Découvrez toutes les capes officielles Minecraft : MineCon, Mojang, Translator, Migrator et plus. Histoire, rareté et prévisualisation 3D." />
</svelte:head>

<main class="capes-page">
  <div class="page-header">
    <h1 class="page-title">Capes Minecraft</h1>
    <p class="page-subtitle">L'histoire complète des capes officielles — de MineCon 2011 à aujourd'hui</p>
    <span class="cape-count">{CAPES_CATALOG.length} capes</span>
  </div>

  <!-- Filters -->
  <div class="filters">
    {#each CATEGORIES as cat}
      <button
        class="filter-btn"
        class:active={activeCategory === cat.id}
        onclick={() => activeCategory = cat.id}
      >{cat.label}</button>
    {/each}
  </div>

  <!-- Grid -->
  <div class="capes-grid">
    {#each filtered as cape (cape.id)}
      <CapeCard {cape} onclick={() => selectedCape = cape} />
    {/each}
  </div>

  {#if filtered.length === 0}
    <p class="empty-state">Aucune cape dans cette catégorie.</p>
  {/if}
</main>

<!-- Detail modal -->
<CapeDetailModal bind:cape={selectedCape} />

<style>
  .capes-page {
    width: var(--layout-width, min(1160px, calc(100% - 2rem)));
    margin: 0 auto;
    padding-top: 2rem;
    padding-bottom: 3rem;
  }

  .page-header { margin-bottom: 1.5rem; }

  .page-title {
    font-family: 'Teko', sans-serif;
    font-size: 2.6rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1;
  }

  .page-subtitle {
    font-size: 0.95rem;
    color: var(--ink-2, #5a7894);
    margin: 0.3rem 0 0;
  }

  .cape-count {
    display: inline-block;
    margin-top: 0.4rem;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--blue-0, #5e90ff);
    background: rgba(94, 144, 255, 0.1);
    padding: 2px 10px;
    border-radius: 999px;
  }

  .filters {
    display: flex;
    gap: 6px;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .filter-btn {
    padding: 6px 16px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 999px;
    background: transparent;
    color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    transition: background var(--ease, 160ms ease), border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .filter-btn:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
  .filter-btn.active { background: var(--blue-0, #5e90ff); border-color: var(--blue-0, #5e90ff); color: #fff; }

  .capes-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
  }

  .empty-state {
    text-align: center;
    color: var(--ink-2, #5a7894);
    padding: 3rem 0;
    font-size: 0.9rem;
  }

  @media (max-width: 980px) { .capes-grid { grid-template-columns: repeat(3, 1fr); } }
  @media (max-width: 700px) { .capes-grid { grid-template-columns: repeat(2, 1fr); } }
  @media (max-width: 460px) { .capes-grid { grid-template-columns: 1fr; } }
</style>
