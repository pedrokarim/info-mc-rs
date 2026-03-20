<script lang="ts">
  import type { Snippet } from 'svelte';

  type AccordionItem = { label: string; value: string };

  let {
    items,
    multiple = false,
    children,
  }: {
    items: AccordionItem[];
    multiple?: boolean;
    children: Snippet<[{ value: string }]>;
  } = $props();

  let openItems = $state<Set<string>>(new Set());

  function toggle(value: string) {
    if (openItems.has(value)) {
      openItems.delete(value);
    } else {
      if (!multiple) openItems.clear();
      openItems.add(value);
    }
    openItems = new Set(openItems);
  }
</script>

<div class="accordion">
  {#each items as item}
    <div class="accordion-item" class:open={openItems.has(item.value)}>
      <button
        class="accordion-trigger"
        aria-expanded={openItems.has(item.value)}
        onclick={() => toggle(item.value)}
      >
        <span class="accordion-label">{item.label}</span>
        <span class="accordion-chevron">▾</span>
      </button>
      {#if openItems.has(item.value)}
        <div class="accordion-content">
          {@render children({ value: item.value })}
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .accordion {
    display: flex;
    flex-direction: column;
    border: 1px solid rgba(84, 126, 181, 0.22);
    border-radius: 8px;
    overflow: hidden;
  }

  .accordion-item + .accordion-item {
    border-top: 1px solid rgba(84, 126, 181, 0.15);
  }

  .accordion-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6rem 0.8rem;
    background: rgba(255, 255, 255, 0.15);
    border: none;
    cursor: pointer;
    transition: background 100ms;
    text-align: left;
  }

  .accordion-trigger:hover { background: rgba(255, 255, 255, 0.3); }

  .accordion-label {
    font-family: 'Teko', sans-serif;
    font-size: 1.05rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    line-height: 1;
  }

  .accordion-chevron {
    font-size: 0.8rem;
    color: var(--ink-2, #5a7894);
    transition: transform 200ms ease;
  }

  .accordion-item.open .accordion-chevron {
    transform: rotate(180deg);
  }

  .accordion-content {
    padding: 0.5rem 0.8rem 0.7rem;
    font-size: 0.85rem;
    color: var(--ink-1, #2d4a65);
    animation: acc-open 200ms ease;
  }

  @keyframes acc-open {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
