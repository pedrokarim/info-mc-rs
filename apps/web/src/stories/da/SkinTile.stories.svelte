<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import SkinTile from '$lib/components/ui/SkinTile.svelte';

  const { Story } = defineMeta({
    title: 'DA/Cards/SkinTile',
    component: SkinTile,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      username: { control: 'text' },
      tag: { control: 'text' },
      image: { control: 'text' },
      href: { control: 'text' }
    },
    args: {
      username: 'Notch',
      tag: 'Classic legend',
      image: '/images/skins/skin-placeholder-head-v01.png',
      href: '/player/Notch'
    }
  });
</script>

<Story name="Single Tile" />

<Story name="Grid Preview">
  <div class="gallery-grid" style="max-width: 920px;">
    <SkinTile username="Notch" tag="Classic legend" image="/images/skins/skin-placeholder-head-v01.png" />
    <SkinTile username="Dream" tag="Iconic profile" image="/images/skins/skin-placeholder-head-v01.png" />
    <SkinTile username="Technoblade" tag="Community favorite" image="/images/skins/skin-placeholder-head-v01.png" />
    <SkinTile username="Skeppy" tag="PvP style" image="/images/skins/skin-placeholder-head-v01.png" />
  </div>
</Story>
