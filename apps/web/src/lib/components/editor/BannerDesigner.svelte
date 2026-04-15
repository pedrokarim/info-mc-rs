<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte';
  import {
    DYE_COLORS, PATTERNS, renderBanner, toGiveCommand,
    type BannerState, type BannerLayer,
  } from '$lib/utils/banner';

  const MAX_LAYERS = 6;

  let baseColor = $state(0); // white
  let layers = $state<BannerLayer[]>([]);
  let selectedPattern = $state(PATTERNS[0].id);
  let selectedColor = $state(15); // black
  let copied = $state(false);

  let canvasEl: HTMLCanvasElement | undefined = $state();

  let bannerState: BannerState = $derived({ baseColor, layers });
  let command = $derived(toGiveCommand(bannerState));

  function addLayer() {
    if (layers.length >= MAX_LAYERS) return;
    layers = [...layers, { pattern: selectedPattern, color: selectedColor }];
  }

  function removeLayer(index: number) {
    layers = layers.filter((_, i) => i !== index);
  }

  function moveLayer(index: number, dir: -1 | 1) {
    const newIdx = index + dir;
    if (newIdx < 0 || newIdx >= layers.length) return;
    const copy = [...layers];
    [copy[index], copy[newIdx]] = [copy[newIdx], copy[index]];
    layers = copy;
  }

  function clearLayers() { layers = []; }

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(command);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch { /* fail */ }
  }

  // Canvas rendering
  $effect(() => {
    if (!canvasEl) return;
    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;
    const dpr = window.devicePixelRatio || 1;
    const w = canvasEl.clientWidth, h = canvasEl.clientHeight;
    canvasEl.width = w * dpr;
    canvasEl.height = h * dpr;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    // Background (checkerboard for transparency feel)
    ctx.fillStyle = '#2a2a2a';
    ctx.fillRect(0, 0, w, h);

    // Center the banner
    const bw = Math.min(w * 0.6, h * 0.45);
    const bh = bw * 2;
    const bx = (w - bw) / 2;
    const by = (h - bh) / 2;

    renderBanner(ctx, bannerState, bx, by, bw, bh);

    // Border
    ctx.strokeStyle = 'rgba(255,255,255,0.15)';
    ctx.lineWidth = 1;
    ctx.strokeRect(bx, by, bw, bh);
  });

  // Pattern mini-preview for selector
  function renderPatternPreview(canvas: HTMLCanvasElement, patternId: string) {
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    const w = canvas.width, h = canvas.height;
    ctx.fillStyle = '#e0e0e0';
    ctx.fillRect(0, 0, w, h);
    const pattern = PATTERNS.find(p => p.id === patternId);
    if (!pattern) return;
    ctx.fillStyle = '#333';
    const pw = 20, ph = 40;
    for (let py = 0; py < ph; py++) {
      for (let px = 0; px < pw; px++) {
        if (pattern.mask(px, py, pw, ph)) {
          ctx.fillRect(px * w / pw, py * h / ph, Math.ceil(w / pw), Math.ceil(h / ph));
        }
      }
    }
  }
</script>

