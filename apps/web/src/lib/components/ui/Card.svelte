<script lang="ts">
  import type { Snippet } from 'svelte';

  type Variant = 'default' | 'elevated' | 'outlined';
  type Padding = 'sm' | 'md' | 'lg';

  let {
    variant = 'default',
    padding = 'md',
    href = '',
    header,
    children,
    footer,
  }: {
    variant?: Variant;
    padding?: Padding;
    href?: string;
    header?: Snippet;
    children: Snippet;
    footer?: Snippet;
  } = $props();

  const paddings: Record<Padding, string> = { sm: '0.55rem', md: '0.82rem 0.88rem', lg: '1.2rem 1.3rem' };
</script>

{#if href}
  <a {href} class="card card--{variant}" style="padding: {paddings[padding]};">
    {#if header}
      <div class="card-header">{@render header()}</div>
    {/if}
    <div class="card-body">{@render children()}</div>
    {#if footer}
      <div class="card-footer">{@render footer()}</div>
    {/if}
  </a>
{:else}
  <div class="card card--{variant}" style="padding: {paddings[padding]};">
    {#if header}
      <div class="card-header">{@render header()}</div>
    {/if}
    <div class="card-body">{@render children()}</div>
    {#if footer}
      <div class="card-footer">{@render footer()}</div>
    {/if}
  </div>
{/if}

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border-radius: 10px;
    text-decoration: none;
    color: inherit;
    transition: transform 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
  }

  .card--default {
    border: 1px solid rgba(84, 126, 181, 0.22);
    background: rgba(255, 255, 255, 0.2);
  }

  .card--elevated {
    border: 1px solid rgba(84, 126, 181, 0.18);
    background: rgba(255, 255, 255, 0.35);
    box-shadow: var(--shadow-sm, 0 8px 18px rgba(16, 45, 72, 0.16));
  }

  .card--outlined {
    border: 1.5px solid rgba(84, 126, 181, 0.35);
    background: transparent;
  }

  a.card:hover {
    transform: translateY(-2px);
    border-color: rgba(84, 139, 206, 0.72);
    box-shadow: 0 10px 20px rgba(21, 61, 96, 0.15);
    background: rgba(255, 255, 255, 0.34);
  }

  .card-header {
    border-bottom: 1px solid rgba(84, 126, 181, 0.15);
    padding-bottom: 0.4rem;
  }

  .card-footer {
    border-top: 1px solid rgba(84, 126, 181, 0.15);
    padding-top: 0.4rem;
  }
</style>
