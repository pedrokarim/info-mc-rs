<script lang="ts">
  import { editorState } from '$lib/stores/skin-editor.svelte';
  import type { Layer } from '$lib/stores/skin-editor.svelte';
  import { BODY_PARTS } from '$lib/utils/skin-uv-regions';
</script>

<div class="viewer-controls">
  <!-- Layer toggle -->
  <div class="section">
    <span class="section-title">Calque actif</span>
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
  </div>

  <!-- Per-part visibility -->
  <div class="section">
    <span class="section-title">Visibilité</span>
    <div class="visibility-grid">
      <div class="vis-header">
        <span></span>
        <span class="vis-col-label">B</span>
        <span class="vis-col-label">O</span>
      </div>
      {#each BODY_PARTS as part}
        <div class="vis-row">
          <div class="vis-label">
            <span class="vis-dot" style="background: {part.color}"></span>
            <span>{part.label}</span>
          </div>
          <input
            type="checkbox"
            class="vis-check"
            bind:checked={editorState.partVisibility[part.id].base}
            title="Base {part.label}"
          />
          <input
            type="checkbox"
            class="vis-check"
            bind:checked={editorState.partVisibility[part.id].overlay}
            title="Overlay {part.label}"
          />
        </div>
      {/each}
    </div>
  </div>

  <!-- 3D controls -->
  <div class="section">
    <span class="section-title">Affichage</span>
    <div class="options-list">
      <label class="option-row">
        <input type="checkbox" bind:checked={editorState.showGrid} />
        <span>Grille</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={editorState.showBodyPartOverlay} />
        <span>Zones</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={editorState.wireframeMode} />
        <span>Wireframe</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={editorState.animationPaused} />
        <span>Pause animation</span>
      </label>
      <button
        class="reset-btn"
        onclick={() => editorState.cameraResetTrigger++}
      >Reset caméra</button>
    </div>
  </div>
</div>

<style>
  .viewer-controls { display: flex; flex-direction: column; gap: 12px; }

  .section { display: flex; flex-direction: column; gap: 6px; }
  .section-title {
    font-size: 0.65rem; font-weight: 700; text-transform: uppercase;
    letter-spacing: 0.06em; color: var(--ink-2, #5a7894);
  }

  .layer-toggle {
    display: flex; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px); overflow: hidden;
  }
  .layer-btn {
    flex: 1; padding: 6px 10px; border: none; background: transparent;
    color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif;
    font-size: 0.75rem; font-weight: 600; cursor: pointer;
    transition: background var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .layer-btn:hover { background: rgba(94, 144, 255, 0.06); }
  .layer-btn.active { background: var(--blue-0, #5e90ff); color: #fff; }

  .visibility-grid { display: flex; flex-direction: column; gap: 2px; }
  .vis-header {
    display: grid; grid-template-columns: 1fr 24px 24px; gap: 4px;
    align-items: center; padding: 0 0 2px;
  }
  .vis-col-label {
    font-size: 0.6rem; font-weight: 700; text-align: center;
    color: var(--ink-2, #5a7894);
  }
  .vis-row {
    display: grid; grid-template-columns: 1fr 24px 24px; gap: 4px;
    align-items: center; padding: 2px 0;
  }
  .vis-label {
    display: flex; align-items: center; gap: 6px;
    font-size: 0.72rem; font-weight: 500; color: var(--ink-1, #2d4a65);
  }
  .vis-dot {
    width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0;
  }
  .vis-check {
    width: 16px; height: 16px; accent-color: var(--blue-0, #5e90ff);
    cursor: pointer; justify-self: center;
  }

  .options-list { display: flex; flex-direction: column; gap: 4px; }
  .option-row {
    display: flex; align-items: center; gap: 6px;
    font-size: 0.72rem; font-weight: 500; color: var(--ink-1, #2d4a65); cursor: pointer;
  }
  .option-row input[type="checkbox"] { width: 16px; height: 16px; accent-color: var(--blue-0, #5e90ff); cursor: pointer; }

  .reset-btn {
    padding: 5px 10px; border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 6px; background: transparent; color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif; font-size: 0.72rem; font-weight: 600;
    cursor: pointer; transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
    align-self: flex-start;
  }
  .reset-btn:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
</style>
