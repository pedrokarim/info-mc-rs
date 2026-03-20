<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Badge from '$lib/components/ui/Badge.svelte';

  const { Story } = defineMeta({
    title: 'DA/Feedback/Badge',
    component: Badge,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      label: { control: 'text' },
      variant: {
        control: { type: 'inline-radio' },
        options: ['default', 'success', 'warning', 'danger', 'info']
      },
      size: {
        control: { type: 'inline-radio' },
        options: ['sm', 'md']
      }
    },
    args: { label: 'Java', variant: 'default', size: 'md' }
  });
</script>

<Story name="Default" args={{ label: 'Java', variant: 'default' }} />
<Story name="Success" args={{ label: 'Online', variant: 'success' }} />
<Story name="Warning" args={{ label: 'Maintenance', variant: 'warning' }} />
<Story name="Danger" args={{ label: 'Banned', variant: 'danger' }} />
<Story name="Info" args={{ label: 'Bedrock', variant: 'info' }} />
<Story name="Small" args={{ label: 'v1.21', variant: 'default', size: 'sm' }} />

<Story name="All Variants">
  <div style="display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap;">
    <Badge label="Java" variant="default" />
    <Badge label="Online" variant="success" />
    <Badge label="Maintenance" variant="warning" />
    <Badge label="Banned" variant="danger" />
    <Badge label="Bedrock" variant="info" />
  </div>
</Story>
