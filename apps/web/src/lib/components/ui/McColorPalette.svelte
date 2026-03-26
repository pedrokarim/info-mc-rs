<script lang="ts">
  import type { Edition } from '$lib/utils/motd';
  import { JAVA_COLORS, BEDROCK_MATERIAL_COLORS } from '$lib/utils/motd';
  import Tooltip from './Tooltip.svelte';

  let {
    edition = 'java',
    selected = null,
    onselect,
  }: {
    edition?: Edition;
    selected?: string | null;
    onselect?: (hex: string, code: string) => void;
  } = $props();

  const bedrockColors = $derived(
    edition === 'bedrock'
      ? [...JAVA_COLORS, ...BEDROCK_MATERIAL_COLORS]
      : JAVA_COLORS,
  );

  function isSelected(hex: string): boolean {
    return selected?.toUpperCase() === hex.toUpperCase();
  }
</script>

<div class="mc-palette">
  <span class="palette-label">Couleurs Minecraft</span>

  <div class="palette-grid" class:bedrock={edition === 'bedrock'}>
    {#each bedrockColors as color (color.code)}
      <Tooltip text="{color.name} (§{color.code})">
        <button
          class="mc-swatch"
          class:active={isSelected(color.hex)}
          style="--c: {color.hex}"
          onclick={() => onselect?.(color.hex, color.code)}
          aria-label="{color.name}"
        >
          <span class="code-label">{color.code}</span>
        </button>
      </Tooltip>
    {/each}
  </div>
</div>

<style>
  .mc-palette {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .palette-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .palette-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: 3px;
  }

  .mc-swatch {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    min-width: 24px;
    border: 2px solid rgba(0, 0, 0, 0.25);
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
    background: var(--c);
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      border-color 160ms ease,
      transform 160ms ease,
      box-shadow 160ms ease;
  }

  .code-label {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.6rem;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.9);
    text-shadow:
      0 0 3px rgba(0, 0, 0, 0.8),
      1px 1px 0 rgba(0, 0, 0, 0.5);
    pointer-events: none;
    line-height: 1;
  }

  .mc-swatch:hover {
    transform: scale(1.15);
    border-color: var(--blue-0, #5e90ff);
    z-index: 2;
  }

  .mc-swatch.active {
    border-color: var(--blue-0, #5e90ff);
    box-shadow: 0 0 0 2px var(--blue-0, #5e90ff);
    z-index: 1;
  }

  .mc-swatch:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 2px;
  }
</style>
