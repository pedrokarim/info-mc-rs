<script lang="ts">
  import SkinViewer3D from './SkinViewer3D.svelte';
  import Badge from './Badge.svelte';
  import GameButton from './GameButton.svelte';
  import { RARITY_CONFIG } from '$lib/data/capes-catalog';
  import type { CapeInfo } from '$lib/data/capes-catalog';

  let {
    cape = $bindable<CapeInfo | null>(null),
  }: {
    cape?: CapeInfo | null;
  } = $props();

  let backEquipment = $state<'cape' | 'elytra'>('cape');

  const open = $derived(cape !== null);
  const rarityInfo = $derived(cape ? RARITY_CONFIG[cape.rarity] : null);
  const capeUrl = $derived(cape ? `/images/skins/capes/${cape.file}` : undefined);
  const accentColor = $derived(
    rarityInfo?.variant === 'danger' ? '#e74c3c' :
    rarityInfo?.variant === 'warning' ? '#f39c12' :
    rarityInfo?.variant === 'info' ? '#5e90ff' : '#8899aa'
  );

  function close() { cape = null; }
  function onBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) close();
  }
  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if open && cape}
  <div class="backdrop" onclick={onBackdropClick} role="dialog" aria-modal="true" aria-label={cape.name}>
    <div class="detail-card" style="--accent: {accentColor}">
      <button class="close-btn" onclick={close} aria-label="Fermer">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M18 6L6 18M6 6l12 12" /></svg>
      </button>

      <div class="detail-glow"></div>

      <div class="detail-layout">
        <!-- 3D Viewer -->
        <div class="viewer-col">
          <div class="viewer-wrap">
            <SkinViewer3D
              skinUrl="/images/skins/steve.png"
              capeUrl={capeUrl}
              backEquipment={backEquipment}
              width={300}
              height={380}
              autoRotate={false}
              paused={true}
            />
          </div>
          <div class="equip-toggle">
            <button class="equip-btn" class:active={backEquipment === 'cape'} onclick={() => backEquipment = 'cape'}>Cape</button>
            <button class="equip-btn" class:active={backEquipment === 'elytra'} onclick={() => backEquipment = 'elytra'}>Elytra</button>
          </div>
        </div>

        <!-- Info -->
        <div class="info-col">
          <div class="info-header">
            <h2 class="detail-name">{cape.name}</h2>
            {#if rarityInfo}
              <Badge label={rarityInfo.label} variant={rarityInfo.variant} />
            {/if}
          </div>

          <span class="detail-year">{cape.year}</span>

          <div class="detail-meta">
            <span class="meta-item">
              <span class="meta-label">Catégorie</span>
              <span class="meta-value">{cape.category === 'event' ? 'Événement' : cape.category === 'mojang' ? 'Mojang' : cape.category === 'community' ? 'Communauté' : 'Spécial'}</span>
            </span>
            <span class="meta-item">
              <span class="meta-label">Texture</span>
              <span class="meta-value">64×32 PNG</span>
            </span>
          </div>

          <p class="detail-desc">{cape.descriptionLong}</p>

          <div class="detail-actions">
            <GameButton
              label="Éditer cette cape"
              variant="primary"
              compact
              href="/tools/cape-editor"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(5, 6, 8, 0.6);
    backdrop-filter: blur(8px);
    padding: 2rem;
    animation: fade-in 200ms ease;
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes card-in {
    from { opacity: 0; transform: scale(0.92) translateY(20px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }

  .detail-card {
    position: relative;
    max-width: 720px;
    width: 100%;
    background: var(--surface-1, #edf5fa);
    border: 2px solid var(--accent);
    border-radius: var(--radius-xl, 24px);
    overflow: hidden;
    box-shadow: 0 0 60px rgba(0, 0, 0, 0.3), 0 0 0 1px var(--accent);
    animation: card-in 300ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .close-btn {
    position: absolute;
    top: 12px;
    right: 12px;
    z-index: 2;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.1);
    color: var(--ink-1, #2d4a65);
    cursor: pointer;
    transition: background 160ms ease, color 160ms ease;
    padding: 0;
  }
  .close-btn:hover { background: rgba(0, 0, 0, 0.2); color: var(--ink-0); }

  .detail-glow {
    position: absolute;
    top: -50%;
    left: -25%;
    width: 150%;
    height: 100%;
    background: radial-gradient(ellipse at center, var(--accent), transparent 70%);
    opacity: 0.06;
    pointer-events: none;
  }

  .detail-layout {
    position: relative;
    display: flex;
    gap: 24px;
    padding: 24px;
  }

  .viewer-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  .viewer-wrap {
    border-radius: var(--radius-md, 12px);
    overflow: hidden;
    background: linear-gradient(135deg, rgba(94,144,255,0.06) 0%, transparent 100%);
  }

  .equip-toggle {
    display: flex;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px);
    overflow: hidden;
  }
  .equip-btn {
    padding: 5px 16px;
    border: none;
    background: transparent;
    color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 160ms ease, color 160ms ease;
  }
  .equip-btn:hover { background: rgba(94, 144, 255, 0.06); }
  .equip-btn.active { background: var(--blue-0, #5e90ff); color: #fff; }

  .info-col {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
    min-width: 0;
    padding-top: 8px;
  }

  .info-header {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }

  .detail-name {
    font-family: 'Teko', sans-serif;
    font-size: 2rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1;
  }

  .detail-year {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--accent);
  }

  .detail-meta {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .meta-label {
    font-size: 0.6rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }
  .meta-value {
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
  }

  .detail-desc {
    font-size: 0.82rem;
    color: var(--ink-1, #2d4a65);
    line-height: 1.6;
    margin: 4px 0 0;
  }

  .detail-actions {
    margin-top: auto;
    padding-top: 8px;
  }

  @media (max-width: 640px) {
    .detail-layout { flex-direction: column; padding: 16px; gap: 16px; }
    .viewer-col { align-self: center; }
    .detail-name { font-size: 1.6rem; }
  }
</style>
