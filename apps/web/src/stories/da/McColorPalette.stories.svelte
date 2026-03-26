<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import McColorPalette from '$lib/components/ui/McColorPalette.svelte';

  const { Story } = defineMeta({
    title: 'DA/Input/McColorPalette',
    component: McColorPalette,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      edition: 'java',
      selected: '#FFAA00',
    },
  });
</script>

<Story name="Java Edition" />
<Story name="Bedrock Edition" args={{ edition: 'bedrock' }} />
<Story name="No Selection" args={{ selected: null }} />
