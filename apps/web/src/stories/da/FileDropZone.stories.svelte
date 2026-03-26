<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import FileDropZone from '$lib/components/ui/FileDropZone.svelte';

  const { Story } = defineMeta({
    title: 'DA/Inputs/FileDropZone',
    component: FileDropZone,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      accept: { control: 'text' },
      label: { control: 'text' },
      multiple: { control: 'boolean' },
    },
    args: {
      accept: 'image/png',
      label: 'Glisser un fichier ici ou cliquer pour parcourir',
      multiple: false,
    },
  });
</script>

<Story name="Default" />

<Story name="Custom Label" args={{ label: 'Importer un skin Minecraft (.png)' }} />

<Story name="Multiple" args={{ label: 'Glisser plusieurs fichiers ici', multiple: true }} />
