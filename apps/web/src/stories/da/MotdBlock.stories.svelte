<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';

  const { Story } = defineMeta({
    title: 'DA/Data/MotdBlock',
    component: MotdBlock,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      html: '<span style="color:#55ff55">Welcome</span> <span style="color:#ffffff">to</span> <span style="color:#ffaa00">MCInfo</span><br><span style="color:#9ab6d6">Survival · Quests · Economy</span>',
      fallback: 'No MOTD'
    }
  });
</script>

<Story name="Styled MOTD" />
<Story name="Fallback" args={{ html: '', fallback: 'No MOTD returned by endpoint' }} />
