<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import BannerDesigner from '$lib/components/editor/BannerDesigner.svelte';

  const { Story } = defineMeta({
    title: 'DA/Editors/BannerDesigner',
    component: BannerDesigner,
    tags: ['autodocs'],
    parameters: { layout: 'padded' }
  });
</script>

<Story name="Default" />
