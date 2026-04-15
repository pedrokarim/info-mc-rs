<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import EnchantmentCalculator from '$lib/components/editor/EnchantmentCalculator.svelte';

  const { Story } = defineMeta({
    title: 'DA/Editors/EnchantmentCalculator',
    component: EnchantmentCalculator,
    tags: ['autodocs'],
    parameters: { layout: 'padded' }
  });
</script>

<Story name="Default" />
