<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';

  const { Story } = defineMeta({
    title: 'DA/Layout/SectionHeading',
    component: SectionHeading,
    tags: ['autodocs'],
    parameters: { layout: 'fullscreen' },
    args: {
      title: 'Server Snapshot',
      description: 'Adresse: play.hypixel.net',
      light: true,
      level: 3
    }
  });
</script>

<Story name="Basic" />

<Story name="With Right Slot">
  <div style="max-width: min(1160px, calc(100% - 2rem)); margin: 0 auto; padding: 1rem 0;">
    <SectionHeading title="Server Snapshot" description="Adresse: play.hypixel.net">
      <StatusPill slot="right" label="Online" kind="online" />
    </SectionHeading>
  </div>
</Story>
