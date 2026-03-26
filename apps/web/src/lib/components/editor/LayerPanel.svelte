<script lang="ts">
  import { editorState } from '$lib/stores/skin-editor.svelte';
  import type { Layer } from '$lib/stores/skin-editor.svelte';
</script>

<div class="layer-panel">
  <span class="panel-title">Calques</span>

  <div class="layer-toggle">
    <button
      class="layer-btn"
      class:active={editorState.activeLayer === 'base'}
      onclick={() => editorState.activeLayer = 'base'}
    >Base</button>
    <button
      class="layer-btn"
      class:active={editorState.activeLayer === 'overlay'}
      onclick={() => editorState.activeLayer = 'overlay'}
    >Overlay</button>
  </div>

  <div class="layer-options">
    <label class="option-row">
      <input type="checkbox" bind:checked={editorState.showGrid} />
      <span>Grille</span>
    </label>
    <label class="option-row">
      <input type="checkbox" bind:checked={editorState.showBodyPartOverlay} />
      <span>Zones</span>
    </label>
  </div>
</div>

<style>
  .layer-panel { display: flex; flex-direction: column; gap: 8px; }
  .panel-title { font-size: 0.7rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  .layer-toggle { display: flex; gap: 0; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px); overflow: hidden; }
  .layer-btn {
    flex: 1; padding: 6px 10px; border: none; background: transparent; color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif; font-size: 0.75rem; font-weight: 600; cursor: pointer;
    transition: background var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .layer-btn:hover { background: rgba(94, 144, 255, 0.06); }
  .layer-btn.active { background: var(--blue-0, #5e90ff); color: #fff; }

  .layer-options { display: flex; flex-direction: column; gap: 4px; }
  .option-row { display: flex; align-items: center; gap: 6px; font-size: 0.75rem; font-weight: 500; color: var(--ink-1, #2d4a65); cursor: pointer; }
  .option-row input[type="checkbox"] { width: 16px; height: 16px; accent-color: var(--blue-0, #5e90ff); cursor: pointer; }
</style>
