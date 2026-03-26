<script lang="ts">
  import CapePreview from './CapePreview.svelte';
  import Badge from './Badge.svelte';
  import { RARITY_CONFIG } from '$lib/data/capes-catalog';
  import type { CapeInfo } from '$lib/data/capes-catalog';

  let {
    cape,
    onclick,
  }: {
    cape: CapeInfo;
    onclick?: () => void;
  } = $props();

  const rarityInfo = $derived(RARITY_CONFIG[cape.rarity]);
  const capeUrl = $derived(`/images/skins/capes/${cape.file}`);

  // 3D tilt effect
  let cardEl: HTMLButtonElement;
  let tiltX = $state(0);
  let tiltY = $state(0);
  let isHovered = $state(false);

  function handleMouseMove(e: MouseEvent) {
    if (!cardEl) return;
    const rect = cardEl.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = (e.clientY - rect.top) / rect.height;
    tiltX = (y - 0.5) * -12;
    tiltY = (x - 0.5) * 12;
  }

  function handleMouseLeave() {
    tiltX = 0;
    tiltY = 0;
    isHovered = false;
  }
</script>

<button
  bind:this={cardEl}
  class="cape-card"
  class:hovered={isHovered}
  style="
    --tilt-x: {tiltX}deg;
    --tilt-y: {tiltY}deg;
    --accent: {rarityInfo.variant === 'danger' ? '#e74c3c' : rarityInfo.variant === 'warning' ? '#f39c12' : rarityInfo.variant === 'info' ? '#5e90ff' : '#8899aa'};
  "
  onclick={onclick}
  onmouseenter={() => isHovered = true}
  onmousemove={handleMouseMove}
  onmouseleave={handleMouseLeave}
>
  <div class="card-glow"></div>
  <div class="card-inner">
    <div class="cape-preview">
      <CapePreview url={capeUrl} scale={5} />
    </div>
    <div class="cape-info">
      <div class="cape-header">
        <h3 class="cape-name">{cape.name}</h3>
        <Badge label={rarityInfo.label} variant={rarityInfo.variant} size="sm" />
      </div>
      <span class="cape-year">{cape.year}</span>
      <p class="cape-desc">{cape.description}</p>
    </div>
  </div>
</button>

<style>
  .cape-card {
    position: relative;
    display: block;
    width: 100%;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-lg, 16px);
    background: var(--surface-1, #edf5fa);
    cursor: pointer;
    padding: 0;
    font-family: inherit;
    text-align: left;
    overflow: hidden;
    transform-style: preserve-3d;
    transform: perspective(800px) rotateX(var(--tilt-x)) rotateY(var(--tilt-y));
    transition: transform 200ms ease, border-color 200ms ease, box-shadow 200ms ease;
  }

  .cape-card:hover {
    border-color: var(--accent);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12), 0 0 0 1px var(--accent);
  }

  .cape-card:focus-visible {
    outline: 2px solid var(--blue-0, #5e90ff);
    outline-offset: 2px;
  }

  .card-glow {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    opacity: 0;
    background: radial-gradient(circle at 50% 0%, var(--accent), transparent 70%);
    transition: opacity 300ms ease;
    pointer-events: none;
  }
  .cape-card:hover .card-glow { opacity: 0.08; }

  .card-inner {
    position: relative;
    display: flex;
    flex-direction: column;
    transform: translateZ(10px);
  }

  .cape-preview {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 16px 12px 8px;
    min-height: 100px;
  }

  .cape-info {
    padding: 0 14px 14px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .cape-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }

  .cape-name {
    font-family: 'Teko', sans-serif;
    font-size: 1.15rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cape-year {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.65rem;
    font-weight: 600;
    color: var(--ink-2, #5a7894);
  }

  .cape-desc {
    font-size: 0.72rem;
    color: var(--ink-1, #2d4a65);
    line-height: 1.4;
    margin: 0;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
