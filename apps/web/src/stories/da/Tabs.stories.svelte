<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Tabs from '$lib/components/ui/Tabs.svelte';

  const { Story } = defineMeta({
    title: 'DA/Navigation/Tabs',
    component: Tabs,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      selected: { control: 'text' }
    },
    args: {
      items: [
        { label: 'Infos', value: 'info' },
        { label: 'Joueurs', value: 'players' },
        { label: 'Mods', value: 'mods' }
      ],
      selected: 'info'
    }
  });
</script>

<Story name="Default" args={{
  items: [
    { label: 'Infos', value: 'info' },
    { label: 'Joueurs', value: 'players' },
    { label: 'Mods', value: 'mods' }
  ],
  selected: 'info'
}} />

<Story name="Two Tabs" args={{
  items: [
    { label: 'Java', value: 'java' },
    { label: 'Bedrock', value: 'bedrock' }
  ],
  selected: 'java'
}} />

<Story name="Many Tabs" args={{
  items: [
    { label: 'Overview', value: 'overview' },
    { label: 'Players', value: 'players' },
    { label: 'Plugins', value: 'plugins' },
    { label: 'Maps', value: 'maps' },
    { label: 'History', value: 'history' }
  ],
  selected: 'overview'
}} />
