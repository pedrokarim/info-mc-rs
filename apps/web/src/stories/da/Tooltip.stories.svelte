<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Tooltip from '$lib/components/ui/Tooltip.svelte';
  import GameButton from '$lib/components/ui/GameButton.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';

  const { Story } = defineMeta({
    title: 'DA/Overlay/Tooltip',
    component: Tooltip,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      text: { control: 'text' },
      position: {
        control: { type: 'inline-radio' },
        options: ['top', 'bottom', 'left', 'right']
      },
      delay: { control: { type: 'number', min: 0, max: 1000, step: 50 } }
    },
    args: { text: 'Copier l\'adresse', position: 'top', delay: 200 }
  });
</script>

<Story name="Top">
  <Tooltip text="Copier l'adresse" position="top">
    <GameButton label="play.hypixel.net" variant="primary" href="#" />
  </Tooltip>
</Story>

<Story name="Bottom">
  <Tooltip text="Version 1.21" position="bottom">
    <Badge label="Java" variant="info" />
  </Tooltip>
</Story>

<Story name="Left">
  <Tooltip text="Cliquer pour voir" position="left">
    <GameButton label="Voir le statut" variant="secondary" href="#" />
  </Tooltip>
</Story>

<Story name="Right">
  <Tooltip text="16 joueurs en ligne" position="right">
    <Badge label="Online" variant="success" />
  </Tooltip>
</Story>

<Story name="All Positions">
  <div style="display: flex; gap: 3rem; align-items: center; padding: 2rem;">
    <Tooltip text="Top" position="top"><span style="cursor: help; text-decoration: underline;">Top</span></Tooltip>
    <Tooltip text="Bottom" position="bottom"><span style="cursor: help; text-decoration: underline;">Bottom</span></Tooltip>
    <Tooltip text="Left" position="left"><span style="cursor: help; text-decoration: underline;">Left</span></Tooltip>
    <Tooltip text="Right" position="right"><span style="cursor: help; text-decoration: underline;">Right</span></Tooltip>
  </div>
</Story>
