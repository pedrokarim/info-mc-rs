<script lang="ts">
  import Select from '$lib/components/ui/Select.svelte';
  import Switch from '$lib/components/ui/Switch.svelte';
  import {
    mapState, setSeed, setVersion, setDimension, setCenter, zoomIn, zoomOut,
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

  // Structure types grouped by dimension (matching chunkbase)
  const ALL_STRUCTURES = [
    // Overworld
    { id: 0,  name: 'Village',            icon: 'village',           dim: 'overworld' },
    { id: 1,  name: 'Temple du désert',   icon: 'desert-temple',     dim: 'overworld' },
    { id: 2,  name: 'Temple de jungle',   icon: 'jungle-temple',     dim: 'overworld' },
    { id: 3,  name: 'Cabane de sorcière', icon: 'witch-hut',         dim: 'overworld' },
    { id: 4,  name: 'Igloo',              icon: 'igloo',             dim: 'overworld' },
    { id: 5,  name: 'Monument océanique', icon: 'ocean-monument',    dim: 'overworld' },
    { id: 6,  name: 'Manoir',             icon: 'mansion',           dim: 'overworld' },
    { id: 7,  name: 'Avant-poste',        icon: 'pillager-outpost',  dim: 'overworld' },
    { id: 8,  name: 'Forteresse',         icon: 'stronghold',        dim: 'overworld' },
    { id: 9,  name: 'Ruine océanique',    icon: 'ocean-ruin',        dim: 'overworld' },
    { id: 10, name: 'Épave',              icon: 'shipwreck',         dim: 'overworld' },
    { id: 11, name: 'Trésor enfoui',      icon: 'buried-treasure',   dim: 'overworld' },
    { id: 12, name: 'Portail en ruines',  icon: 'ruined-portal',     dim: 'overworld' },
    { id: 13, name: 'Cité antique',       icon: 'ancient-city',      dim: 'overworld' },
    { id: 14, name: 'Ruines du sentier',  icon: 'trail-ruin',        dim: 'overworld' },
    { id: 15, name: 'Chambre d\'épreuve', icon: 'trial-chamber',     dim: 'overworld' },
    { id: 18, name: 'Mine abandonnée',    icon: 'mineshaft',         dim: 'overworld' },
    { id: 19, name: 'Donjon',             icon: 'dungeon',           dim: 'overworld' },
    { id: 20, name: 'Puits du désert',    icon: 'desert-well',       dim: 'overworld' },
    { id: 21, name: 'Fossile',            icon: 'fossil',            dim: 'overworld' },
    // Nether
    { id: 16, name: 'Forteresse du Nether', icon: 'nether-fortress', dim: 'nether' },
    { id: 17, name: 'Bastion',            icon: 'pillager-outpost',  dim: 'nether' },
    { id: 12, name: 'Portail en ruines',  icon: 'ruined-portal',     dim: 'nether' },
    // End
    { id: 23, name: 'Cité de l\'End',     icon: 'end-city',          dim: 'end' },
  ];

  // Filter structures by current dimension
  let STRUCTURE_TYPES = $derived(
    ALL_STRUCTURES.filter(s => s.dim === mapState.dimension)
  );

  function toggleStructure(id: number) {
    if (mapState.enabledStructures.has(id)) {
      mapState.enabledStructures.delete(id);
    } else {
      mapState.enabledStructures.add(id);
    }
    // Force reactivity
    mapState.enabledStructures = new Set(mapState.enabledStructures);
  }

  function toggleAllStructures() {
    if (mapState.enabledStructures.size === STRUCTURE_TYPES.length) {
      mapState.enabledStructures = new Set();
    } else {
      mapState.enabledStructures = new Set(STRUCTURE_TYPES.map(s => s.id));
    }
  }

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

  // Find closest structure to cursor
  let nearestStructure = $derived.by(() => {
    if (!mapState.hoverActive || mapState.structures.length === 0) return null;
    const hx = mapState.hoverWorldX;
    const hz = mapState.hoverWorldZ;
    let best: { name: string; x: number; z: number; dist: number } | null = null;
    for (const s of mapState.structures) {
      const dx = s.x - hx;
      const dz = s.z - hz;
      const dist = Math.sqrt(dx * dx + dz * dz);
      if (dist < 256 && (!best || dist < best.dist)) {
        best = { name: s.name, x: s.x, z: s.z, dist: Math.round(dist) };
      }
    }
    return best;
  });

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
    <span class="section-label">Seed</span>
    <form class="seed-row" onsubmit={(e) => { e.preventDefault(); handleSeedSubmit(); }}>
      <input
        type="text"
        class="seed-input"
        placeholder="12345 ou texte..."
        bind:value={seedInput}
      />
      <button type="submit" class="btn-go" disabled={!seedInput.trim()}>Go</button>
      <button type="button" class="btn-dice" onclick={handleRandomSeed} title="Seed aléatoire">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="2" y="2" width="20" height="20" rx="3" />
          <circle cx="8" cy="8" r="1.5" fill="currentColor" />
          <circle cx="16" cy="8" r="1.5" fill="currentColor" />
          <circle cx="8" cy="16" r="1.5" fill="currentColor" />
          <circle cx="16" cy="16" r="1.5" fill="currentColor" />
          <circle cx="12" cy="12" r="1.5" fill="currentColor" />
        </svg>
      </button>
    </form>
    {#if isTextSeed}
      <span class="seed-hint">Hash Java : <code>{textSeedHash}</code></span>
    {:else}
      <span class="seed-hint-subtle">Nombre, mot ou phrase</span>
    {/if}
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

  <!-- Dimension -->
  <div class="section">
    <span class="section-label">Dimension</span>
    <div class="dim-row">
      <button
        class="dim-btn" class:active={mapState.dimension === 'overworld'}
        onclick={() => setDimension('overworld')}
      >🌍 Overworld</button>
      <button
        class="dim-btn" class:active={mapState.dimension === 'nether'}
        onclick={() => setDimension('nether')}
      >🔥 Nether</button>
      <button
        class="dim-btn" class:active={mapState.dimension === 'end'}
        onclick={() => setDimension('end')}
      >🌌 End</button>
    </div>
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

  <!-- Structures -->
  <div class="section">
    <div class="section-label-row">
      <span class="section-label">Structures</span>
      <button class="btn-toggle-all" onclick={toggleAllStructures}>
        {mapState.enabledStructures.size === STRUCTURE_TYPES.length ? 'Aucune' : 'Toutes'}
      </button>
    </div>
    <div class="struct-grid">
      {#each STRUCTURE_TYPES as st}
        <button
          class="struct-btn"
          class:active={mapState.enabledStructures.has(st.id)}
          title={st.name}
          onclick={() => toggleStructure(st.id)}
        >
          <img src="/images/ui/structures/{st.icon}.png" alt={st.name} class="struct-icon" />
        </button>
      {/each}
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
        {#if nearestStructure}
          <span class="info-k">Structure</span>
          <span class="info-v struct-near">
            {nearestStructure.name}
            <span class="struct-dist">({nearestStructure.dist}m)</span>
          </span>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Structure count -->
  {#if mapState.structures.length > 0}
    <div class="section">
      <span class="section-label">Structures trouvées</span>
      <span class="struct-count">{mapState.structures.length} structures</span>
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
    gap: 4px;
    align-items: center;
  }

  .seed-input {
    flex: 1;
    min-width: 0;
    height: 34px;
    box-sizing: border-box;
    padding: 0 10px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-0, #0f253a);
    font-size: 0.8rem;
    font-family: monospace;
  }
  .seed-input:focus {
    outline: 2px solid rgba(95, 143, 255, 0.5);
    outline-offset: 1px;
    border-color: var(--blue-0, #5e90ff);
  }

  .seed-hint {
    font-size: 0.7rem;
    color: var(--blue-0, #5e90ff);
  }
  .seed-hint code {
    font-family: monospace;
    background: rgba(94, 144, 255, 0.08);
    padding: 1px 4px;
    border-radius: 3px;
  }
  .seed-hint-subtle {
    font-size: 0.67rem;
    color: var(--ink-2, #5a7894);
    font-style: italic;
  }

  .btn-go {
    height: 34px;
    padding: 0 12px;
    flex-shrink: 0;
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
    height: 34px;
    width: 34px;
    flex-shrink: 0;
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

  /* ── Dimension buttons ── */
  .dim-row {
    display: flex;
    gap: 4px;
  }

  .dim-btn {
    flex: 1;
    padding: 5px 4px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: transparent;
    color: var(--ink-2, #5a7894);
    font-family: inherit;
    font-size: 0.68rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 120ms ease;
    text-align: center;
  }

  .dim-btn:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  .dim-btn.active {
    background: var(--blue-0, #5e90ff);
    border-color: var(--blue-0, #5e90ff);
    color: #fff;
  }

  /* ── Structures grid ── */
  .section-label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .btn-toggle-all {
    font-size: 0.65rem;
    font-weight: 600;
    padding: 2px 8px;
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 4px;
    background: transparent;
    color: var(--ink-2, #5a7894);
    cursor: pointer;
  }
  .btn-toggle-all:hover {
    color: var(--blue-0, #5e90ff);
    border-color: var(--blue-0, #5e90ff);
  }

  .struct-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 3px;
  }

  .struct-btn {
    width: 32px;
    height: 32px;
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.3;
    filter: grayscale(1);
    transition: opacity 120ms ease, filter 120ms ease, border-color 120ms ease;
  }

  .struct-btn.active {
    opacity: 1;
    filter: none;
    border-color: var(--blue-0, #5e90ff);
    background: rgba(94, 144, 255, 0.08);
  }

  .struct-btn:hover {
    opacity: 1;
    filter: none;
  }

  .struct-icon {
    width: 22px;
    height: 22px;
    object-fit: contain;
    image-rendering: pixelated;
    pointer-events: none;
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

  .struct-near {
    color: var(--blue-0, #5e90ff) !important;
    font-weight: 600;
  }

  .struct-dist {
    font-weight: 400;
    font-size: 0.7rem;
    opacity: 0.7;
  }

  .struct-count {
    font-size: 0.8rem;
    color: var(--ink-1, #2d4a65);
  }

  .status {
    font-size: 0.75rem;
    color: var(--ink-2, #5a7894);
    text-align: center;
    padding: 4px;
  }
</style>
