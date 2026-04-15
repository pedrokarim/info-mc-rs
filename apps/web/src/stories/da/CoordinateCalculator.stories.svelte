<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import CoordinateCalculator from '$lib/components/editor/CoordinateCalculator.svelte';

  const { Story } = defineMeta({
    title: 'DA/Editors/CoordinateCalculator',
    component: CoordinateCalculator,
    tags: ['autodocs'],
    parameters: {
      layout: 'padded'
    }
  });
</script>

<Story name="Default" />
