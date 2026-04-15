<script lang="ts">
  import BannerDesigner from '$lib/components/editor/BannerDesigner.svelte';
</script>

<svelte:head>
  <title>Designer de Bannière Minecraft — MCInfo</title>
  <meta name="description" content="Créez vos bannières Minecraft avec un éditeur visuel. Composez couche par couche avec 39 patterns et 16 couleurs, puis copiez la commande /give." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Designer de Bannière</h1>
    <p class="editor-subtitle">Composez votre bannière couche par couche et copiez la commande</p>
  </div>
  <BannerDesigner />
</main>

<style>
  .editor-page { width: var(--layout-width, min(1160px, calc(100% - 2rem))); margin: 0 auto; padding-top: 1rem; padding-bottom: 2rem; }
  .editor-header { margin-bottom: 1rem; }
  .editor-title { font-family: 'Teko', sans-serif; font-size: 2rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1.1; }
  .editor-subtitle { font-size: 0.88rem; color: var(--ink-2, #5a7894); margin: 0.25rem 0 0; }
</style>
