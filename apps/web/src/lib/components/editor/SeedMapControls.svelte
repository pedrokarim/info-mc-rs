<script lang="ts">
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Switch from '$lib/components/ui/Switch.svelte';
  import {
    mapState, setSeed, setVersion, setCenter, zoomIn, zoomOut,
  } from '$lib/stores/seed-map.svelte';

  let seedInput = $state('');
  let goToX = $state('0');
  let goToZ = $state('0');

  // Detect if the input is text (not a number) to show the hash
  let isTextSeed = $derived(seedInput.trim() !== '' && isNaN(Number(seedInput.trim())));
  let textSeedHash = $derived.by(() => {
    if (!isTextSeed) return '';
    let hash = 0;
    for (const ch of seedInput.trim()) {
      hash = (Math.imul(hash, 31) + ch.charCodeAt(0)) | 0;
    }
    return hash.toString();
  });

  const MC_VERSIONS = [
    { value: '1.21', label: '1.21 — Tricky Trials' },
    { value: '1.20', label: '1.20 — Trails & Tales' },
    { value: '1.19', label: '1.19 — The Wild Update' },
    { value: '1.18', label: '1.18 — Caves & Cliffs II' },
    { value: '1.17', label: '1.17 — Caves & Cliffs I' },
    { value: '1.16', label: '1.16 — Nether Update' },
    { value: '1.15', label: '1.15 — Buzzy Bees' },
    { value: '1.14', label: '1.14 — Village & Pillage' },
    { value: '1.13', label: '1.13 — Update Aquatic' },
    { value: '1.12', label: '1.12 — World of Color' },
    { value: '1.7',  label: '1.7 — The Update that Changed the World' },
  ];

  function handleSeedSubmit() {
    if (seedInput.trim()) {
      setSeed(seedInput.trim());
    }
  }

  function handleGoTo() {
    const x = parseInt(goToX) || 0;
    const z = parseInt(goToZ) || 0;
    setCenter(x, z);
  }

  function handleRandomSeed() {
    const lo = Math.floor(Math.random() * 0xFFFFFFFF) - 0x80000000;
    const hi = Math.floor(Math.random() * 0xFFFFFFFF) - 0x80000000;
    const seed = BigInt(hi) * BigInt(0x100000000) + BigInt(lo >>> 0);
    seedInput = seed.toString();
    setSeed(seedInput);
  }
</script>

