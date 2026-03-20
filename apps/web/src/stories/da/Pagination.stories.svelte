<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Pagination from '$lib/components/ui/Pagination.svelte';

  const { Story } = defineMeta({
    title: 'DA/Navigation/Pagination',
    component: Pagination,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      current: { control: 'number' },
      total: { control: 'number' },
      perPage: { control: 'number' }
    },
    args: { current: 1, total: 200, perPage: 20 }
  });
</script>

<Story name="First Page" args={{ current: 1, total: 200, perPage: 20 }} />
<Story name="Middle" args={{ current: 5, total: 200, perPage: 20 }} />
<Story name="Last Page" args={{ current: 10, total: 200, perPage: 20 }} />
<Story name="Few Pages" args={{ current: 2, total: 60, perPage: 20 }} />
<Story name="Single Page" args={{ current: 1, total: 15, perPage: 20 }} />
