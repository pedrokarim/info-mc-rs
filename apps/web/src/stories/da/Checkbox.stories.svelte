<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Checkbox',
    component: Checkbox,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      checked: { control: 'boolean' },
      label: { control: 'text' },
      disabled: { control: 'boolean' }
    },
    args: { checked: false, label: 'Accepter les conditions', disabled: false }
  });
</script>

<Story name="Unchecked" args={{ label: 'Accepter les conditions', checked: false }} />
<Story name="Checked" args={{ label: 'Notifications activées', checked: true }} />
<Story name="Disabled" args={{ label: 'Option indisponible', disabled: true }} />
<Story name="Disabled Checked" args={{ label: 'Toujours activé', checked: true, disabled: true }} />

<Story name="Group">
  <div style="display: flex; flex-direction: column; gap: 0.5rem;">
    <Checkbox label="Java Edition" checked={true} />
    <Checkbox label="Bedrock Edition" />
    <Checkbox label="Pocket Edition" disabled />
  </div>
</Story>
