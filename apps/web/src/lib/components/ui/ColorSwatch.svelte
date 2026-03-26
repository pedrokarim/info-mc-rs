<script lang="ts">
  let {
    color = { r: 0, g: 0, b: 0, a: 255 },
    selected = false,
    size = 'md',
    onclick,
  }: {
    color?: { r: number; g: number; b: number; a: number };
    selected?: boolean;
    size?: 'sm' | 'md' | 'lg';
    onclick?: () => void;
  } = $props();

  const cssColor = $derived(
    `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a / 255})`
  );
</script>

<button
  class="swatch swatch--{size}"
  class:selected
  style="--swatch-color: {cssColor}"
  onclick={onclick}
  title="R:{color.r} G:{color.g} B:{color.b} A:{color.a}"
  aria-label="Couleur R:{color.r} G:{color.g} B:{color.b}"
>
  <span class="swatch-fill"></span>
</button>

<style>
  .swatch {
    position: relative;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
    overflow: hidden;
    transition: border-color var(--ease, 160ms ease), transform var(--ease, 160ms ease);
    /* Checkerboard background for alpha visibility */
    background-image:
      linear-gradient(45deg, #ccc 25%, transparent 25%),
      linear-gradient(-45deg, #ccc 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #ccc 75%),
      linear-gradient(-45deg, transparent 75%, #ccc 75%);
    background-size: 8px 8px;
    background-position: 0 0, 0 4px, 4px -4px, -4px 0;
  }

  .swatch--sm { width: 20px; height: 20px; }
  .swatch--md { width: 28px; height: 28px; }
  .swatch--lg { width: 36px; height: 36px; }

  .swatch-fill {
    display: block;
    width: 100%;
    height: 100%;
    background: var(--swatch-color);
  }

  .swatch:hover {
    border-color: var(--blue-0, #5e90ff);
    transform: scale(1.1);
  }

  .swatch.selected {
    border-color: var(--blue-0, #5e90ff);
    box-shadow: 0 0 0 2px var(--blue-0, #5e90ff);
  }

  .swatch:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 2px;
  }
</style>
