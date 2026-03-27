<script lang="ts">
  import SeedMapViewer from '$lib/components/editor/SeedMapViewer.svelte';
</script>

<svelte:head>
  <title>Carte de Seed Minecraft — MCInfo</title>
  <meta name="description" content="Visualisez les biomes, slime chunks et structures de votre monde Minecraft à partir de sa seed. Support 1.7 à 1.21." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Carte de Seed</h1>
    <p class="editor-subtitle">Visualisez les biomes et slime chunks de votre monde Minecraft</p>
  </div>
  <SeedMapViewer />
</main>

<style>
  .editor-page {
    width: var(--layout-width, min(1160px, calc(100% - 2rem)));
    margin: 0 auto;
    padding-top: 1rem;
    padding-bottom: 2rem;
  }

  .editor-header {
    margin-bottom: 1rem;
  }

  .editor-title {
    font-family: 'Teko', sans-serif;
    font-size: 2rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1.1;
  }

  .editor-subtitle {
    font-size: 0.88rem;
    color: var(--ink-2, #5a7894);
    margin: 0.25rem 0 0;
  }
</style>
