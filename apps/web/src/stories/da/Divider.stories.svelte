<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Divider from '$lib/components/ui/Divider.svelte';

  const { Story } = defineMeta({
    title: 'DA/Layout/Divider',
    component: Divider,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      label: { control: 'text' },
      direction: { control: { type: 'inline-radio' }, options: ['horizontal', 'vertical'] },
      spacing: { control: { type: 'inline-radio' }, options: ['sm', 'md', 'lg'] }
    },
    args: { direction: 'horizontal', spacing: 'md' }
  });
</script>

<Story name="Simple">
  <div style="width: 300px;">
    <p style="margin: 0; font-size: 0.85rem;">Section A</p>
    <Divider />
    <p style="margin: 0; font-size: 0.85rem;">Section B</p>
  </div>
</Story>

<Story name="With Label">
  <div style="width: 300px;">
    <p style="margin: 0; font-size: 0.85rem;">Java servers</p>
    <Divider label="ou" />
    <p style="margin: 0; font-size: 0.85rem;">Bedrock servers</p>
  </div>
</Story>

<Story name="Vertical">
  <div style="display: flex; align-items: center; height: 40px;">
    <span style="font-size: 0.85rem;">Joueurs</span>
    <Divider direction="vertical" />
    <span style="font-size: 0.85rem;">Serveurs</span>
    <Divider direction="vertical" />
    <span style="font-size: 0.85rem;">Skins</span>
  </div>
</Story>

<Story name="Spacings">
  <div style="width: 300px;">
    <p style="margin: 0; font-size: 0.8rem;">Petit</p>
    <Divider spacing="sm" />
    <p style="margin: 0; font-size: 0.8rem;">Moyen</p>
    <Divider spacing="md" />
    <p style="margin: 0; font-size: 0.8rem;">Grand</p>
    <Divider spacing="lg" />
    <p style="margin: 0; font-size: 0.8rem;">Fin</p>
  </div>
</Story>
