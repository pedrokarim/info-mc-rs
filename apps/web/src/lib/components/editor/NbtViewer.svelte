<script lang="ts">
  import FileDropZone from '$lib/components/ui/FileDropZone.svelte';
  import NbtTreeView from '$lib/components/ui/NbtTreeView.svelte';
  import StructurePresetTree from '$lib/components/ui/StructurePresetTree.svelte';
  import Slider from '$lib/components/ui/Slider.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import StructureViewer3D from './StructureViewer3D.svelte';
  import { parseNbt, nbtToTree, extractStructure } from '$lib/utils/nbt-parser';
  import type { NbtTag, StructureData } from '$lib/utils/nbt-parser';

  let tree = $state<NbtTag | null>(null);
  let structure = $state<StructureData | null>(null);
  let layerY = $state(-1);
  let brightness = $state(100);
  let loading = $state(false);
  let error = $state('');
  let fileName = $state('');

  async function loadPreset(path: string) {
    loading = true;
    error = '';
    fileName = path.split('/').pop() ?? 'structure.nbt';
    tree = null;
    structure = null;

    try {
      const res = await fetch(path);
      if (!res.ok) throw new Error(`Fichier introuvable : ${path}`);
      const buffer = new Uint8Array(await res.arrayBuffer());
      const parsed = parseNbt(buffer);
      tree = nbtToTree(parsed);
      structure = extractStructure(parsed);
      if (structure) layerY = -1;
    } catch (e: any) {
      error = `Erreur : ${e.message || 'format invalide'}`;
    } finally {
      loading = false;
    }
  }

  async function handleFile(file: File) {
    if (!file.name.endsWith('.nbt') && !file.name.endsWith('.schematic') && !file.name.endsWith('.dat')) {
      error = 'Format non supporté. Utilisez un fichier .nbt';
      return;
    }

    loading = true;
    error = '';
    fileName = file.name;
    tree = null;
    structure = null;

    try {
      const buffer = new Uint8Array(await file.arrayBuffer());
      const parsed = parseNbt(buffer);

      // Build tree
      tree = nbtToTree(parsed);

      // Try to extract structure data
      structure = extractStructure(parsed);
      if (structure) {
        layerY = -1; // Show all layers
      }
    } catch (e: any) {
      error = `Erreur de parsing : ${e.message || 'format invalide'}`;
    } finally {
      loading = false;
    }
  }
</script>

