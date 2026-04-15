<script lang="ts">
  import CoordinateCalculator from '$lib/components/editor/CoordinateCalculator.svelte';
</script>

<svelte:head>
  <title>Calculateur de Coordonnées Minecraft — MCInfo</title>
  <meta name="description" content="Convertissez vos coordonnées Overworld ↔ Nether, trouvez votre chunk et région, calculez les distances et spawn chunks." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Calculateur de Coordonnées</h1>
    <p class="editor-subtitle">Conversion Overworld ↔ Nether, chunks, régions et distances</p>
  </div>
  <CoordinateCalculator />
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
