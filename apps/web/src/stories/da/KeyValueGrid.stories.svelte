<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import KeyValueGrid from '$lib/components/ui/KeyValueGrid.svelte';

  const { Story } = defineMeta({
    title: 'DA/Data/KeyValueGrid',
    component: KeyValueGrid,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      items: [
        { key: 'Hostname', value: 'play.hypixel.net' },
        { key: 'IP', value: '172.65.128.35' },
        { key: 'Port', value: 25565 },
        { key: 'Edition', value: 'java' },
        { key: 'Latency', value: '48 ms' }
      ]
    }
  });
</script>

<Story name="Server Meta" />
