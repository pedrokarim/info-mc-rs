<script lang="ts">
  import CapeEditor from '$lib/components/editor/CapeEditor.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>Éditeur de Cape Minecraft — MCInfo</title>
  <meta name="description" content="Créez et personnalisez vos capes Minecraft. Mode OptiFine, presets officiels, vue elytra, peinture 3D en temps réel." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Éditeur de Cape</h1>
    <p class="editor-subtitle">Créez et personnalisez votre cape Minecraft</p>
  </div>
  <CapeEditor apiBase={data.apiBase} />
</main>

<style>
  .editor-page { width: var(--layout-width, min(1160px, calc(100% - 2rem))); margin: 0 auto; padding-top: 1rem; padding-bottom: 2rem; }
  .editor-header { margin-bottom: 1rem; }
  .editor-title { font-family: 'Teko', sans-serif; font-size: 2rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1.1; }
  .editor-subtitle { font-size: 0.88rem; color: var(--ink-2, #5a7894); margin: 0.25rem 0 0; }
</style>
