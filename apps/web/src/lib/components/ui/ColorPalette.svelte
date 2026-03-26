<script lang="ts">
  import ColorSwatch from './ColorSwatch.svelte';

  let {
    selected = $bindable({ r: 0, g: 0, b: 0, a: 255 }),
    recentColors = [],
    onselect,
  }: {
    selected?: { r: number; g: number; b: number; a: number };
    recentColors?: { r: number; g: number; b: number; a: number }[];
    onselect?: (color: { r: number; g: number; b: number; a: number }) => void;
  } = $props();

  // Minecraft-inspired palette (wool/dye colors + skin tones + basics)
  const PALETTE: { r: number; g: number; b: number; a: number }[] = [
    // Row 1: Basics
    { r: 0, g: 0, b: 0, a: 255 },       // Black
    { r: 255, g: 255, b: 255, a: 255 },   // White
    { r: 157, g: 157, b: 157, a: 255 },   // Light gray
    { r: 71, g: 71, b: 71, a: 255 },      // Dark gray
    { r: 131, g: 84, b: 50, a: 255 },     // Brown
    { r: 79, g: 50, b: 31, a: 255 },      // Dark brown
    { r: 216, g: 187, b: 155, a: 255 },   // Beige/skin
    { r: 186, g: 133, b: 96, a: 255 },    // Tan/skin

    // Row 2: Warm colors
    { r: 176, g: 46, b: 38, a: 255 },     // Red (wool)
    { r: 249, g: 128, b: 29, a: 255 },    // Orange
    { r: 254, g: 216, b: 61, a: 255 },    // Yellow
    { r: 128, g: 199, b: 31, a: 255 },    // Lime
    { r: 94, g: 124, b: 22, a: 255 },     // Green
    { r: 22, g: 154, b: 96, a: 255 },     // Emerald
    { r: 58, g: 179, b: 218, a: 255 },    // Light blue
    { r: 60, g: 68, b: 170, a: 255 },     // Blue

    // Row 3: Cool & misc
    { r: 137, g: 50, b: 184, a: 255 },    // Purple
    { r: 199, g: 78, b: 189, a: 255 },    // Magenta
    { r: 243, g: 139, b: 170, a: 255 },   // Pink
    { r: 22, g: 156, b: 157, a: 255 },    // Cyan
    { r: 94, g: 144, b: 255, a: 255 },    // Bright blue
    { r: 235, g: 157, b: 42, a: 255 },    // Gold
    { r: 112, g: 2, b: 0, a: 255 },       // Dark red
    { r: 0, g: 0, b: 0, a: 0 },           // Transparent

    // Row 4: Extra skin/hair tones
    { r: 255, g: 219, b: 172, a: 255 },   // Light skin
    { r: 241, g: 194, b: 125, a: 255 },   // Medium light skin
    { r: 198, g: 134, b: 66, a: 255 },    // Medium skin
    { r: 141, g: 85, b: 36, a: 255 },     // Medium dark skin
    { r: 87, g: 47, b: 14, a: 255 },      // Dark skin
    { r: 60, g: 30, b: 10, a: 255 },      // Very dark skin
    { r: 195, g: 154, b: 64, a: 255 },    // Blonde hair
    { r: 44, g: 22, b: 8, a: 255 },       // Dark hair
  ];

  function isSelected(c: { r: number; g: number; b: number; a: number }): boolean {
    return c.r === selected.r && c.g === selected.g && c.b === selected.b && c.a === selected.a;
  }

  function pick(c: { r: number; g: number; b: number; a: number }) {
    selected = c;
    onselect?.(c);
  }
</script>

<div class="color-palette">
  <div class="palette-grid">
    {#each PALETTE as c}
      <ColorSwatch color={c} size="sm" selected={isSelected(c)} onclick={() => pick(c)} />
    {/each}
  </div>

  {#if recentColors.length > 0}
    <div class="recent-section">
      <span class="recent-label">Récents</span>
      <div class="recent-row">
        {#each recentColors.slice(0, 8) as c}
          <ColorSwatch color={c} size="sm" selected={isSelected(c)} onclick={() => pick(c)} />
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .color-palette {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .palette-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: 3px;
  }

  .recent-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .recent-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .recent-row {
    display: flex;
    gap: 3px;
  }
</style>
