<script lang="ts">
  import FileDropZone from '$lib/components/ui/FileDropZone.svelte';
  import {
    editorState, loadFromImage, loadFromUrl, exportAsPng, loadTemplate,
  } from '$lib/stores/skin-editor.svelte';

  let {
    apiBase = '',
  }: {
    apiBase?: string;
  } = $props();

  let username = $state('');
  let loading = $state(false);
  let error = $state('');

  function handleFile(file: File) {
    if (!file.type.startsWith('image/')) {
      error = 'Le fichier doit être une image PNG';
      return;
    }
    error = '';
    const reader = new FileReader();
    reader.onload = () => {
      const img = new Image();
      img.onload = () => loadFromImage(img);
      img.src = reader.result as string;
    };
    reader.readAsDataURL(file);
  }

  async function loadFromUsername() {
    if (!username.trim()) return;
    loading = true;
    error = '';
    try {
      const res = await fetch(`${apiBase}/api/v1/player/${encodeURIComponent(username.trim())}`);
      if (!res.ok) throw new Error('Joueur introuvable');
      const data = await res.json();
      if (!data.skin?.url) throw new Error('Aucun skin trouvé');
      editorState.slim = data.skin.model === 'slim';
      await loadFromUrl(data.skin.url);
    } catch (e: any) {
      error = e.message || 'Erreur lors du chargement';
    } finally {
      loading = false;
    }
  }
</script>

<div class="import-export">
  <span class="panel-title">Import / Export</span>

  <FileDropZone
    accept="image/png"
    label="Glisser un skin PNG ici"
    onfile={handleFile}
  />

  <div class="username-row">
    <input
      class="username-input"
      type="text"
      bind:value={username}
      placeholder="Pseudo Minecraft..."
      onkeydown={(e) => e.key === 'Enter' && loadFromUsername()}
    />
    <button class="action-btn" disabled={loading || !username.trim()} onclick={loadFromUsername}>
      {loading ? '...' : 'Charger'}
    </button>
  </div>

  {#if error}
    <span class="error-msg">{error}</span>
  {/if}

  <button class="action-btn action-btn--primary" onclick={exportAsPng}>
    Télécharger PNG
  </button>

  <div class="template-section">
    <span class="sub-title">Templates</span>
    <div class="template-row">
      <button class="action-btn" onclick={() => loadTemplate('steve')}>Steve</button>
      <button class="action-btn" onclick={() => loadTemplate('alex')}>Alex</button>
      <button class="action-btn" onclick={() => loadTemplate('pedrokarim')}>PedroKarim</button>
      <button class="action-btn" onclick={() => loadTemplate('blank')}>Vierge</button>
    </div>
  </div>

  <label class="option-row">
    <input type="checkbox" bind:checked={editorState.slim} />
    <span>Modèle slim (Alex)</span>
  </label>
</div>

<style>
  .import-export { display: flex; flex-direction: column; gap: 10px; }
  .panel-title { font-size: 0.7rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .sub-title { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  .username-row { display: flex; gap: 6px; }
  .username-input {
    flex: 1; min-width: 0; padding: 6px 10px; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px); background: var(--surface-1, #edf5fa); color: var(--ink-0, #0f253a);
    font-family: 'Chakra Petch', sans-serif; font-size: 0.8rem; outline: none;
  }
  .username-input:focus { border-color: var(--blue-0, #5e90ff); }

  .action-btn {
    padding: 6px 12px; border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa); color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif;
    font-size: 0.75rem; font-weight: 600; cursor: pointer;
    transition: background var(--ease, 160ms ease), border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .action-btn:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
  .action-btn:disabled { opacity: 0.4; cursor: default; }
  .action-btn--primary { background: var(--blue-0, #5e90ff); color: #fff; border-color: var(--blue-1, #345fcd); }
  .action-btn--primary:hover { background: #6e9dff; }

  .error-msg { font-size: 0.72rem; color: var(--danger, #b83b3b); font-weight: 500; }

  .template-section { display: flex; flex-direction: column; gap: 6px; }
  .template-row { display: flex; gap: 4px; flex-wrap: wrap; }

  .option-row { display: flex; align-items: center; gap: 6px; font-size: 0.75rem; font-weight: 500; color: var(--ink-1, #2d4a65); cursor: pointer; }
  .option-row input[type="checkbox"] { width: 16px; height: 16px; accent-color: var(--blue-0, #5e90ff); cursor: pointer; }
</style>
