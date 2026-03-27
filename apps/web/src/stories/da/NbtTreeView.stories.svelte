<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import NbtTreeView from '$lib/components/ui/NbtTreeView.svelte';

  const { Story } = defineMeta({
    title: 'DA/Content/NbtTreeView',
    component: NbtTreeView,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
  });

  const sampleTag = {
    type: 'compound',
    name: 'root',
    value: '{4}',
    children: [
      { type: 'int', name: 'DataVersion', value: 3465 },
      { type: 'list', name: 'size', value: '[3]', children: [
        { type: 'int', name: '[0]', value: 5 },
        { type: 'int', name: '[1]', value: 4 },
        { type: 'int', name: '[2]', value: 5 },
      ]},
      { type: 'list', name: 'palette', value: '[3]', children: [
        { type: 'compound', name: '[0]', value: '{1}', children: [
          { type: 'string', name: 'Name', value: 'minecraft:stone' },
        ]},
        { type: 'compound', name: '[1]', value: '{1}', children: [
          { type: 'string', name: 'Name', value: 'minecraft:oak_planks' },
        ]},
        { type: 'compound', name: '[2]', value: '{1}', children: [
          { type: 'string', name: 'Name', value: 'minecraft:glass' },
        ]},
      ]},
      { type: 'list', name: 'blocks', value: '[12]', children: [
        { type: 'compound', name: '[0]', value: '{2}', children: [
          { type: 'list', name: 'pos', value: '[3]', children: [
            { type: 'int', name: '[0]', value: 0 },
            { type: 'int', name: '[1]', value: 0 },
            { type: 'int', name: '[2]', value: 0 },
          ]},
          { type: 'int', name: 'state', value: 0 },
        ]},
      ]},
    ],
  };
</script>

<Story name="Default">
  <div style="width: 400px; background: var(--surface-1, #edf5fa); padding: 12px; border-radius: 12px;">
    <NbtTreeView tag={sampleTag} />
  </div>
</Story>
