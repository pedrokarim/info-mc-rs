<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import ColorSwatch from '$lib/components/ui/ColorSwatch.svelte';

  const { Story } = defineMeta({
    title: 'DA/Inputs/ColorSwatch',
    component: ColorSwatch,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      size: { control: 'select', options: ['sm', 'md', 'lg'] },
      selected: { control: 'boolean' },
    },
    args: {
      color: { r: 94, g: 144, b: 255, a: 255 },
      size: 'md',
      selected: false,
    },
  });
</script>

<Story name="Default" args={{ color: { r: 94, g: 144, b: 255, a: 255 } }} />

<Story name="Selected" args={{ color: { r: 235, g: 157, b: 42, a: 255 }, selected: true }} />

<Story name="Transparent" args={{ color: { r: 255, g: 107, b: 107, a: 128 } }} />

<Story name="Sizes">
  <div style="display: flex; gap: 12px; align-items: center;">
    <ColorSwatch color={{ r: 22, g: 154, b: 96, a: 255 }} size="sm" />
    <ColorSwatch color={{ r: 22, g: 154, b: 96, a: 255 }} size="md" />
    <ColorSwatch color={{ r: 22, g: 154, b: 96, a: 255 }} size="lg" />
  </div>
</Story>
