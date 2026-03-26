<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import GradientPicker from '$lib/components/ui/GradientPicker.svelte';

  const { Story } = defineMeta({
    title: 'DA/Input/GradientPicker',
    component: GradientPicker,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      stops: ['#FF5555', '#5555FF'],
    },
  });
</script>

<Story name="Two Stops" />
<Story name="Three Stops" args={{ stops: ['#FF5555', '#55FF55', '#5555FF'] }} />
<Story name="Rainbow Style" args={{ stops: ['#FF5555', '#FFAA00', '#55FF55', '#55FFFF', '#5555FF', '#FF55FF'] }} />
