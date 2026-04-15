<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import CommandGenerator from '$lib/components/editor/CommandGenerator.svelte';

  const { Story } = defineMeta({
    title: 'DA/Editors/CommandGenerator',
    component: CommandGenerator,
    tags: ['autodocs'],
    parameters: { layout: 'padded' }
  });
</script>

<Story name="Default" />
