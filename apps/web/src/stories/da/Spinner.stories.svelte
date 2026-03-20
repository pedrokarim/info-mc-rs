<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Spinner from '$lib/components/ui/Spinner.svelte';

  const { Story } = defineMeta({
    title: 'DA/Feedback/Spinner',
    component: Spinner,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      size: { control: { type: 'inline-radio' }, options: ['sm', 'md', 'lg'] },
      label: { control: 'text' }
    },
    args: { size: 'md', label: 'Chargement...' }
  });
</script>

<Story name="Small" args={{ size: 'sm' }} />
<Story name="Medium" args={{ size: 'md' }} />
<Story name="Large" args={{ size: 'lg' }} />

<Story name="All Sizes">
  <div style="display: flex; align-items: center; gap: 1.5rem;">
    <Spinner size="sm" />
    <Spinner size="md" />
    <Spinner size="lg" />
  </div>
</Story>

<Story name="With Text">
  <div style="display: flex; align-items: center; gap: 0.5rem; color: #5a7894; font-size: 0.85rem;">
    <Spinner size="sm" />
    <span>Chargement du serveur...</span>
  </div>
</Story>