<aside class="controls">
  <!-- Seed input -->
  <div class="section">
    <form class="seed-row" onsubmit={(e) => { e.preventDefault(); handleSeedSubmit(); }}>
      <div class="seed-input-wrap">
        <Input
          label="Seed"
          placeholder="12345 ou texte..."
          bind:value={seedInput}
          hint={isTextSeed ? `Hash Java : ${textSeedHash}` : 'Nombre, mot ou phrase'}
        />
      </div>
      <div class="seed-actions">
        <button type="submit" class="btn-go" disabled={!seedInput.trim()}>Go</button>
        <button type="button" class="btn-dice" onclick={handleRandomSeed} title="Seed aléatoire">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="2" width="20" height="20" rx="3" />
            <circle cx="8" cy="8" r="1.5" fill="currentColor" />
            <circle cx="16" cy="8" r="1.5" fill="currentColor" />
            <circle cx="8" cy="16" r="1.5" fill="currentColor" />
            <circle cx="16" cy="16" r="1.5" fill="currentColor" />
            <circle cx="12" cy="12" r="1.5" fill="currentColor" />
          </svg>
        </button>
      </div>
    </form>
  </div>

  <!-- Version -->
  <div class="section">
    <Select
      label="Version Minecraft"
      options={MC_VERSIONS}
      bind:value={mapState.mcVersion}
      onchange={(v) => setVersion(v)}
    />
    {#if parseFloat(mapState.mcVersion) >= 1.18}
      <span class="badge badge--modern">Multi-Noise</span>
    {:else}
      <span class="badge badge--legacy">Legacy Layers</span>
    {/if}
  </div>

  <!-- Layers -->
  <div class="section">
    <span class="section-label">Couches</span>
    <div class="switches">
      <Switch label="Biomes" bind:checked={mapState.showBiomes} />
      <Switch label="Slime Chunks" bind:checked={mapState.showSlimeChunks} />
      <Switch label="Grille" bind:checked={mapState.showGrid} />
      <Switch label="Coordonnées" bind:checked={mapState.showCoordinates} />
    </div>
  </div>

  <!-- Zoom -->
  <div class="section">
    <span class="section-label">Zoom</span>
    <div class="zoom-row">
      <button class="btn-sq" onclick={zoomOut}>−</button>
      <span class="zoom-value">{mapState.zoom}x</span>
      <button class="btn-sq" onclick={zoomIn}>+</button>
    </div>
  </div>

  <!-- Go to -->
  <div class="section">
    <span class="section-label">Aller à</span>
    <div class="goto-row">
      <span class="coord-prefix">X</span>
      <input type="number" class="coord-input" bind:value={goToX} />
      <span class="coord-prefix">Z</span>
      <input type="number" class="coord-input" bind:value={goToZ} />
      <button class="btn-sq" onclick={handleGoTo}>→</button>
    </div>
    <button class="btn-outline" onclick={() => setCenter(0, 0)}>
      Retour au spawn (0, 0)
    </button>
  </div>

  <!-- Hover info -->
  {#if mapState.hoverActive && mapState.seedValid}
    <div class="section info-section">
      <span class="section-label">Info curseur</span>
      <div class="info-grid">
        <span class="info-k">Bloc</span>
        <span class="info-v">{mapState.hoverWorldX}, {mapState.hoverWorldZ}</span>
        <span class="info-k">Chunk</span>
        <span class="info-v">{mapState.hoverChunkX}, {mapState.hoverChunkZ}</span>
        {#if mapState.hoverBiome}
          <span class="info-k">Biome</span>
          <span class="info-v">{mapState.hoverBiome}</span>
        {/if}
        <span class="info-k">Slime</span>
        <span class="info-v" class:slime-yes={mapState.hoverIsSlime}>
          {mapState.hoverIsSlime ? 'Oui' : 'Non'}
        </span>
      </div>
    </div>
  {/if}

  {#if mapState.computing}
    <div class="status">Calcul en cours…</div>
  {/if}
</aside>

<style>
  .controls {
    width: 260px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
    background: var(--surface-1, #edf5fa);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-md, 12px);
    overflow-y: auto;
    max-height: calc(100vh - 140px);
    align-self: start;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .section-label {
    font-size: 0.73rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-1, #2d4a65);
  }

  /* ── Seed row ── */
  .seed-row {
    display: flex;
    gap: 6px;
    align-items: flex-end;
  }

  .seed-input-wrap {
    flex: 1;
    min-width: 0;
  }

  .seed-actions {
    display: flex;
    gap: 4px;
    padding-bottom: 1px; /* align with input bottom */
  }

  .btn-go {
    height: 36px;
    padding: 0 14px;
    border: none;
    border-radius: 8px;
    background: var(--blue-0, #5e90ff);
    color: #fff;
    font-family: inherit;
    font-size: 0.8rem;
    font-weight: 700;
    cursor: pointer;
    transition: background var(--ease, 160ms ease);
  }
  .btn-go:hover { background: var(--blue-1, #345fcd); }
  .btn-go:disabled { opacity: 0.4; cursor: default; }

  .btn-dice {
    height: 36px;
    width: 36px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-2, #5a7894);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .btn-dice:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  /* ── Badge ── */
  .badge {
    display: inline-block;
    font-size: 0.65rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 4px;
    width: fit-content;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .badge--modern {
    background: rgba(22, 154, 96, 0.12);
    color: var(--ok, #169a60);
  }
  .badge--legacy {
    background: rgba(200, 150, 40, 0.12);
    color: #a07820;
  }

  /* ── Switches ── */
  .switches {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  /* ── Zoom ── */
  .zoom-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .zoom-value {
    font-size: 0.85rem;
    font-family: monospace;
    color: var(--ink-0, #0f253a);
    min-width: 36px;
    text-align: center;
    font-weight: 600;
  }

  .btn-sq {
    width: 32px;
    height: 32px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-1, #2d4a65);
    font-size: 1rem;
    font-family: inherit;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
    padding: 0;
  }
  .btn-sq:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  /* ── Go to ── */
  .goto-row {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .coord-prefix {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--ink-2, #5a7894);
  }

  .coord-input {
    width: 60px;
    box-sizing: border-box;
    padding: 0.4rem 0.5rem;
    border: 1px solid rgba(70, 113, 166, 0.42);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-0, #0f253a);
    font-size: 0.8rem;
    font-family: monospace;
    transition: border-color 120ms ease;
  }
  .coord-input:focus {
    outline: 2px solid rgba(95, 143, 255, 0.6);
    outline-offset: 1px;
    border-color: var(--blue-0, #5e90ff);
  }

  .btn-outline {
    width: 100%;
    padding: 7px 12px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: transparent;
    color: var(--ink-1, #2d4a65);
    font-family: inherit;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .btn-outline:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  /* ── Info grid ── */
  .info-section {
    border-top: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    padding-top: 10px;
  }

  .info-grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 2px 10px;
    font-size: 0.8rem;
  }

  .info-k {
    color: var(--ink-2, #5a7894);
    font-weight: 600;
  }

  .info-v {
    color: var(--ink-0, #0f253a);
    font-family: monospace;
  }

  .slime-yes {
    color: var(--ok, #169a60);
    font-weight: 700;
  }

  .status {
    font-size: 0.75rem;
    color: var(--ink-2, #5a7894);
    text-align: center;
    padding: 4px;
  }
</style>
