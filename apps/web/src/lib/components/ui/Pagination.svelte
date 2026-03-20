<script lang="ts">
  let {
    current = $bindable(1),
    total,
    perPage = 20,
    onchange,
  }: {
    current?: number;
    total: number;
    perPage?: number;
    onchange?: (page: number) => void;
  } = $props();

  const totalPages = $derived(Math.max(1, Math.ceil(total / perPage)));

  const pages = $derived.by(() => {
    const p: (number | '...')[] = [];
    const tp = totalPages;
    const c = current;
    if (tp <= 7) {
      for (let i = 1; i <= tp; i++) p.push(i);
    } else {
      p.push(1);
      if (c > 3) p.push('...');
      const start = Math.max(2, c - 1);
      const end = Math.min(tp - 1, c + 1);
      for (let i = start; i <= end; i++) p.push(i);
      if (c < tp - 2) p.push('...');
      p.push(tp);
    }
    return p;
  });

  function go(page: number) {
    if (page < 1 || page > totalPages || page === current) return;
    current = page;
    onchange?.(page);
  }
</script>

<nav class="pagination" aria-label="Pagination">
  <button class="pg-btn" disabled={current <= 1} onclick={() => go(current - 1)} aria-label="Page précédente">←</button>

  {#each pages as p}
    {#if p === '...'}
      <span class="pg-dots">…</span>
    {:else}
      <button
        class="pg-btn"
        class:active={p === current}
        onclick={() => go(p)}
        aria-current={p === current ? 'page' : undefined}
      >{p}</button>
    {/if}
  {/each}

  <button class="pg-btn" disabled={current >= totalPages} onclick={() => go(current + 1)} aria-label="Page suivante">→</button>
</nav>

<style>
  .pagination {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .pg-btn {
    min-width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid rgba(84, 126, 181, 0.25);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.5);
    color: var(--ink-1, #2d4a65);
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 100ms, color 100ms, border-color 100ms;
    padding: 0 0.4rem;
  }

  .pg-btn:hover:not(:disabled):not(.active) {
    background: rgba(94, 144, 255, 0.1);
    border-color: rgba(94, 144, 255, 0.35);
  }

  .pg-btn.active {
    background: var(--blue-0, #5e90ff);
    color: #fff;
    border-color: var(--blue-1, #345fcd);
  }

  .pg-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .pg-dots {
    font-size: 0.8rem;
    color: var(--ink-2, #5a7894);
    padding: 0 0.2rem;
  }
</style>
