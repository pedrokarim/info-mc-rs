<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Card from '$lib/components/ui/Card.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';

  const { Story } = defineMeta({
    title: 'DA/Layout/Card',
    component: Card,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      variant: {
        control: { type: 'inline-radio' },
        options: ['default', 'elevated', 'outlined']
      },
      padding: {
        control: { type: 'inline-radio' },
        options: ['sm', 'md', 'lg']
      },
      href: { control: 'text' }
    },
    args: { variant: 'default', padding: 'md' }
  });
</script>

<Story name="Default">
  <Card>
    <h4 style="margin: 0; font-family: Teko; font-size: 1.4rem; line-height: 0.95;">play.hypixel.net</h4>
    <p style="margin: 0; font-size: 0.82rem; color: #5a7894;">Le plus grand réseau Java</p>
  </Card>
</Story>

<Story name="Elevated">
  <Card variant="elevated" padding="lg">
    <h4 style="margin: 0; font-family: Teko; font-size: 1.4rem; line-height: 0.95;">play.cubecraft.net</h4>
    <p style="margin: 0; font-size: 0.82rem; color: #5a7894;">Mini-jeux multijoueur</p>
  </Card>
</Story>

<Story name="Outlined">
  <Card variant="outlined">
    <h4 style="margin: 0; font-family: Teko; font-size: 1.4rem; line-height: 0.95;">2b2t.org</h4>
    <p style="margin: 0; font-size: 0.82rem; color: #5a7894;">Le plus vieux serveur anarchy</p>
  </Card>
</Story>

<Story name="With Header & Footer">
  <Card variant="elevated" padding="md">
    {#snippet header()}
      <div style="display: flex; align-items: center; gap: 0.5rem;">
        <strong style="font-family: Teko; font-size: 1.2rem;">Hypixel</strong>
        <Badge label="Java" variant="info" size="sm" />
        <Badge label="Online" variant="success" size="sm" />
      </div>
    {/snippet}
    <p style="margin: 0; font-size: 0.82rem;">42,831 joueurs en ligne</p>
    {#snippet footer()}
      <span style="font-size: 0.72rem; color: #5a7894;">Dernière mise à jour il y a 2 min</span>
    {/snippet}
  </Card>
</Story>

<Story name="As Link">
  <Card href="/server/play.hypixel.net" variant="default">
    <h4 style="margin: 0; font-family: Teko; font-size: 1.4rem; line-height: 0.95;">play.hypixel.net →</h4>
    <p style="margin: 0; font-size: 0.82rem; color: #5a7894;">Cliquer pour voir le statut</p>
  </Card>
</Story>

<Story name="All Variants">
  <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
    <Card variant="default" padding="md">
      <strong style="font-family: Teko; font-size: 1.1rem;">Default</strong>
    </Card>
    <Card variant="elevated" padding="md">
      <strong style="font-family: Teko; font-size: 1.1rem;">Elevated</strong>
    </Card>
    <Card variant="outlined" padding="md">
      <strong style="font-family: Teko; font-size: 1.1rem;">Outlined</strong>
    </Card>
  </div>
</Story>
