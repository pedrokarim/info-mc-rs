<script lang="ts">
  import { onMount } from 'svelte';

  export let url: string;
  export let scale: number = 8; // display scale (10*scale × 16*scale px)

  let canvas: HTMLCanvasElement;
  let loaded = false;
  let error = false;

  function capeScale(h: number): number {
    if (h % 22 === 0) return h / 22;
    if (h % 17 === 0) return h / 17;
    if (h >= 32 && (h & (h - 1)) === 0) return h / 32;
    return Math.max(1, Math.floor(h / 22));
  }

  onMount(() => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => {
      const cs = capeScale(img.naturalHeight);
      const ctx = canvas.getContext('2d')!;
      ctx.imageSmoothingEnabled = false;
      // Crop front face of cape: pixel (cs, cs) → (cs+10*cs, cs+16*cs)
      ctx.drawImage(img, cs, cs, 10 * cs, 16 * cs, 0, 0, canvas.width, canvas.height);
      loaded = true;
    };
    img.onerror = () => { error = true; };
    img.src = url;
  });
</script>

<canvas
  bind:this={canvas}
  width={10 * scale}
  height={16 * scale}
  class="cape-canvas"
  class:hidden={error}
  title="Cape"
></canvas>

{#if error}
  <span class="cape-error">?</span>
{/if}

<style>
  .cape-canvas {
    image-rendering: pixelated;
    display: block;
  }
  .hidden { display: none; }
  .cape-error {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 128px;
    background: var(--surface-2, #1a1a2e);
    border-radius: 4px;
    color: var(--ink-2, #888);
    font-size: 1.5rem;
  }
</style>
