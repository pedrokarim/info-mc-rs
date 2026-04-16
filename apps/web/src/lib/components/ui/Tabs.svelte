<script lang="ts">
  type TabItem = { label: string; value: string };

  let {
    items,
    selected = $bindable(''),
    onchange,
  }: {
    items: TabItem[];
    selected?: string;
    onchange?: (value: string) => void;
  } = $props();

  if (!selected && items.length > 0) selected = items[0].value;

  function select(value: string) {
    selected = value;
    onchange?.(value);
  }

  function onKeydown(e: KeyboardEvent, idx: number) {
    let next = idx;
    if (e.key === 'ArrowRight') next = (idx + 1) % items.length;
    else if (e.key === 'ArrowLeft') next = (idx - 1 + items.length) % items.length;
    else return;
    e.preventDefault();
    select(items[next].value);
    (e.currentTarget as HTMLElement)?.parentElement?.querySelectorAll<HTMLButtonElement>('[role="tab"]')[next]?.focus();
  }
</script>

<div class="tabs" role="tablist">
  {#each items as item, i}
    <button
      class="tab"
      class:active={selected === item.value}
      role="tab"
      aria-selected={selected === item.value}
      tabindex={selected === item.value ? 0 : -1}
      onclick={() => select(item.value)}
      onkeydown={(e) => onKeydown(e, i)}
    >
      {item.label}
    </button>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    gap: 0;
    border-bottom: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: none;
  }

  .tabs::-webkit-scrollbar {
    display: none;
  }

  .tab {
    font-family: 'Teko', sans-serif;
    font-size: 1.1rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    color: var(--ink-2, #5a7894);
    background: none;
    border: none;
    padding: 0.5rem 1rem;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    margin-bottom: -2px;
    transition: color 140ms ease, border-color 140ms ease;
    white-space: nowrap;
  }

  .tab:hover {
    color: var(--ink-0, #0f253a);
  }

  .tab.active {
    color: var(--blue-0, #5e90ff);
    border-bottom-color: var(--blue-0, #5e90ff);
  }

  .tab:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.6);
    outline-offset: -2px;
    border-radius: 4px 4px 0 0;
  }
</style>
