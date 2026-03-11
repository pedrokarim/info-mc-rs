<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import ProgressMeter from '$lib/components/ui/ProgressMeter.svelte';

  const { Story } = defineMeta({
    title: 'DA/Data/ProgressMeter',
    component: ProgressMeter,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      value: { control: { type: 'range', min: 0, max: 100, step: 1 } },
      max: { control: { type: 'number', min: 1, step: 1 } }
    },
    args: {
      value: 72,
      max: 100
    }
  });
</script>

<Story name="Population" />
<Story name="Low" args={{ value: 14, max: 100 }} />
<Story name="High" args={{ value: 93, max: 100 }} />
