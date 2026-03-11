<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import TogglePills from '$lib/components/ui/TogglePills.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/TogglePills',
    component: TogglePills,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      options: [
        { label: 'Auto', value: 'auto' },
        { label: 'Java', value: 'java' },
        { label: 'Bedrock', value: 'bedrock' }
      ],
      value: 'auto',
      name: 'edition',
      ariaLabel: 'Mode serveur'
    }
  });
</script>

<Story name="Editions" />

<Story
  name="Lookup Mode"
  args={{
    options: [
      { label: 'Server Lookup', value: 'server' },
      { label: 'Player / Skin Lookup', value: 'player' }
    ],
    value: 'server',
    name: 'lookup',
    ariaLabel: 'Mode lookup'
  }}
/>
