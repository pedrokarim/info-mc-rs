<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import LookupStrip from '$lib/components/ui/LookupStrip.svelte';

  const { Story } = defineMeta({
    title: 'DA/Sections/LookupStrip',
    component: LookupStrip,
    tags: ['autodocs'],
    parameters: {
      layout: 'fullscreen'
    },
    argTypes: {
      heading: { control: 'text' },
      description: { control: 'text' },
      action: { control: 'text' },
      serverPlaceholder: { control: 'text' },
      playerPlaceholder: { control: 'text' }
    },
    args: {
      heading: 'Lookup rapide',
      description: 'Entre une adresse ou un pseudo pour ouvrir la fiche detaillee.',
      action: 'Analyser',
      serverPlaceholder: 'play.hypixel.net ou 172.65.128.35:25565',
      playerPlaceholder: 'Notch, Dream ou UUID'
    }
  });
</script>

<Story name="Default Strip" />

<Story
  name="Player Search Copy"
  args={{
    heading: 'Lookup skin / joueur',
    description: 'Tape un pseudo pour ouvrir la fiche detaillee style NameMC.',
    action: 'Rechercher'
  }}
/>
