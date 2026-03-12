<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import CapePreview from '$lib/components/ui/CapePreview.svelte';

  const OPTIFINE_CAPE = 'http://s.optifine.net/capes/Notch.png';

  const { Story } = defineMeta({
    title: 'DA/Cards/CapePreview',
    component: CapePreview,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      url: { control: 'text' },
      scale: { control: { type: 'range', min: 1, max: 16, step: 1 } }
    },
    args: {
      url: OPTIFINE_CAPE,
      scale: 8
    }
  });
</script>

<Story name="Default" />

<Story name="Small Scale" args={{ scale: 4 }} />

<Story name="Large Scale" args={{ scale: 12 }} />

<Story name="Error State" args={{ url: 'https://invalid.example/cape.png' }} />
