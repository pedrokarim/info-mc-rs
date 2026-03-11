<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/SearchInputRow',
    component: SearchInputRow,
    tags: ['autodocs'],
    parameters: { layout: 'fullscreen' },
    argTypes: {
      placeholder: { control: 'text' },
      actionLabel: { control: 'text' },
      actionVariant: {
        control: { type: 'inline-radio' },
        options: ['primary', 'secondary']
      },
      disabled: { control: 'boolean' }
    },
    args: {
      placeholder: 'play.hypixel.net ou 172.65.128.35:25565',
      actionLabel: 'Analyser',
      actionVariant: 'primary',
      disabled: false
    }
  });
</script>

<Story name="Primary Action">
  <form style="max-width: min(1160px, calc(100% - 2rem)); margin: 0 auto; padding: 1rem 0;">
    <SearchInputRow
      placeholder="play.hypixel.net ou 172.65.128.35:25565"
      actionLabel="Analyser"
      actionVariant="primary"
    />
  </form>
</Story>

<Story
  name="Secondary Action"
  args={{
    placeholder: 'Notch, Dream ou UUID',
    actionLabel: 'Voir profil',
    actionVariant: 'secondary'
  }}
>
  <form style="max-width: min(1160px, calc(100% - 2rem)); margin: 0 auto; padding: 1rem 0;">
    <SearchInputRow
      placeholder="Notch, Dream ou UUID"
      actionLabel="Voir profil"
      actionVariant="secondary"
    />
  </form>
</Story>
