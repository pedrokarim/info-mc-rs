<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Radio from '$lib/components/ui/Radio.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Radio',
    component: Radio,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      selected: { control: 'text' },
      direction: {
        control: { type: 'inline-radio' },
        options: ['vertical', 'horizontal']
      }
    },
    args: {
      items: [
        { label: 'Java Edition', value: 'java' },
        { label: 'Bedrock Edition', value: 'bedrock' },
        { label: 'Pocket Edition', value: 'pocket', disabled: true }
      ],
      name: 'edition',
      selected: 'java',
      direction: 'vertical'
    }
  });
</script>

<Story name="Vertical" args={{
  items: [
    { label: 'Java Edition', value: 'java' },
    { label: 'Bedrock Edition', value: 'bedrock' },
    { label: 'Pocket Edition', value: 'pocket', disabled: true }
  ],
  name: 'edition-v',
  selected: 'java',
  direction: 'vertical'
}} />

<Story name="Horizontal" args={{
  items: [
    { label: 'Survie', value: 'survival' },
    { label: 'Créatif', value: 'creative' },
    { label: 'Aventure', value: 'adventure' }
  ],
  name: 'gamemode',
  selected: 'survival',
  direction: 'horizontal'
}} />
