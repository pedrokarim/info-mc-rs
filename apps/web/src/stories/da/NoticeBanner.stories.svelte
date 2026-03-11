<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import NoticeBanner from '$lib/components/ui/NoticeBanner.svelte';

  const { Story } = defineMeta({
    title: 'DA/Feedback/NoticeBanner',
    component: NoticeBanner,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      message: { control: 'text' },
      kind: {
        control: { type: 'inline-radio' },
        options: ['error', 'info']
      }
    },
    args: {
      message: 'Impossible de joindre le serveur. Vérifie l\'adresse.',
      kind: 'error'
    }
  });
</script>

<Story name="Error" />
<Story name="Info" args={{ message: 'Aucune donnée live, affichage de fallback.', kind: 'info' }} />
