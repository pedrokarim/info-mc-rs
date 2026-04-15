<script lang="ts">
  import EnchantmentCalculator from '$lib/components/editor/EnchantmentCalculator.svelte';
</script>

<svelte:head>
  <title>Calculateur d'Enchantements Minecraft — MCInfo</title>
  <meta name="description" content="Calculez le coût XP optimal pour enchanter vos items Minecraft. Ordre d'enclume, détection Too Expensive, et commande /give." />
</svelte:head>

<main class="editor-page">
  <div class="editor-header">
    <h1 class="editor-title">Calculateur d'Enchantements</h1>
    <p class="editor-subtitle">Calculez le coût XP optimal et évitez le "Too Expensive!"</p>
  </div>
  <EnchantmentCalculator />
</main>

<style>
  .editor-page { width: var(--layout-width, min(1160px, calc(100% - 2rem))); margin: 0 auto; padding-top: 1rem; padding-bottom: 2rem; }
  .editor-header { margin-bottom: 1rem; }
  .editor-title { font-family: 'Teko', sans-serif; font-size: 2rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1.1; }
  .editor-subtitle { font-size: 0.88rem; color: var(--ink-2, #5a7894); margin: 0.25rem 0 0; }
</style>
