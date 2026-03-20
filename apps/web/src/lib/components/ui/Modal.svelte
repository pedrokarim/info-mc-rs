<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    open = $bindable(false),
    title = '',
    closable = true,
    onclose,
    children,
    footer,
  }: {
    open?: boolean;
    title?: string;
    closable?: boolean;
    onclose?: () => void;
    children: Snippet;
    footer?: Snippet;
  } = $props();

  function close() {
    if (!closable) return;
    open = false;
    onclose?.();
  }

  function onBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) close();
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="backdrop" onclick={onBackdrop} onkeydown={onKeydown} role="dialog" aria-modal="true" aria-label={title || 'Dialog'}>
    <div class="modal">
      {#if title || closable}
        <div class="modal-header">
          {#if title}
            <h3 class="modal-title">{title}</h3>
          {/if}
          {#if closable}
            <button class="modal-close" onclick={close} aria-label="Fermer">✕</button>
          {/if}
        </div>
      {/if}

      <div class="modal-body">
        {@render children()}
      </div>

      {#if footer}
        <div class="modal-footer">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    animation: fade-in 150ms ease;
  }

  .modal {
    background: var(--surface-0, #dbe8f1);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-md, 12px);
    box-shadow: var(--shadow-lg, 0 28px 70px rgba(0, 0, 0, 0.48));
    width: min(480px, calc(100vw - 2rem));
    max-height: calc(100vh - 4rem);
    display: flex;
    flex-direction: column;
    animation: modal-in 200ms ease;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.8rem 1rem;
    border-bottom: 1px solid rgba(84, 126, 181, 0.18);
  }

  .modal-title {
    margin: 0;
    font-family: 'Teko', sans-serif;
    font-size: 1.4rem;
    font-weight: 600;
    line-height: 1;
    color: var(--ink-0, #0f253a);
  }

  .modal-close {
    background: none;
    border: none;
    font-size: 1rem;
    color: var(--ink-2, #5a7894);
    cursor: pointer;
    padding: 0.2rem;
    border-radius: 4px;
    line-height: 1;
    transition: color 100ms;
  }
  .modal-close:hover { color: var(--danger, #b83b3b); }

  .modal-body {
    padding: 1rem;
    overflow-y: auto;
    font-size: 0.88rem;
    color: var(--ink-1, #2d4a65);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 0.7rem 1rem;
    border-top: 1px solid rgba(84, 126, 181, 0.18);
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes modal-in {
    from { opacity: 0; transform: scale(0.95) translateY(8px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }
</style>
