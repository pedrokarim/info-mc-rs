<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';

  const { Story } = defineMeta({
    title: 'DA/Feedback/StatusPill',
    component: StatusPill,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      label: { control: 'text' },
      kind: {
        control: { type: 'inline-radio' },
        options: ['neutral', 'online', 'offline', 'loading']
      }
    },
    args: {
      label: 'Online',
      kind: 'online'
    }
  });
</script>

<Story name="Neutral" args={{ label: 'En attente', kind: 'neutral' }} />
<Story name="Online" args={{ label: 'Online', kind: 'online' }} />
<Story name="Offline" args={{ label: 'Offline', kind: 'offline' }} />
<Story name="Loading" args={{ label: 'Chargement', kind: 'loading' }} />
