<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Select from '$lib/components/ui/Select.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Select',
    component: Select,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      value: { control: 'text' },
      label: { control: 'text' },
      placeholder: { control: 'text' },
      disabled: { control: 'boolean' }
    },
    args: {
      options: [
        { label: 'Java Edition', value: 'java' },
        { label: 'Bedrock Edition', value: 'bedrock' }
      ],
      value: 'java',
      label: 'Edition'
    }
  });
</script>

<Story name="Default" args={{
  options: [
    { label: 'Java Edition', value: 'java' },
    { label: 'Bedrock Edition', value: 'bedrock' }
  ],
  label: 'Edition',
  value: 'java'
}} />

<Story name="With Placeholder" args={{
  options: [
    { label: 'Survie', value: 'survival' },
    { label: 'Créatif', value: 'creative' },
    { label: 'Aventure', value: 'adventure' },
    { label: 'Spectateur', value: 'spectator' }
  ],
  label: 'Mode de jeu',
  placeholder: 'Sélectionner...',
  value: ''
}} />

<Story name="Disabled" args={{
  options: [{ label: 'Java', value: 'java' }],
  label: 'Edition',
  value: 'java',
  disabled: true
}} />
