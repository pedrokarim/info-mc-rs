<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Toast from '$lib/components/ui/Toast.svelte';
  import { addToast } from '$lib/stores/toasts';

  const { Story } = defineMeta({
    title: 'DA/Feedback/Toast',
    component: Toast,
    tags: ['autodocs'],
    parameters: { layout: 'fullscreen' }
  });
</script>

<Story name="Interactive Demo">
  <div style="padding: 2rem; display: flex; flex-direction: column; gap: 0.8rem; align-items: flex-start;">
    <p style="font-size: 0.85rem; color: #5a7894; margin: 0;">Cliquez pour déclencher un toast :</p>
    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
      <button style="padding: 0.4rem 0.8rem; border-radius: 6px; border: 1px solid #169a60; background: rgba(22,154,96,0.1); color: #169a60; cursor: pointer; font-weight: 600; font-size: 0.8rem;" onclick={() => addToast('Serveur ajouté aux favoris', 'success')}>Success</button>
      <button style="padding: 0.4rem 0.8rem; border-radius: 6px; border: 1px solid #b83b3b; background: rgba(184,59,59,0.1); color: #b83b3b; cursor: pointer; font-weight: 600; font-size: 0.8rem;" onclick={() => addToast('Impossible de contacter le serveur', 'error')}>Error</button>
      <button style="padding: 0.4rem 0.8rem; border-radius: 6px; border: 1px solid #b8860b; background: rgba(210,153,34,0.1); color: #b8860b; cursor: pointer; font-weight: 600; font-size: 0.8rem;" onclick={() => addToast('Rate limit atteint, réessayez dans 30s', 'warning')}>Warning</button>
      <button style="padding: 0.4rem 0.8rem; border-radius: 6px; border: 1px solid #5e90ff; background: rgba(94,144,255,0.1); color: #5e90ff; cursor: pointer; font-weight: 600; font-size: 0.8rem;" onclick={() => addToast('Skin mis à jour', 'info')}>Info</button>
    </div>
  </div>
  <Toast />
</Story>
