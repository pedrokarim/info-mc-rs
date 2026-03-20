<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Slider from '$lib/components/ui/Slider.svelte';

  const { Story } = defineMeta({
    title: 'DA/Forms/Slider',
    component: Slider,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      value: { control: { type: 'number' } },
      min: { control: 'number' },
      max: { control: 'number' },
      step: { control: 'number' },
      label: { control: 'text' },
      showValue: { control: 'boolean' },
      disabled: { control: 'boolean' }
    },
    args: { value: 50, min: 0, max: 100, step: 1, label: 'Volume', showValue: true }
  });
</script>

<Story name="Default">
  <div style="width: 280px;">
    <Slider label="Volume" value={50} />
  </div>
</Story>

<Story name="Rate Limit">
  <div style="width: 280px;">
    <Slider label="Requêtes / min" value={100} min={10} max={500} step={10} />
  </div>
</Story>

<Story name="Cache TTL">
  <div style="width: 280px;">
    <Slider label="TTL Cache (sec)" value={60} min={5} max={300} step={5} />
  </div>
</Story>

<Story name="Disabled">
  <div style="width: 280px;">
    <Slider label="Verrouillé" value={75} disabled />
  </div>
</Story>
