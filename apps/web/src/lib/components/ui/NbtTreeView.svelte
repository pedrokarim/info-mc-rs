<script lang="ts">
  import type { NbtTag } from '$lib/utils/nbt-parser';
  import NbtTreeView from './NbtTreeView.svelte';

  let {
    tag,
    depth = 0,
  }: {
    tag: NbtTag;
    depth?: number;
  } = $props();

  let expanded = $state(depth < 2);

  const hasChildren = $derived(tag.children && tag.children.length > 0);

  const typeIcon: Record<string, string> = {
    compound: '{}',
    list: '[]',
    byte: 'b',
    short: 's',
    int: '#',
    long: 'L',
    float: 'f',
    double: 'd',
    string: '"',
    byteArray: '[B]',
    intArray: '[I]',
    longArray: '[L]',
  };

  const typeColor: Record<string, string> = {
    compound: 'var(--ink-2, #5a7894)',
    list: 'var(--ink-2, #5a7894)',
    byte: '#d4880a',
    short: '#d4880a',
    int: '#3080d0',
    long: '#3080d0',
    float: '#b040a0',
    double: '#b040a0',
    string: '#1a8a4a',
    byteArray: '#d4880a',
    intArray: '#3080d0',
    longArray: '#3080d0',
  };
</script>

<div class="nbt-node" style="padding-left: {depth * 14}px">
  <button
    class="node-header"
    class:expandable={hasChildren}
    onclick={() => { if (hasChildren) expanded = !expanded; }}
  >
    {#if hasChildren}
      <span class="arrow" class:open={expanded}>&#9654;</span>
    {:else}
      <span class="arrow-spacer"></span>
    {/if}

    <span class="type-badge" style="color: {typeColor[tag.type] ?? 'var(--ink-2)'}">
      {typeIcon[tag.type] ?? '?'}
    </span>

    <span class="node-name">{tag.name}</span>

    {#if !hasChildren}
      <span class="node-value" style="color: {typeColor[tag.type] ?? 'var(--ink-1)'}">
        {#if tag.type === 'string'}
          "{tag.value}"
        {:else}
          {tag.value}
        {/if}
      </span>
    {:else}
      <span class="node-count">{tag.value}</span>
    {/if}
  </button>

  {#if expanded && hasChildren}
    <div class="node-children">
      {#each tag.children! as child (child.name)}
        <NbtTreeView tag={child} depth={depth + 1} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .nbt-node { font-size: 0.78rem; }

  .node-header {
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    padding: 2px 4px;
    border: none;
    border-radius: 3px;
    background: transparent;
    color: var(--ink-0, #0f253a);
    font-family: 'JetBrains Mono', monospace;
    font-size: inherit;
    text-align: left;
    cursor: default;
    transition: background 100ms ease;
  }

  .node-header.expandable { cursor: pointer; }
  .node-header.expandable:hover { background: rgba(94, 144, 255, 0.06); }

  .arrow {
    display: inline-block;
    width: 12px;
    font-size: 0.55rem;
    color: var(--ink-2, #5a7894);
    transition: transform 120ms ease;
    flex-shrink: 0;
    text-align: center;
  }
  .arrow.open { transform: rotate(90deg); }
  .arrow-spacer { width: 12px; flex-shrink: 0; }

  .type-badge {
    font-size: 0.65rem;
    font-weight: 700;
    min-width: 18px;
    text-align: center;
    flex-shrink: 0;
  }

  .node-name {
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    white-space: nowrap;
  }

  .node-value {
    font-weight: 400;
    margin-left: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 300px;
  }

  .node-count {
    font-weight: 400;
    color: var(--ink-2, #5a7894);
    font-size: 0.7rem;
    margin-left: 2px;
  }

  .node-children {
    border-left: 1px solid var(--line-0, rgba(46, 94, 143, 0.2));
    margin-left: 6px;
  }
</style>
