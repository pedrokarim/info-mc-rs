<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Textarea from '$lib/components/ui/Textarea.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Textarea',
    component: Textarea,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      value: { control: 'text' },
      label: { control: 'text' },
      placeholder: { control: 'text' },
      rows: { control: { type: 'number', min: 2, max: 20 } },
      maxlength: { control: 'number' },
      disabled: { control: 'boolean' }
    },
    args: { label: 'Description', placeholder: 'Décrivez le serveur...', rows: 4 }
  });
</script>

<Story name="Default" args={{ label: 'Description', placeholder: 'Décrivez le serveur...' }} />
<Story name="With Max Length" args={{ label: 'MOTD personnalisé', placeholder: 'Bienvenue sur...', maxlength: 120 }} />
<Story name="Prefilled" args={{ label: 'Notes', value: 'Serveur de test avec plugins custom.\nVersion 1.21.1', rows: 3 }} />
<Story name="Disabled" args={{ label: 'Lecture seule', value: 'Contenu non modifiable', disabled: true }} />
