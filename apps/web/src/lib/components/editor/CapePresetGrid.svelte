<script lang="ts">
  import { onMount } from 'svelte';
  import { PRESETS, loadPreset } from '$lib/stores/cape-editor.svelte';

  let canvasRefs: Record<string, HTMLCanvasElement> = {};

  function capeScale(h: number): number {
    if (h % 22 === 0) return h / 22;
    if (h % 17 === 0) return h / 17;
    if (h >= 32 && (h & (h - 1)) === 0) return h / 32;
    return Math.max(1, Math.floor(h / 22));
  }

  function renderThumb(preset: { id: string; file: string }) {
    if (!preset.file) return;
    const canvas = canvasRefs[preset.id];
    if (!canvas) return;

    const img = new Image();
    img.onload = () => {
      const cs = capeScale(img.naturalHeight);
      const ctx = canvas.getContext('2d')!;
      ctx.imageSmoothingEnabled = false;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      // Crop front face: (cs, cs) → 10*cs × 16*cs
      ctx.drawImage(img, cs, cs, 10 * cs, 16 * cs, 0, 0, canvas.width, canvas.height);
    };
    img.src = `/images/skins/capes/${preset.file}`;
  }

  onMount(() => {
    for (const preset of PRESETS) {
      if (preset.file) renderThumb(preset);
    }
  });
</script>

<div class="preset-section">
  <span class="section-title">Presets ({PRESETS.length})</span>
  <div class="preset-grid">
    {#each PRESETS as preset}
      <button class="preset-card" onclick={() => loadPreset(preset.id)} title={preset.label}>
        {#if preset.file}
          <canvas
            bind:this={canvasRefs[preset.id]}
            width={50}
            height={80}
            class="preset-thumb"
          ></canvas>
        {:else}
          <div class="preset-thumb preset-blank"></div>
        {/if}
        <span class="preset-name">{preset.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .preset-section { display: flex; flex-direction: column; gap: 6px; }
  .section-title { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .preset-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px; max-height: 360px; overflow-y: auto; }
  .preset-card {
    display: flex; flex-direction: column; align-items: center; gap: 3px;
    padding: 4px; border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px); background: var(--surface-1, #edf5fa);
    cursor: pointer; transition: border-color var(--ease, 160ms ease), transform var(--ease, 160ms ease);
    font-family: inherit;
  }
  .preset-card:hover { border-color: var(--blue-0, #5e90ff); transform: translateY(-1px); }

  .preset-thumb {
    width: 50px; height: 80px; border-radius: 3px;
    image-rendering: pixelated;
    background: repeating-conic-gradient(#e0e0e0 0% 25%, #fff 0% 50%) 0 0 / 8px 8px;
  }
  .preset-blank {
    background: repeating-conic-gradient(#ccc 0% 25%, #fff 0% 50%) 0 0 / 8px 8px;
  }

  .preset-name { font-size: 0.55rem; font-weight: 600; color: var(--ink-1, #2d4a65); text-align: center; line-height: 1.15; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; width: 100%; }
</style>
