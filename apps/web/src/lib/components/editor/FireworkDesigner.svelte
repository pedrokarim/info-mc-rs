<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte';
  import {
    SHAPES, FIREWORK_COLORS, createExplosion, toGiveCommand, renderFireworkPreview,
    type FireworkState, type FireworkExplosion,
  } from '$lib/utils/firework';

  let flight = $state(2);
  let explosions = $state<FireworkExplosion[]>([createExplosion()]);
  let activeExpIdx = $state(0);
  let copied = $state(false);
  let canvasEl: HTMLCanvasElement | undefined = $state();
  let animTime = $state(0);

  let fwState: FireworkState = $derived({ flight, explosions });
  let command = $derived(toGiveCommand(fwState));
  let activeExp = $derived(explosions[activeExpIdx] ?? null);

  function addExplosion() {
    explosions = [...explosions, createExplosion()];
    activeExpIdx = explosions.length - 1;
  }

  function removeExplosion(i: number) {
    explosions = explosions.filter((_, idx) => idx !== i);
    if (activeExpIdx >= explosions.length) activeExpIdx = Math.max(0, explosions.length - 1);
  }

  function toggleColor(colorIdx: number) {
    if (!activeExp) return;
    const exp = explosions[activeExpIdx];
    if (exp.colors.includes(colorIdx)) {
      exp.colors = exp.colors.filter(c => c !== colorIdx);
    } else {
      exp.colors = [...exp.colors, colorIdx];
    }
    explosions = [...explosions];
  }

  function toggleFadeColor(colorIdx: number) {
    if (!activeExp) return;
    const exp = explosions[activeExpIdx];
    if (exp.fadeColors.includes(colorIdx)) {
      exp.fadeColors = exp.fadeColors.filter(c => c !== colorIdx);
    } else {
      exp.fadeColors = [...exp.fadeColors, colorIdx];
    }
    explosions = [...explosions];
  }

  function setShape(shapeId: number) {
    if (!activeExp) return;
    explosions[activeExpIdx].shape = shapeId;
    explosions = [...explosions];
  }

  function toggleEffect(key: 'trail' | 'twinkle') {
    if (!activeExp) return;
    explosions[activeExpIdx][key] = !explosions[activeExpIdx][key];
    explosions = [...explosions];
  }

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(command);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch { /* fail */ }
  }

  function replay() { animTime = 0; }

  // Animation loop
  let animStart = 0;
  function animLoop(ts: number) {
    if (animStart === 0) animStart = ts;
    animTime = ((ts - animStart) % 3000) / 3000; // 3s loop

    if (canvasEl) {
      const ctx = canvasEl.getContext('2d');
      if (ctx) {
        const dpr = window.devicePixelRatio || 1;
        const w = canvasEl.clientWidth, h = canvasEl.clientHeight;
        if (canvasEl.width !== w * dpr || canvasEl.height !== h * dpr) {
          canvasEl.width = w * dpr; canvasEl.height = h * dpr;
        }
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        renderFireworkPreview(ctx, fwState, w, h, animTime);
      }
    }
    requestAnimationFrame(animLoop);
  }

  $effect(() => {
    if (canvasEl) requestAnimationFrame(animLoop);
  });
</script>

