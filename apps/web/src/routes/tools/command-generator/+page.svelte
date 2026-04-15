<script lang="ts">
  import CommandGenerator from '$lib/components/editor/CommandGenerator.svelte';
</script>

<svelte:head>
  <title>Générateur de Commandes Minecraft — MCInfo</title>
  <meta name="description" content="Générez des commandes Minecraft : /give, /summon, /setblock, /fill, /particle, /effect, /tp et plus. Autocomplete sur les IDs." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Générateur de Commandes</h1>
    <p class="editor-subtitle">Construisez vos commandes Minecraft avec autocomplete et preview en temps réel</p>
  </div>
  <CommandGenerator />
</main>

<style>
  .editor-page { width: var(--layout-width, min(1160px, calc(100% - 2rem))); margin: 0 auto; padding-top: 1rem; padding-bottom: 2rem; }
  .editor-header { margin-bottom: 1rem; }
  .editor-title { font-family: 'Teko', sans-serif; font-size: 2rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1.1; }
  .editor-subtitle { font-size: 0.88rem; color: var(--ink-2, #5a7894); margin: 0.25rem 0 0; }
</style>
