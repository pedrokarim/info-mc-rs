<script lang="ts">
  import { onMount } from 'svelte';
  import SkinCanvas from './SkinCanvas.svelte';
  import SkinEditorViewer3D from './SkinEditorViewer3D.svelte';
  import ToolBar from './ToolBar.svelte';
  import BodyPartMap from './BodyPartMap.svelte';
  import ViewerControlPanel from './ViewerControlPanel.svelte';
  import HistoryControls from './HistoryControls.svelte';
  import ImportExportPanel from './ImportExportPanel.svelte';
  import ColorPicker from '$lib/components/ui/ColorPicker.svelte';
  import ColorPalette from '$lib/components/ui/ColorPalette.svelte';
  import {
    editorState, undo, redo, exportAsPng, loadTemplate,
  } from '$lib/stores/skin-editor.svelte';
  import type { Tool } from '$lib/stores/skin-editor.svelte';

  let {
    apiBase = '',
  }: {
    apiBase?: string;
  } = $props();

  let show2D = $state(false);

  const toolShortcuts: Record<string, Tool> = {
    p: 'pencil', e: 'eraser', i: 'eyedropper', g: 'fill',
    l: 'line', r: 'rect', h: 'pan',
  };

  function handleKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement)?.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) { e.preventDefault(); undo(); return; }
    if ((e.ctrlKey || e.metaKey) && (e.key === 'Z' || (e.key === 'z' && e.shiftKey))) { e.preventDefault(); redo(); return; }
    if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); exportAsPng(); return; }

    if (e.key === '+' || e.key === '=') { editorState.zoom = Math.min(32, editorState.zoom + 1); return; }
    if (e.key === '-') { editorState.zoom = Math.max(1, editorState.zoom - 1); return; }
    if (e.key === 'm' || e.key === 'M') { editorState.mirrorMode = !editorState.mirrorMode; return; }

    const tool = toolShortcuts[e.key.toLowerCase()];
    if (tool) editorState.activeTool = tool;
  }

  function handlePaletteSelect(c: { r: number; g: number; b: number; a: number }) {
    editorState.primaryColor = { ...c };
  }

  function handlePickerChange(c: { r: number; g: number; b: number; a: number }) {
    editorState.primaryColor = { ...c };
  }

  onMount(() => {
    loadTemplate('steve');
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="skin-editor">
  <!-- Left sidebar: tools + colors -->
  <aside class="editor-left">
    <ToolBar />
    <HistoryControls />
    <div class="left-colors">
      <ColorPicker bind:color={editorState.primaryColor} showAlpha={true} onchange={handlePickerChange} />
      <ColorPalette
        selected={editorState.primaryColor}
        recentColors={editorState.recentColors}
        onselect={handlePaletteSelect}
      />
    </div>
  </aside>

  <!-- Center: 3D viewer (main editing surface) -->
  <main class="editor-center">
    <div class="viewer-area">
      <SkinEditorViewer3D />
    </div>

    <!-- 2D texture toggle -->
    <button class="toggle-2d" onclick={() => show2D = !show2D}>
      {show2D ? 'Masquer texture 2D' : 'Afficher texture 2D'}
    </button>

    {#if show2D}
      <div class="canvas-area-mini">
        <SkinCanvas />
      </div>
    {/if}
  </main>

  <!-- Right sidebar: controls -->
  <aside class="editor-right">
    <BodyPartMap />
    <ViewerControlPanel />
    <ImportExportPanel {apiBase} />
  </aside>
</div>

<style>
  .skin-editor {
    display: grid;
    grid-template-columns: auto 1fr 260px;
    gap: 12px;
    width: 100%;
    min-height: 600px;
    padding: 0;
  }

  /* ── Left sidebar ── */
  .editor-left {
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-self: start;
    position: sticky;
    top: 80px;
    max-height: calc(100vh - 100px);
    overflow-y: auto;
    padding: 8px;
  }

  .left-colors {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px;
    background: var(--surface-1, #edf5fa);
    border-radius: var(--radius-md, 12px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }

  /* ── Center: 3D main area ── */
  .editor-center {
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: center;
    min-width: 0;
  }

  .viewer-area {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 500px;
    background: linear-gradient(180deg, rgba(94,144,255,0.06) 0%, rgba(94,144,255,0.01) 100%);
    border-radius: var(--radius-lg, 16px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    overflow: hidden;
  }

  .toggle-2d {
    padding: 6px 16px;
    border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa);
    color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease);
  }
  .toggle-2d:hover {
    border-color: var(--blue-0, #5e90ff);
    color: var(--blue-0, #5e90ff);
  }

  .canvas-area-mini {
    width: 100%;
    height: 360px;
    border-radius: var(--radius-md, 12px);
    overflow: hidden;
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
  }

  /* ── Right sidebar ── */
  .editor-right {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
    background: var(--surface-1, #edf5fa);
    border-radius: var(--radius-md, 12px);
    border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34));
    align-self: start;
    position: sticky;
    top: 80px;
    max-height: calc(100vh - 100px);
    overflow-y: auto;
  }

  /* ── Responsive ── */
  @media (max-width: 1100px) {
    .skin-editor { grid-template-columns: auto 1fr; }
    .editor-right {
      grid-column: 1 / -1;
      flex-direction: row;
      flex-wrap: wrap;
      position: static;
      max-height: none;
      gap: 16px;
    }
    .editor-right > :global(*) { flex: 1; min-width: 200px; }
  }

  @media (max-width: 640px) {
    .skin-editor { grid-template-columns: 1fr; padding: 8px; }
    .editor-left {
      flex-direction: row;
      flex-wrap: wrap;
      overflow-x: auto;
      position: static;
      max-height: none;
    }
    .viewer-area { min-height: 350px; }
    .editor-right { flex-direction: column; }
  }
</style>
