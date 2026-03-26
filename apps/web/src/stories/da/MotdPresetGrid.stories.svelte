<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import MotdPresetGrid from '$lib/components/ui/MotdPresetGrid.svelte';

  const { Story } = defineMeta({
    title: 'DA/Data/MotdPresetGrid',
    component: MotdPresetGrid,
    tags: ['autodocs'],
    parameters: { layout: 'padded' },
    args: {
      edition: 'java',
    },
  });
</script>

<Story name="Java Edition" />
<Story name="Bedrock Edition" args={{ edition: 'bedrock' }} />