<div class="banner-root">
  <div class="banner-layout">
    <!-- Left: Pattern selector + color -->
    <div class="banner-controls">
      <Card variant="elevated" padding="md">
        <span class="section-label">Couleur de base</span>
        <div class="color-grid">
          {#each DYE_COLORS as dye}
            <button
              class="color-swatch"
              class:active={baseColor === dye.id}
              style="background:{dye.hex}"
              title={dye.label}
              onclick={() => { baseColor = dye.id; }}
            ></button>
          {/each}
        </div>
      </Card>

      <Card variant="elevated" padding="md">
        <span class="section-label">Couleur du pattern</span>
        <div class="color-grid">
          {#each DYE_COLORS as dye}
            <button
              class="color-swatch"
              class:active={selectedColor === dye.id}
              style="background:{dye.hex}"
              title={dye.label}
              onclick={() => { selectedColor = dye.id; }}
            ></button>
          {/each}
        </div>
      </Card>

      <Card variant="elevated" padding="md">
        <span class="section-label">Pattern ({PATTERNS.length})</span>
        <div class="pattern-grid">
          {#each PATTERNS as pat}
            <button
              class="pattern-btn"
              class:active={selectedPattern === pat.id}
              title={pat.label}
              onclick={() => { selectedPattern = pat.id; }}
            >
              <canvas
                width="20" height="40"
                class="pattern-preview"
                use:renderPatternPreview={pat.id}
              ></canvas>
            </button>
          {/each}
        </div>

        <button
          class="add-btn"
          onclick={addLayer}
          disabled={layers.length >= MAX_LAYERS}
        >
          + Ajouter ({layers.length}/{MAX_LAYERS})
        </button>
      </Card>
    </div>

    <!-- Center: Preview canvas -->
    <div class="banner-preview">
      <canvas bind:this={canvasEl} class="banner-canvas"></canvas>
    </div>

    <!-- Right: Layers + output -->
    <div class="banner-layers">
      <Card variant="elevated" padding="md">
        <div class="layers-header">
          <span class="section-label">Couches</span>
          {#if layers.length > 0}
            <button class="clear-btn" onclick={clearLayers}>Tout effacer</button>
          {/if}
        </div>

        {#if layers.length === 0}
          <p class="empty-hint">Aucune couche. Sélectionnez un pattern et une couleur, puis cliquez "Ajouter".</p>
        {:else}
          <div class="layer-list">
            {#each layers as layer, i}
              <div class="layer-item">
                <span class="layer-num">{i + 1}</span>
                <span class="layer-color-dot" style="background:{DYE_COLORS[layer.color]?.hex}"></span>
                <span class="layer-name">{PATTERNS.find(p => p.id === layer.pattern)?.label ?? layer.pattern}</span>
                <div class="layer-actions">
                  <button class="layer-btn" onclick={() => moveLayer(i, -1)} disabled={i === 0} title="Monter">▲</button>
                  <button class="layer-btn" onclick={() => moveLayer(i, 1)} disabled={i === layers.length - 1} title="Descendre">▼</button>
                  <button class="layer-btn layer-btn--del" onclick={() => removeLayer(i)} title="Supprimer">×</button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </Card>

      <Card variant="elevated" padding="md">
        <span class="section-label">Commande</span>
        <div class="command-box">
          <code class="command-text">{command}</code>
          <button class="copy-btn" onclick={copyCommand}>
            {copied ? 'Copié !' : 'Copier'}
          </button>
        </div>
      </Card>
    </div>
  </div>
</div>

<style>
  .banner-root { display: flex; flex-direction: column; }

  .banner-layout {
    display: grid;
    grid-template-columns: 260px 1fr 300px;
    gap: 1rem;
    min-height: 520px;
  }

  .banner-controls { display: flex; flex-direction: column; gap: 1rem; }

  /* ── Color grid ── */
  .color-grid { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; margin-top: 0.3rem; }
  .color-swatch {
    width: 100%; aspect-ratio: 1; border-radius: 4px; border: 2px solid transparent;
    cursor: pointer; transition: border-color 120ms ease, transform 120ms ease;
  }
  .color-swatch:hover { transform: scale(1.15); }
  .color-swatch.active { border-color: var(--blue-0, #5e90ff); transform: scale(1.15); }

  /* ── Pattern grid ── */
  .pattern-grid { display: grid; grid-template-columns: repeat(6, 1fr); gap: 3px; margin-top: 0.3rem; max-height: 260px; overflow-y: auto; }
  .pattern-btn {
    padding: 2px; border: 2px solid transparent; border-radius: 3px;
    background: rgba(255,255,255,0.3); cursor: pointer;
    transition: border-color 120ms ease;
  }
  .pattern-btn:hover { border-color: rgba(94,144,255,0.4); }
  .pattern-btn.active { border-color: var(--blue-0, #5e90ff); background: rgba(94,144,255,0.1); }
  .pattern-preview { width: 100%; height: auto; display: block; image-rendering: pixelated; }

  .add-btn {
    margin-top: 0.5rem; padding: 0.45rem; border-radius: 6px; border: 1px solid rgba(70,113,166,0.35);
    background: rgba(94,144,255,0.08); color: var(--blue-0, #5e90ff);
    font-family: inherit; font-size: 0.78rem; font-weight: 600; cursor: pointer;
    transition: background 120ms ease;
  }
  .add-btn:hover { background: rgba(94,144,255,0.15); }
  .add-btn:disabled { opacity: 0.4; cursor: not-allowed; }

  /* ── Preview ── */
  .banner-preview {
    border: 2px solid var(--line-0, rgba(46,94,143,0.34));
    border-radius: var(--radius-lg, 16px);
    overflow: hidden; background: #1a1a1a;
  }
  .banner-canvas { width: 100%; height: 100%; display: block; }

  /* ── Layers ── */
  .banner-layers { display: flex; flex-direction: column; gap: 1rem; }
  .layers-header { display: flex; justify-content: space-between; align-items: center; }
  .clear-btn {
    font-size: 0.68rem; color: var(--danger, #b83b3b); background: none; border: none;
    cursor: pointer; font-weight: 600; text-decoration: underline;
  }
  .empty-hint { font-size: 0.78rem; color: var(--ink-2, #5a7894); margin: 0.5rem 0; line-height: 1.4; }

  .layer-list { display: flex; flex-direction: column; gap: 4px; }
  .layer-item {
    display: flex; align-items: center; gap: 6px; padding: 6px 8px;
    background: rgba(255,255,255,0.2); border-radius: 6px;
    font-size: 0.78rem;
  }
  .layer-num { font-weight: 700; color: var(--ink-2, #5a7894); font-size: 0.7rem; width: 16px; text-align: center; }
  .layer-color-dot { width: 14px; height: 14px; border-radius: 3px; flex-shrink: 0; border: 1px solid rgba(0,0,0,0.15); }
  .layer-name { flex: 1; font-weight: 600; color: var(--ink-0, #0f253a); }
  .layer-actions { display: flex; gap: 2px; }
  .layer-btn {
    width: 22px; height: 22px; border: 1px solid rgba(70,113,166,0.25); border-radius: 4px;
    background: rgba(255,255,255,0.4); font-size: 0.65rem; cursor: pointer;
    display: flex; align-items: center; justify-content: center; color: var(--ink-2, #5a7894);
    transition: background 100ms ease;
  }
  .layer-btn:hover { background: rgba(94,144,255,0.1); }
  .layer-btn:disabled { opacity: 0.3; cursor: default; }
  .layer-btn--del { color: var(--danger, #b83b3b); }
  .layer-btn--del:hover { background: rgba(184,59,59,0.1); }

  /* ── Command ── */
  .command-box {
    display: flex; align-items: center; gap: 8px;
    background: #1a1a2e; border-radius: 6px; padding: 8px 10px; overflow-x: auto; margin-top: 0.3rem;
  }
  .command-text { flex: 1; font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; color: #e0e0e0; word-break: break-all; }
  .copy-btn {
    padding: 0.3rem 0.7rem; border-radius: 4px; border: 1px solid rgba(255,255,255,0.2);
    background: rgba(94,144,255,0.2); color: #fff; font-family: 'Chakra Petch', sans-serif;
    font-size: 0.68rem; font-weight: 600; cursor: pointer; flex-shrink: 0;
  }
  .copy-btn:hover { background: rgba(94,144,255,0.4); }

  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  @media (max-width: 900px) {
    .banner-layout { grid-template-columns: 1fr; }
    .banner-preview { min-height: 350px; }
  }
</style>
