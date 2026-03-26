<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import ColorPalette from '$lib/components/ui/ColorPalette.svelte';

  const { Story } = defineMeta({
    title: 'DA/Inputs/ColorPalette',
    component: ColorPalette,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      selected: { r: 176, g: 46, b: 38, a: 255 },
    },
  });
</script>

<Story name="Default" />

<Story name="With Recent Colors" args={{
  selected: { r: 94, g: 144, b: 255, a: 255 },
  recentColors: [
    { r: 255, g: 0, b: 0, a: 255 },
    { r: 0, g: 255, b: 0, a: 255 },
    { r: 0, g: 0, b: 255, a: 255 },
    { r: 255, g: 255, b: 0, a: 255 },
    { r: 128, g: 0, b: 128, a: 255 },
  ],
}} />
