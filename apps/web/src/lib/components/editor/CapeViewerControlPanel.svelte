<script lang="ts">
  import { capeState } from '$lib/stores/cape-editor.svelte';
</script>

<div class="cape-controls">
  <div class="section">
    <span class="section-title">Équipement</span>
    <div class="equip-toggle">
      <button class="equip-btn" class:active={capeState.backEquipment === 'cape'} onclick={() => capeState.backEquipment = 'cape'}>Cape</button>
      <button class="equip-btn" class:active={capeState.backEquipment === 'elytra'} onclick={() => capeState.backEquipment = 'elytra'}>Elytra</button>
    </div>
  </div>

  <div class="section">
    <span class="section-title">Affichage</span>
    <div class="options-list">
      <label class="option-row"><input type="checkbox" bind:checked={capeState.showBody} /><span>Personnage</span></label>
      <label class="option-row"><input type="checkbox" bind:checked={capeState.showGrid} /><span>Grille</span></label>
      <label class="option-row"><input type="checkbox" bind:checked={capeState.showFaceOverlay} /><span>Faces</span></label>
      <label class="option-row"><input type="checkbox" bind:checked={capeState.wireframeMode} /><span>Wireframe</span></label>
      <label class="option-row"><input type="checkbox" bind:checked={capeState.animationPaused} /><span>Pause animation</span></label>
      <button class="reset-btn" onclick={() => capeState.cameraResetTrigger++}>Reset caméra</button>
    </div>
  </div>
</div>

<style>
  .cape-controls { display: flex; flex-direction: column; gap: 12px; }
  .section { display: flex; flex-direction: column; gap: 6px; }
  .section-title { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .equip-toggle { display: flex; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px); overflow: hidden; }
  .equip-btn { flex: 1; padding: 6px 10px; border: none; background: transparent; color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif; font-size: 0.75rem; font-weight: 600; cursor: pointer; transition: background var(--ease, 160ms ease), color var(--ease, 160ms ease); }
  .equip-btn:hover { background: rgba(94, 144, 255, 0.06); }
  .equip-btn.active { background: var(--blue-0, #5e90ff); color: #fff; }
  .options-list { display: flex; flex-direction: column; gap: 4px; }
  .option-row { display: flex; align-items: center; gap: 6px; font-size: 0.72rem; font-weight: 500; color: var(--ink-1, #2d4a65); cursor: pointer; }
  .option-row input[type="checkbox"] { width: 16px; height: 16px; accent-color: var(--blue-0, #5e90ff); cursor: pointer; }
  .reset-btn { padding: 5px 10px; border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: 6px; background: transparent; color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif; font-size: 0.72rem; font-weight: 600; cursor: pointer; align-self: flex-start; transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease); }
  .reset-btn:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
</style>
