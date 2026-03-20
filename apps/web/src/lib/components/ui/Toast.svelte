<script lang="ts">
  import { toasts, removeToast } from '$lib/stores/toasts';
</script>

{#if $toasts.length > 0}
  <div class="toast-container" aria-live="polite">
    {#each $toasts as toast (toast.id)}
      <div class="toast toast--{toast.variant}">
        <span class="toast-msg">{toast.message}</span>
        <button class="toast-close" onclick={() => removeToast(toast.id)} aria-label="Fermer">✕</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    bottom: 1.2rem;
    right: 1.2rem;
    z-index: 2000;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 360px;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 0.8rem;
    border-radius: 8px;
    font-size: 0.82rem;
    font-weight: 500;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
    animation: slide-in 250ms ease;
  }

  .toast--success {
    background: #0d3320;
    color: #3fb950;
    border: 1px solid rgba(63, 185, 80, 0.35);
  }
  .toast--error {
    background: #330d0d;
    color: #f85149;
    border: 1px solid rgba(248, 81, 73, 0.35);
  }
  .toast--warning {
    background: #332200;
    color: #d29922;
    border: 1px solid rgba(210, 153, 34, 0.35);
  }
  .toast--info {
    background: #0d1b33;
    color: #58a6ff;
    border: 1px solid rgba(88, 166, 255, 0.35);
  }

  .toast-msg { flex: 1; }

  .toast-close {
    background: none;
    border: none;
    color: inherit;
    opacity: 0.6;
    cursor: pointer;
    font-size: 0.78rem;
    padding: 0.1rem;
    line-height: 1;
  }
  .toast-close:hover { opacity: 1; }

  @keyframes slide-in {
    from { opacity: 0; transform: translateX(20px); }
    to { opacity: 1; transform: translateX(0); }
  }
</style>
