<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import FireworkDesigner from '$lib/components/editor/FireworkDesigner.svelte';

  const { Story } = defineMeta({
    title: 'DA/Editors/FireworkDesigner',
    component: FireworkDesigner,
    tags: ['autodocs'],
    parameters: { layout: 'padded' }
  });
</script>

<Story name="Default" />
