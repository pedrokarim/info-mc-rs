<script lang="ts">
  type Crumb = { label: string; href?: string };

  let {
    items,
    separator = '/',
  }: {
    items: Crumb[];
    separator?: string;
  } = $props();
</script>

<nav class="breadcrumb" aria-label="Breadcrumb">
  <ol>
    {#each items as item, i}
      <li>
        {#if item.href && i < items.length - 1}
          <a href={item.href}>{item.label}</a>
        {:else}
          <span class="current" aria-current={i === items.length - 1 ? 'page' : undefined}>{item.label}</span>
        {/if}
      </li>
      {#if i < items.length - 1}
        <li class="sep" aria-hidden="true">{separator}</li>
      {/if}
    {/each}
  </ol>
</nav>

<style>
  .breadcrumb ol {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    list-style: none;
    margin: 0;
    padding: 0;
    flex-wrap: wrap;
  }

  .breadcrumb a {
    font-size: 0.8rem;
    color: var(--blue-0, #5e90ff);
    text-decoration: none;
    transition: color 100ms;
  }
  .breadcrumb a:hover { text-decoration: underline; }

  .current {
    font-size: 0.8rem;
    color: var(--ink-1, #2d4a65);
    font-weight: 600;
  }

  .sep {
    font-size: 0.72rem;
    color: var(--ink-2, #5a7894);
  }
</style>
