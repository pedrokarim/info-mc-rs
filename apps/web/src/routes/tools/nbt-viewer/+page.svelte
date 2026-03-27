<script lang="ts">
  import NbtViewer from '$lib/components/editor/NbtViewer.svelte';
</script>

<svelte:head>
  <title>Visualiseur NBT Minecraft — MCInfo</title>
  <meta name="description" content="Visualisez vos fichiers NBT Minecraft : arbre de tags et rendu 3D des structures. Support structure blocks et schematics." />
</svelte:head>

<main class="nbt-page">
  <div class="page-header">
    <h1 class="page-title">Visualiseur NBT</h1>
    <p class="page-subtitle">Explorez vos fichiers NBT Minecraft en arbre et en 3D</p>
  </div>
  <NbtViewer />
</main>

<style>
  .nbt-page { width: var(--layout-width, min(1160px, calc(100% - 2rem))); margin: 0 auto; padding-top: 1rem; padding-bottom: 2rem; }
  .page-header { margin-bottom: 1rem; }
  .page-title { font-family: 'Teko', sans-serif; font-size: 2rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1.1; }
  .page-subtitle { font-size: 0.88rem; color: var(--ink-2, #5a7894); margin: 0.25rem 0 0; }
</style>
