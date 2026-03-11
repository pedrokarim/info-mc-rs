<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import GameChip from '$lib/components/ui/GameChip.svelte';

  const { Story } = defineMeta({
    title: 'DA/Actions/GameChip',
    component: GameChip,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      label: { control: 'text' },
      href: { control: 'text' },
      active: { control: 'boolean' },
      disabled: { control: 'boolean' }
    },
    args: {
      label: 'Notch',
      href: '/player/Notch',
      active: false,
      disabled: false
    }
  });
</script>

<Story name="Link" />

<Story name="Active Link" args={{ label: 'Hypixel', href: '/server/play.hypixel.net', active: true }} />

<Story name="Toggle Button" args={{ label: 'Java', href: undefined, active: true }} />

<Story name="Disabled" args={{ label: 'Offline', href: undefined, disabled: true }} />
