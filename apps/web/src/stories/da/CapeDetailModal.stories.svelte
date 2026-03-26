<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import CapeDetailModal from '$lib/components/ui/CapeDetailModal.svelte';

  const { Story } = defineMeta({
    title: 'DA/Overlay/CapeDetailModal',
    component: CapeDetailModal,
    tags: ['autodocs'],
    parameters: { layout: 'fullscreen' },
  });
</script>

<Story name="Legendary Cape">
  <CapeDetailModal cape={{
    id: 'minecon-2011', name: 'Minecon 2011', file: 'minecon-2011.png', year: '2011',
    rarity: 'legendary', category: 'event',
    description: 'Première cape MineCon, offerte à Las Vegas.',
    descriptionLong: "La première MineCon s'est tenue les 18-19 novembre 2011 au Mandalay Bay à Las Vegas, coïncidant avec la sortie officielle de Minecraft 1.0. Les quelques milliers de participants ont reçu cette cape verte emblématique.",
  }} />
</Story>
