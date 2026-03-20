<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Accordion from '$lib/components/ui/Accordion.svelte';

  const { Story } = defineMeta({
    title: 'DA/Layout/Accordion',
    component: Accordion,
    tags: ['autodocs'],
    parameters: { layout: 'centered' }
  });
</script>

<Story name="Single Open">
  <div style="width: 360px;">
    <Accordion items={[
      { label: 'Informations', value: 'info' },
      { label: 'Joueurs en ligne', value: 'players' },
      { label: 'Plugins', value: 'plugins' }
    ]}>
      {#snippet children({ value })}
        {#if value === 'info'}
          <p style="margin: 0;">Version 1.21.1 — Java Edition — PvE Survival</p>
        {:else if value === 'players'}
          <p style="margin: 0;">42 joueurs connectés sur 100 max</p>
        {:else}
          <p style="margin: 0;">WorldEdit, EssentialsX, Vault, LuckPerms</p>
        {/if}
      {/snippet}
    </Accordion>
  </div>
</Story>

<Story name="Multiple Open">
  <div style="width: 360px;">
    <Accordion multiple items={[
      { label: 'FAQ #1 — Comment rejoindre ?', value: 'q1' },
      { label: 'FAQ #2 — Serveur payant ?', value: 'q2' },
      { label: 'FAQ #3 — Discord ?', value: 'q3' }
    ]}>
      {#snippet children({ value })}
        {#if value === 'q1'}
          <p style="margin: 0;">Lancez Minecraft 1.21+ et ajoutez l'adresse play.example.net</p>
        {:else if value === 'q2'}
          <p style="margin: 0;">Non, le serveur est 100% gratuit</p>
        {:else}
          <p style="margin: 0;">Rejoignez-nous sur discord.gg/example</p>
        {/if}
      {/snippet}
    </Accordion>
  </div>
</Story>
