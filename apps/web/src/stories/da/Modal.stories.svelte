<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Modal from '$lib/components/ui/Modal.svelte';
  import GameButton from '$lib/components/ui/GameButton.svelte';

  const { Story } = defineMeta({
    title: 'DA/Overlay/Modal',
    component: Modal,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      open: { control: 'boolean' },
      title: { control: 'text' },
      closable: { control: 'boolean' }
    },
    args: { open: true, title: 'Confirmer', closable: true }
  });
</script>

<Story name="Basic">
  <Modal open={true} title="Confirmer l'action">
    <p>Voulez-vous vraiment supprimer ce serveur de l'index ?</p>
    {#snippet footer()}
      <GameButton label="Annuler" variant="secondary" compact href="#" />
      <GameButton label="Supprimer" variant="primary" compact href="#" />
    {/snippet}
  </Modal>
</Story>

<Story name="No Title">
  <Modal open={true} closable={true}>
    <p style="text-align: center; font-size: 1rem;">Session expirée. Veuillez vous reconnecter.</p>
    {#snippet footer()}
      <GameButton label="Se reconnecter" variant="primary" compact href="#" />
    {/snippet}
  </Modal>
</Story>

<Story name="Not Closable">
  <Modal open={true} title="Maintenance en cours" closable={false}>
    <p>Le serveur est en mode maintenance. Veuillez patienter.</p>
  </Modal>
</Story>

<Story name="Long Content">
  <Modal open={true} title="Audit Log">
    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
      {#each Array(15) as _, i}
        <p style="margin: 0; font-size: 0.82rem; padding: 0.3rem; border-bottom: 1px solid rgba(84,126,181,0.12);">
          [{new Date(Date.now() - i * 3600000).toISOString().slice(0, 16)}] Action #{i + 1} — admin modifié config
        </p>
      {/each}
    </div>
  </Modal>
</Story>
