<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Switch from '$lib/components/ui/Switch.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Switch',
    component: Switch,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      checked: { control: 'boolean' },
      label: { control: 'text' },
      disabled: { control: 'boolean' }
    },
    args: { checked: false, label: 'Mode sombre', disabled: false }
  });
</script>

<Story name="Off" args={{ label: 'Mode sombre', checked: false }} />
<Story name="On" args={{ label: 'Notifications', checked: true }} />
<Story name="Disabled" args={{ label: 'Option verrouillée', disabled: true }} />

<Story name="Settings">
  <div style="display: flex; flex-direction: column; gap: 0.8rem;">
    <Switch label="Afficher les skins" checked={true} />
    <Switch label="Mode maintenance" />
    <Switch label="Auto-refresh" checked={true} />
    <Switch label="2FA requis" disabled />
  </div>
</Story>