<div class="nbt-viewer">
  {#if !tree}
    <!-- Drop zone + presets -->
    <div class="intro-layout">
      <div class="drop-section">
        <FileDropZone
          accept=".nbt,.schematic,.dat"
          label="Glisser un fichier .nbt ici"
          onfile={handleFile}
        />
        {#if loading}
          <p class="loading-msg">Parsing en cours...</p>
        {/if}
        {#if error}
          <p class="error-msg">{error}</p>
        {/if}
      </div>
      <div class="presets-panel">
        <StructurePresetTree onselect={loadPreset} />
      </div>
    </div>
  {:else}
    <!-- Header -->
    <div class="viewer-header">
      <div class="header-info">
        <h2 class="file-name">{fileName}</h2>
        <div class="badges">
          {#if structure}
            <Badge label="{structure.size[0]}×{structure.size[1]}×{structure.size[2]}" variant="info" size="sm" />
            <Badge label="{structure.blocks.length} blocs" variant="default" size="sm" />
            <Badge label="{structure.palette.length} palette" variant="default" size="sm" />
            <Badge label="v{structure.dataVersion}" variant="default" size="sm" />
          {/if}
        </div>
      </div>
      <button class="reset-btn" onclick={() => { tree = null; structure = null; fileName = ''; }}>
        Nouveau fichier
      </button>
    </div>

    <!-- Main content -->
    <div class="viewer-content">
      <!-- Tree panel -->
      <div class="tree-panel">
        <span class="panel-title">Arbre NBT</span>
        <div class="tree-scroll">
          <NbtTreeView tag={tree} />
        </div>
      </div>

      <!-- 3D panel -->
      <div class="viewer-panel">
        {#if structure}
          <div class="viewer-3d-area">
            <StructureViewer3D {structure} {layerY} {brightness} />
          </div>
          <div class="layer-control">
            <Slider
              bind:value={layerY}
              min={-1}
              max={structure.size[1] - 1}
              step={1}
              label="Couche Y"
              showValue={true}
            />
            <span class="layer-hint">{layerY === -1 ? 'Toutes les couches' : `Couche Y = ${layerY}`}</span>
            <Slider
              bind:value={brightness}
              min={10}
              max={150}
              step={5}
              label="Luminosité"
              showValue={true}
            />
          </div>
        {:else}
          <div class="no-structure">
            <p>Ce fichier NBT ne contient pas de données de structure.</p>
            <p class="hint">L'arbre NBT est visible sur la gauche.</p>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .nbt-viewer { display: flex; flex-direction: column; gap: 12px; width: 100%; }

  .intro-layout { display: grid; grid-template-columns: 1fr 300px; gap: 16px; align-items: start; }
  .drop-section { display: flex; flex-direction: column; gap: 8px; justify-content: center; min-height: 200px; }
  .presets-panel {
    padding: 12px; background: var(--surface-1, #edf5fa); border-radius: var(--radius-md, 12px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }
  @media (max-width: 700px) { .intro-layout { grid-template-columns: 1fr; } }
  .loading-msg { font-size: 0.82rem; color: var(--blue-0, #5e90ff); font-weight: 500; text-align: center; }
  .error-msg { font-size: 0.82rem; color: var(--danger, #b83b3b); font-weight: 500; text-align: center; }

  .viewer-header {
    display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap;
    padding: 12px 16px; background: var(--surface-1, #edf5fa);
    border-radius: var(--radius-md, 12px); border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }
  .header-info { display: flex; flex-direction: column; gap: 4px; }
  .file-name { font-family: 'Teko', sans-serif; font-size: 1.4rem; font-weight: 600; color: var(--ink-0, #0f253a); margin: 0; line-height: 1; }
  .badges { display: flex; gap: 6px; flex-wrap: wrap; }

  .reset-btn {
    padding: 6px 14px; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px);
    background: transparent; color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif;
    font-size: 0.75rem; font-weight: 600; cursor: pointer;
    transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .reset-btn:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }

  .viewer-content { display: grid; grid-template-columns: 340px 1fr; gap: 12px; min-height: 500px; }

  .tree-panel {
    display: flex; flex-direction: column; gap: 6px; padding: 12px;
    background: var(--surface-1, #edf5fa); border-radius: var(--radius-md, 12px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); overflow: hidden;
  }
  .panel-title { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .tree-scroll { flex: 1; overflow: auto; max-height: calc(100vh - 260px); }

  .viewer-panel { display: flex; flex-direction: column; gap: 10px; }
  .viewer-3d-area { flex: 1; border-radius: var(--radius-md, 12px); overflow: hidden; border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); min-height: 400px; }

  .layer-control {
    padding: 10px 14px; background: var(--surface-1, #edf5fa);
    border-radius: var(--radius-md, 12px); border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }
  .layer-hint { font-size: 0.7rem; color: var(--ink-2, #5a7894); font-weight: 500; }

  .no-structure {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    flex: 1; text-align: center; color: var(--ink-2, #5a7894);
    background: var(--surface-1, #edf5fa); border-radius: var(--radius-md, 12px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); padding: 2rem;
  }
  .no-structure p { margin: 0.3rem 0; }
  .hint { font-size: 0.78rem; opacity: 0.7; }

  @media (max-width: 800px) {
    .viewer-content { grid-template-columns: 1fr; }
    .tree-scroll { max-height: 300px; }
  }
</style>