<div class="fw-root">
  <div class="fw-layout">
    <!-- Left: controls -->
    <div class="fw-controls">
      <!-- Flight duration -->
      <Card variant="elevated" padding="md">
        <span class="section-label">Durée de vol</span>
        <div class="flight-row">
          {#each [1, 2, 3] as f}
            <button class="pill" class:active={flight === f} onclick={() => { flight = f; }}>
              {f} {'🚀'.repeat(f)}
            </button>
          {/each}
        </div>
      </Card>

      <!-- Explosions tabs -->
      <Card variant="elevated" padding="md">
        <div class="exp-header">
          <span class="section-label">Explosions</span>
          <button class="add-btn" onclick={addExplosion}>+ Ajouter</button>
        </div>
        <div class="exp-tabs">
          {#each explosions as _, i}
            <div class="exp-tab" class:active={activeExpIdx === i}>
              <button class="exp-tab-btn" onclick={() => { activeExpIdx = i; }}>#{i + 1}</button>
              {#if explosions.length > 1}
                <button class="exp-del" onclick={() => removeExplosion(i)}>×</button>
              {/if}
            </div>
          {/each}
        </div>
      </Card>

      {#if activeExp}
        <!-- Shape -->
        <Card variant="elevated" padding="md">
          <span class="section-label">Forme</span>
          <div class="shape-grid">
            {#each SHAPES as shape}
              <button class="shape-btn" class:active={activeExp.shape === shape.id} onclick={() => setShape(shape.id)}>
                {shape.label}
              </button>
            {/each}
          </div>
        </Card>

        <!-- Colors -->
        <Card variant="elevated" padding="md">
          <span class="section-label">Couleurs</span>
          <div class="color-grid">
            {#each FIREWORK_COLORS as fc, i}
              <button
                class="color-swatch"
                class:active={activeExp.colors.includes(i)}
                style="background:{fc.hex}"
                title={fc.label}
                onclick={() => toggleColor(i)}
              ></button>
            {/each}
          </div>
        </Card>

        <!-- Fade colors -->
        <Card variant="elevated" padding="md">
          <span class="section-label">Couleurs de fondu</span>
          <div class="color-grid">
            {#each FIREWORK_COLORS as fc, i}
              <button
                class="color-swatch"
                class:active={activeExp.fadeColors.includes(i)}
                style="background:{fc.hex}"
                title={fc.label}
                onclick={() => toggleFadeColor(i)}
              ></button>
            {/each}
          </div>
        </Card>

        <!-- Effects -->
        <Card variant="elevated" padding="md">
          <span class="section-label">Effets</span>
          <div class="effect-row">
            <button class="effect-btn" class:active={activeExp.trail} onclick={() => toggleEffect('trail')}>
              Traînée
            </button>
            <button class="effect-btn" class:active={activeExp.twinkle} onclick={() => toggleEffect('twinkle')}>
              Scintillement
            </button>
          </div>
        </Card>
      {/if}
    </div>

    <!-- Center: preview canvas -->
    <div class="fw-preview">
      <canvas bind:this={canvasEl} class="fw-canvas"></canvas>
      <button class="replay-btn" onclick={replay} title="Rejouer">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M1 4v6h6" /><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />
        </svg>
      </button>
    </div>

    <!-- Right: command -->
    <div class="fw-output">
      <Card variant="elevated" padding="md">
        <span class="section-label">Commande</span>
        <div class="command-box">
          <code class="command-text">{command}</code>
          <button class="copy-btn" onclick={copyCommand}>
            {copied ? 'Copié !' : 'Copier'}
          </button>
        </div>
      </Card>

      <Card variant="elevated" padding="md">
        <span class="section-label">Résumé</span>
        <div class="summary">
          <div class="sum-item"><span class="sum-key">Vol</span><span class="sum-val">{flight}</span></div>
          <div class="sum-item"><span class="sum-key">Explosions</span><span class="sum-val">{explosions.length}</span></div>
          {#each explosions as exp, i}
            <div class="sum-exp">
              <span class="sum-exp-title">#{i + 1} — {SHAPES[exp.shape]?.label}</span>
              <div class="sum-colors">
                {#each exp.colors as c}
                  <span class="sum-dot" style="background:{FIREWORK_COLORS[c]?.hex}"></span>
                {/each}
                {#if exp.fadeColors.length > 0}
                  <span class="sum-fade-arrow">→</span>
                  {#each exp.fadeColors as c}
                    <span class="sum-dot" style="background:{FIREWORK_COLORS[c]?.hex}"></span>
                  {/each}
                {/if}
              </div>
              <div class="sum-effects">
                {#if exp.trail}<span class="sum-tag">Traînée</span>{/if}
                {#if exp.twinkle}<span class="sum-tag">Scintillement</span>{/if}
              </div>
            </div>
          {/each}
        </div>
      </Card>
    </div>
  </div>
</div>

<style>
  .fw-root { display: flex; flex-direction: column; }
  .fw-layout { display: grid; grid-template-columns: 280px 1fr 280px; gap: 1rem; min-height: 520px; }
  .fw-controls { display: flex; flex-direction: column; gap: 0.8rem; overflow-y: auto; max-height: 80vh; }

  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  /* Flight */
  .flight-row { display: flex; gap: 4px; margin-top: 0.3rem; }
  .pill { flex: 1; padding: 0.4rem; font-size: 0.75rem; font-family: inherit; font-weight: 600; border: 1px solid rgba(70,113,166,0.35); border-radius: 6px; background: rgba(255,255,255,0.5); color: var(--ink-1); cursor: pointer; transition: all 120ms; }
  .pill:hover { background: rgba(94,144,255,0.08); }
  .pill.active { background: var(--blue-0, #5e90ff); color: #fff; border-color: var(--blue-0); }

  /* Explosion tabs */
  .exp-header { display: flex; justify-content: space-between; align-items: center; }
  .add-btn { font-size: 0.68rem; color: var(--blue-0); background: none; border: none; cursor: pointer; font-weight: 600; }
  .exp-tabs { display: flex; gap: 4px; margin-top: 0.4rem; flex-wrap: wrap; }
  .exp-tab { display: flex; align-items: center; border: 1px solid rgba(70,113,166,0.3); border-radius: 5px; overflow: hidden; }
  .exp-tab.active { border-color: var(--blue-0); }
  .exp-tab-btn { padding: 0.3rem 0.5rem; font-size: 0.72rem; font-weight: 600; background: rgba(255,255,255,0.4); border: none; cursor: pointer; color: var(--ink-1); }
  .exp-tab.active .exp-tab-btn { background: var(--blue-0); color: #fff; }
  .exp-del { padding: 0.2rem 0.4rem; font-size: 0.65rem; background: none; border: none; border-left: 1px solid rgba(70,113,166,0.2); color: var(--danger, #b83b3b); cursor: pointer; }

  /* Shape */
  .shape-grid { display: flex; flex-direction: column; gap: 3px; margin-top: 0.3rem; }
  .shape-btn { padding: 0.35rem 0.6rem; font-size: 0.75rem; font-family: inherit; font-weight: 600; border: 1px solid rgba(70,113,166,0.3); border-radius: 5px; background: rgba(255,255,255,0.4); color: var(--ink-1); cursor: pointer; text-align: left; transition: all 120ms; }
  .shape-btn:hover { background: rgba(94,144,255,0.08); }
  .shape-btn.active { background: var(--blue-0); color: #fff; border-color: var(--blue-0); }

  /* Colors */
  .color-grid { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; margin-top: 0.3rem; }
  .color-swatch { width: 100%; aspect-ratio: 1; border-radius: 4px; border: 2px solid transparent; cursor: pointer; transition: all 120ms; }
  .color-swatch:hover { transform: scale(1.15); }
  .color-swatch.active { border-color: var(--blue-0); transform: scale(1.15); box-shadow: 0 0 0 2px rgba(94,144,255,0.3); }

  /* Effects */
  .effect-row { display: flex; gap: 6px; margin-top: 0.3rem; }
  .effect-btn { flex: 1; padding: 0.4rem; font-size: 0.75rem; font-family: inherit; font-weight: 600; border: 1px solid rgba(70,113,166,0.3); border-radius: 6px; background: rgba(255,255,255,0.4); color: var(--ink-1); cursor: pointer; transition: all 120ms; }
  .effect-btn.active { background: var(--blue-0); color: #fff; border-color: var(--blue-0); }

  /* Preview */
  .fw-preview { position: relative; border: 2px solid var(--line-0, rgba(46,94,143,0.34)); border-radius: var(--radius-lg, 16px); overflow: hidden; background: #0a0a1e; min-height: 400px; }
  .fw-canvas { width: 100%; height: 100%; display: block; }
  .replay-btn { position: absolute; top: 10px; right: 10px; width: 30px; height: 30px; border: 1px solid rgba(255,255,255,0.2); border-radius: 6px; background: rgba(0,0,0,0.5); color: rgba(255,255,255,0.7); cursor: pointer; display: flex; align-items: center; justify-content: center; backdrop-filter: blur(4px); }
  .replay-btn:hover { background: rgba(0,0,0,0.7); color: #fff; }

  /* Output */
  .fw-output { display: flex; flex-direction: column; gap: 0.8rem; }
  .command-box { display: flex; align-items: center; gap: 8px; background: #1a1a2e; border-radius: 6px; padding: 8px 10px; overflow-x: auto; margin-top: 0.3rem; }
  .command-text { flex: 1; font-family: 'JetBrains Mono', monospace; font-size: 0.68rem; color: #e0e0e0; word-break: break-all; }
  .copy-btn { padding: 0.3rem 0.7rem; border-radius: 4px; border: 1px solid rgba(255,255,255,0.2); background: rgba(94,144,255,0.2); color: #fff; font-family: 'Chakra Petch', sans-serif; font-size: 0.68rem; font-weight: 600; cursor: pointer; flex-shrink: 0; }
  .copy-btn:hover { background: rgba(94,144,255,0.4); }

  /* Summary */
  .summary { display: flex; flex-direction: column; gap: 0.5rem; margin-top: 0.3rem; }
  .sum-item { display: flex; justify-content: space-between; font-size: 0.78rem; }
  .sum-key { color: var(--ink-2); font-weight: 600; }
  .sum-val { color: var(--ink-0); font-weight: 700; }
  .sum-exp { padding: 6px 8px; background: rgba(255,255,255,0.15); border-radius: 6px; }
  .sum-exp-title { font-size: 0.72rem; font-weight: 600; color: var(--ink-0); }
  .sum-colors { display: flex; align-items: center; gap: 3px; margin-top: 3px; flex-wrap: wrap; }
  .sum-dot { width: 12px; height: 12px; border-radius: 3px; border: 1px solid rgba(0,0,0,0.15); }
  .sum-fade-arrow { font-size: 0.65rem; color: var(--ink-2); margin: 0 2px; }
  .sum-effects { display: flex; gap: 4px; margin-top: 3px; }
  .sum-tag { font-size: 0.6rem; padding: 1px 5px; border-radius: 3px; background: rgba(94,144,255,0.1); color: var(--blue-0); font-weight: 600; }

  @media (max-width: 900px) {
    .fw-layout { grid-template-columns: 1fr; }
    .fw-preview { min-height: 300px; }
    .fw-controls { max-height: none; }
  }
</style>
