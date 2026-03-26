<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import CapeCard from '$lib/components/ui/CapeCard.svelte';

  const { Story } = defineMeta({
    title: 'DA/Content/CapeCard',
    component: CapeCard,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
  });
</script>

<Story name="Legendary">
  <div style="width: 240px;">
    <CapeCard cape={{
      id: 'mojang', name: 'Mojang', file: 'mojang.png', year: '2010',
      rarity: 'legendary', category: 'mojang',
      description: 'Cape réservée aux employés de Mojang Studios.',
      descriptionLong: '',
    }} />
  </div>
</Story>

<Story name="Event">
  <div style="width: 240px;">
    <CapeCard cape={{
      id: 'cherry-blossom', name: 'Cherry Blossom', file: 'cherry-blossom.png', year: '2023',
      rarity: 'event', category: 'event',
      description: "Célèbre l'ajout des cerisiers dans la mise à jour 1.20.",
      descriptionLong: '',
    }} />
  </div>
</Story>

<Story name="Common">
  <div style="width: 240px;">
    <CapeCard cape={{
      id: 'migrator', name: 'Migrator', file: 'migrator.png', year: '2021',
      rarity: 'common', category: 'special',
      description: 'Offerte pour la migration du compte Mojang vers Microsoft.',
      descriptionLong: '',
    }} />
  </div>
</Story>
