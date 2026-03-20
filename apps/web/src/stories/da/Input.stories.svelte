<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Input from '$lib/components/ui/Input.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Input',
    component: Input,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      value: { control: 'text' },
      label: { control: 'text' },
      placeholder: { control: 'text' },
      type: { control: { type: 'inline-radio' }, options: ['text', 'password', 'email', 'number'] },
      variant: { control: { type: 'inline-radio' }, options: ['default', 'error', 'success'] },
      hint: { control: 'text' },
      disabled: { control: 'boolean' }
    },
    args: { value: '', label: 'Pseudo', placeholder: 'Ex: Notch', variant: 'default' }
  });
</script>

<Story name="Default" args={{ label: 'Pseudo', placeholder: 'Ex: Notch' }} />
<Story name="With Value" args={{ label: 'Adresse serveur', value: 'play.hypixel.net' }} />
<Story name="Error" args={{ label: 'Email', value: 'invalid', variant: 'error', hint: 'Email invalide' }} />
<Story name="Success" args={{ label: 'UUID', value: '069a79f4-44e9-4726-a5be-fca90e38aaf5', variant: 'success', hint: 'UUID valide' }} />
<Story name="Password" args={{ label: 'Mot de passe', type: 'password', placeholder: '••••••••' }} />
<Story name="Disabled" args={{ label: 'Serveur', value: 'locked.server.net', disabled: true }} />

<Story name="Form Group">
  <div style="display: flex; flex-direction: column; gap: 0.8rem; width: 300px;">
    <Input label="Nom d'utilisateur" placeholder="Ex: Notch" />
    <Input label="Adresse serveur" placeholder="Ex: play.hypixel.net" />
    <Input label="Port" type="number" placeholder="25565" />
  </div>
</Story>
