<script lang="ts">
  import { hexToRgb, rgbToHex } from '$lib/utils/motd';

  let {
    stops = $bindable(['#FF5555', '#5555FF']),
    onchange,
    onapply,
    onrainbow,
  }: {
    stops?: string[];
    onchange?: (stops: string[]) => void;
    onapply?: (stops: string[]) => void;
    onrainbow?: () => void;
  } = $props();

  let editingIdx = $state<number | null>(null);

  const gradientCss = $derived(
    `linear-gradient(90deg, ${stops.join(', ')})`,
  );

  function updateStop(idx: number, hex: string) {
    stops[idx] = hex.toUpperCase();
    stops = [...stops];
    onchange?.(stops);
  }

  function addStop() {
    if (stops.length >= 6) return;
    // Interpolate midpoint of last two
    const a = hexToRgb(stops[stops.length - 2]);
    const b = hexToRgb(stops[stops.length - 1]);
    const mid = rgbToHex(
      (a[0] + b[0]) / 2,
      (a[1] + b[1]) / 2,
      (a[2] + b[2]) / 2,
    );
    stops = [...stops.slice(0, -1), mid, stops[stops.length - 1]];
    onchange?.(stops);
  }

  function removeStop(idx: number) {
    if (stops.length <= 2) return;
    stops = stops.filter((_, i) => i !== idx);
    if (editingIdx === idx) editingIdx = null;
    onchange?.(stops);
  }
</script>

<div class="gradient-picker">
  <span class="gp-label">Dégradé / Rainbow</span>

  <!-- Gradient preview bar -->
  <div class="gradient-bar" style="background: {gradientCss}"></div>

  <!-- Color stops -->
  <div class="stops-row">
    {#each stops as stop, idx (idx)}
      <div class="stop-item">
        <input
          type="color"
          class="stop-input"
          value={stop}
          oninput={(e) => updateStop(idx, (e.target as HTMLInputElement).value)}
          title="Stop {idx + 1}"
        />
        {#if stops.length > 2}
          <button class="stop-remove" onclick={() => removeStop(idx)} aria-label="Supprimer le stop {idx + 1}">
            &times;
          </button>
        {/if}
      </div>
    {/each}

    {#if stops.length < 6}
      <button class="stop-add" onclick={addStop} aria-label="Ajouter un stop">
        +
      </button>
    {/if}
  </div>

  <!-- Action buttons -->
  <div class="actions-row">
    <button class="gp-btn apply-btn" onclick={() => onapply?.(stops)}>
      Appliquer le dégradé
    </button>
    <button class="gp-btn rainbow-btn" onclick={() => onrainbow?.()}>
      Rainbow
    </button>
  </div>
</div>

<style>
  .gradient-picker {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .gp-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .gradient-bar {
    height: 20px;
    border-radius: 6px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }

  .stops-row {
    display: flex;
    gap: 6px;
    align-items: center;
    flex-wrap: wrap;
  }

  .stop-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .stop-input {
    width: 28px;
    height: 28px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 4px;
    padding: 0;
    cursor: pointer;
    background: none;
  }

  .stop-input::-webkit-color-swatch-wrapper {
    padding: 1px;
  }

  .stop-input::-webkit-color-swatch {
    border: none;
    border-radius: 2px;
  }

  .stop-remove {
    position: absolute;
    top: -6px;
    right: -6px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: none;
    background: var(--danger, #b83b3b);
    color: #fff;
    font-size: 0.6rem;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    opacity: 0;
    transition: opacity 160ms ease;
  }

  .stop-item:hover .stop-remove {
    opacity: 1;
  }

  .stop-add {
    width: 28px;
    height: 28px;
    border: 2px dashed var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 4px;
    background: none;
    color: var(--ink-2, #5a7894);
    font-size: 1rem;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: border-color 160ms ease, color 160ms ease;
  }

  .stop-add:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  .actions-row {
    display: flex;
    gap: 6px;
  }

  .gp-btn {
    flex: 1;
    padding: 6px 10px;
    border-radius: var(--radius-sm, 8px);
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.72rem;
    font-weight: 600;
    cursor: pointer;
    border: 2px solid;
    transition: background 160ms ease, border-color 160ms ease;
  }

  .apply-btn {
    background: var(--blue-0, #5e90ff);
    border-color: var(--blue-1, #345fcd);
    color: #fff;
  }

  .apply-btn:hover {
    background: var(--blue-1, #345fcd);
  }

  .rainbow-btn {
    background: linear-gradient(90deg, #ff5555, #ffaa00, #55ff55, #55ffff, #5555ff, #ff55ff);
    border-color: rgba(0, 0, 0, 0.2);
    color: #fff;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  }

  .rainbow-btn:hover {
    opacity: 0.9;
  }
</style>
