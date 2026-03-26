<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import ColorPicker from '$lib/components/ui/ColorPicker.svelte';

  const { Story } = defineMeta({
    title: 'DA/Inputs/ColorPicker',
    component: ColorPicker,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      showAlpha: { control: 'boolean' },
    },
    args: {
      color: { r: 255, g: 0, b: 0, a: 255 },
      showAlpha: true,
    },
  });
</script>

<Story name="Default" />

<Story name="No Alpha" args={{ showAlpha: false }} />

<Story name="Preset Blue" args={{ color: { r: 94, g: 144, b: 255, a: 255 } }} />

<Story name="Semi-Transparent" args={{ color: { r: 22, g: 154, b: 96, a: 128 } }} />
