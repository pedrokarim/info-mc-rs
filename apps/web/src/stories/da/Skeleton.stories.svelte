<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Skeleton from '$lib/components/ui/Skeleton.svelte';

  const { Story } = defineMeta({
    title: 'DA/Feedback/Skeleton',
    component: Skeleton,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      width: { control: 'text' },
      height: { control: 'text' },
      rounded: { control: 'boolean' },
      circle: { control: 'boolean' }
    },
    args: { width: '200px', height: '1rem', rounded: false, circle: false }
  });
</script>

<Story name="Text Line" args={{ width: '200px', height: '1rem' }} />
<Story name="Heading" args={{ width: '300px', height: '1.8rem', rounded: true }} />
<Story name="Circle Avatar" args={{ width: '48px', height: '48px', circle: true }} />
<Story name="Card Block" args={{ width: '280px', height: '160px' }} />

<Story name="Player Card Skeleton">
  <div style="display: flex; flex-direction: column; gap: 0.5rem; width: 260px;">
    <Skeleton width="260px" height="120px" />
    <Skeleton width="140px" height="1.4rem" rounded />
    <Skeleton width="200px" height="0.9rem" />
    <div style="display: flex; gap: 0.5rem;">
      <Skeleton width="48px" height="48px" circle />
      <div style="display: flex; flex-direction: column; gap: 0.4rem; flex: 1;">
        <Skeleton width="100%" height="0.9rem" />
        <Skeleton width="60%" height="0.9rem" />
      </div>
    </div>
  </div>
</Story>
