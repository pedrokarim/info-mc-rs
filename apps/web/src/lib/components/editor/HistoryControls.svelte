<script lang="ts">
  import { undo, redo, getHistoryLength, getRedoLength } from '$lib/stores/skin-editor.svelte';
</script>

<div class="history-controls">
  <button
    class="hist-btn"
    disabled={getHistoryLength() === 0}
    onclick={undo}
    title="Annuler (Ctrl+Z)"
    aria-label="Annuler"
  >
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 7v6h6" />
      <path d="M3 13A9 9 0 0 1 21 13a9 9 0 0 1-9 9 9 9 0 0 1-6.69-3" />
    </svg>
  </button>

  <button
    class="hist-btn"
    disabled={getRedoLength() === 0}
    onclick={redo}
    title="Rétablir (Ctrl+Shift+Z)"
    aria-label="Rétablir"
  >
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M21 7v6h-6" />
      <path d="M21 13A9 9 0 0 0 3 13a9 9 0 0 0 9 9 9 9 0 0 0 6.69-3" />
    </svg>
  </button>
</div>

<style>
  .history-controls { display: flex; gap: 4px; }

  .hist-btn {
    display: flex; align-items: center; justify-content: center; width: 36px; height: 36px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa); color: var(--ink-1, #2d4a65); cursor: pointer;
    transition: background var(--ease, 160ms ease), color var(--ease, 160ms ease), opacity var(--ease, 160ms ease);
    padding: 0; font-family: inherit;
  }
  .hist-btn:hover:not(:disabled) { background: rgba(94, 144, 255, 0.08); color: var(--blue-0, #5e90ff); border-color: var(--blue-0, #5e90ff); }
  .hist-btn:disabled { opacity: 0.35; cursor: default; }
  .hist-btn:focus-visible { outline: 2px solid rgba(94, 144, 255, 0.5); outline-offset: 1px; }
</style>
