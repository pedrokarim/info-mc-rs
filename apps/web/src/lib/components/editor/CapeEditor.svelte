<script lang="ts">
  import { onMount } from 'svelte';
  import CapeCanvas from './CapeCanvas.svelte';
  import CapeEditorViewer3D from './CapeEditorViewer3D.svelte';
  import CapeToolBar from './CapeToolBar.svelte';
  import CapeHistoryControls from './CapeHistoryControls.svelte';
  import CapeViewerControlPanel from './CapeViewerControlPanel.svelte';
  import CapeImportExportPanel from './CapeImportExportPanel.svelte';
  import CapePresetGrid from './CapePresetGrid.svelte';
  import ColorPicker from '$lib/components/ui/ColorPicker.svelte';
  import ColorPalette from '$lib/components/ui/ColorPalette.svelte';
  import { capeState, undo, redo, exportAsPng, loadPreset } from '$lib/stores/cape-editor.svelte';
  import type { Tool } from '$lib/stores/cape-editor.svelte';

  let { apiBase = '' }: { apiBase?: string } = $props();
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
    if (e.key === '+' || e.key === '=') { capeState.zoom = Math.min(48, capeState.zoom + 1); return; }
    if (e.key === '-') { capeState.zoom = Math.max(1, capeState.zoom - 1); return; }
    if (e.key === 'm' || e.key === 'M') { capeState.mirrorMode = !capeState.mirrorMode; return; }

    const tool = toolShortcuts[e.key.toLowerCase()];
    if (tool) capeState.activeTool = tool;
  }

  function handlePaletteSelect(c: { r: number; g: number; b: number; a: number }) {
    capeState.primaryColor = { ...c };
  }

  onMount(() => {
    loadPreset('mojang');
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="cape-editor">
  <aside class="editor-left">
    <CapeToolBar />
    <CapeHistoryControls />
    <ColorPicker bind:color={capeState.primaryColor} showAlpha={true} />
    <ColorPalette selected={capeState.primaryColor} recentColors={capeState.recentColors} onselect={handlePaletteSelect} />
  </aside>

  <main class="editor-center">
    <div class="viewer-area">
      <CapeEditorViewer3D />
    </div>
    <button class="toggle-2d" onclick={() => show2D = !show2D}>
      {show2D ? 'Masquer texture 2D' : 'Afficher texture 2D'}
    </button>
    {#if show2D}
      <div class="canvas-area-mini"><CapeCanvas /></div>
    {/if}
  </main>

  <aside class="editor-right">
    <CapeViewerControlPanel />
    <CapeImportExportPanel {apiBase} />
    <CapePresetGrid />
  </aside>
</div>

<style>
  .cape-editor { display: grid; grid-template-columns: auto 1fr 300px; gap: 12px; width: 100%; min-height: 600px; }

  .editor-left { display: flex; flex-direction: column; gap: 8px; align-self: start; position: sticky; top: 80px; max-height: calc(100vh - 100px); overflow-y: auto; padding: 8px; }

  .editor-center { display: flex; flex-direction: column; gap: 10px; align-items: center; min-width: 0; }
  .viewer-area { width: 100%; display: flex; justify-content: center; align-items: center; min-height: 500px; background: linear-gradient(180deg, rgba(94,144,255,0.06) 0%, rgba(94,144,255,0.01) 100%); border-radius: var(--radius-lg, 16px); border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); overflow: hidden; }

  .toggle-2d { padding: 6px 16px; border: 1.5px solid var(--line-0, rgba(46, 94, 143, 0.34)); border-radius: var(--radius-sm, 8px); background: var(--surface-1, #edf5fa); color: var(--ink-1, #2d4a65); font-family: 'Chakra Petch', sans-serif; font-size: 0.75rem; font-weight: 600; cursor: pointer; transition: border-color var(--ease, 160ms ease), color var(--ease, 160ms ease); }
  .toggle-2d:hover { border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
  .canvas-area-mini { width: 100%; height: 300px; border-radius: var(--radius-md, 12px); overflow: hidden; border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); }

  .editor-right { display: flex; flex-direction: column; gap: 12px; padding: 12px; background: var(--surface-1, #edf5fa); border-radius: var(--radius-md, 12px); border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); align-self: start; position: sticky; top: 80px; max-height: calc(100vh - 100px); overflow-y: auto; }

  @media (max-width: 1100px) {
    .cape-editor { grid-template-columns: auto 1fr; }
    .editor-right { grid-column: 1 / -1; flex-direction: row; flex-wrap: wrap; position: static; max-height: none; gap: 16px; }
  }
  @media (max-width: 640px) {
    .cape-editor { grid-template-columns: 1fr; padding: 8px; }
    .editor-left { flex-direction: row; flex-wrap: wrap; overflow-x: auto; position: static; max-height: none; }
    .viewer-area { min-height: 350px; }
    .editor-right { flex-direction: column; }
  }
</style>
