<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import GameButton from '$lib/components/ui/GameButton.svelte';

  const { Story } = defineMeta({
    title: 'DA/Actions/GameButton',
    component: GameButton,
    tags: ['autodocs'],
    parameters: {
      layout: 'centered'
    },
    argTypes: {
      label: { control: 'text' },
      variant: {
        control: { type: 'inline-radio' },
        options: ['primary', 'secondary']
      },
      href: { control: 'text' },
      disabled: { control: 'boolean' },
      compact: { control: 'boolean' }
    },
    args: {
      label: 'Play.mcinfo-rs.gg',
      variant: 'primary',
      disabled: false,
      compact: false
    }
  });
</script>

<Story name="Primary CTA" args={{ label: 'Play.mcinfo-rs.gg', variant: 'primary' }} />

<Story name="Secondary CTA" args={{ label: 'Discord', variant: 'secondary' }} />

<Story name="Compact" args={{ label: 'Boutique', variant: 'primary', compact: true }} />

<Story name="Disabled" args={{ label: 'Indisponible', variant: 'secondary', disabled: true }} />

<Story name="Hero Pair">
  <div style="display: flex; align-items: center; gap: 0.6rem;">
    <GameButton label="Play.mcinfo-rs.gg" variant="primary" href="/server/play.mcinfo-rs.gg" />
    <GameButton label="Discord" variant="secondary" href="https://discord.gg/example" />
  </div>
</Story>
