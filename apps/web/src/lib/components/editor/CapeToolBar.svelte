<script lang="ts">
  import { capeState } from '$lib/stores/cape-editor.svelte';
  import type { Tool } from '$lib/stores/cape-editor.svelte';

  const tools: { id: Tool; label: string; shortcut: string; icon: string }[] = [
    { id: 'pencil', label: 'Crayon', shortcut: 'P', icon: 'M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25ZM20.71 7.04a1 1 0 0 0 0-1.41l-2.34-2.34a1 1 0 0 0-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83Z' },
    { id: 'eraser', label: 'Gomme', shortcut: 'E', icon: 'M18.85 10.39l-5.24-5.24a2 2 0 0 0-2.83 0L3.15 12.78a2 2 0 0 0 0 2.83l3.54 3.54a2 2 0 0 0 1.41.59H20v-2h-7.17l5.02-5.02a2 2 0 0 0 0-2.83ZM8.1 17.74l-3.54-3.54 4.24-4.24 3.54 3.54-4.24 4.24Z' },
    { id: 'eyedropper', label: 'Pipette', shortcut: 'I', icon: 'M20.71 5.63l-2.34-2.34a1 1 0 0 0-1.41 0l-3.12 3.12-1.42-1.42-1.41 1.42 1.42 1.42-8.9 8.9a1 1 0 0 0-.29.7V21h3.47a1 1 0 0 0 .71-.29l8.9-8.9 1.41 1.41 1.42-1.41-1.42-1.42 3.12-3.12a1 1 0 0 0 0-1.41ZM6.73 19H5v-1.73L13.9 8.37l1.73 1.73L6.73 19Z' },
    { id: 'fill', label: 'Remplissage', shortcut: 'G', icon: 'M16.56 8.94L8.32.7a1 1 0 0 0-1.41 0L5.17 2.44a1 1 0 0 0 0 1.42L12.7 11.4 4.26 19.83a2 2 0 0 0 0 2.83l.14.14a2 2 0 0 0 2.83 0L16.56 13.47a2 2 0 0 0 0-2.83L16.56 8.94ZM19 17c-1.5 2-3 3.5-3 5a3 3 0 0 0 6 0c0-1.5-1.5-3-3-5Z' },
    { id: 'line', label: 'Ligne', shortcut: 'L', icon: 'M4 20L20 4' },
    { id: 'rect', label: 'Rectangle', shortcut: 'R', icon: 'M3 5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5Zm2 0v14h14V5H5Z' },
    { id: 'pan', label: 'Déplacer', shortcut: 'H', icon: 'M12 2l-4 4h3v5H6V8l-4 4 4 4v-3h5v5H8l4 4 4-4h-3v-5h5v3l4-4-4-4v3h-5V6h3l-4-4Z' },
  ];
</script>

<div class="toolbar" role="toolbar" aria-label="Outils de dessin">
  {#each tools as tool}
    <button class="tool-btn" class:active={capeState.activeTool === tool.id} onclick={() => capeState.activeTool = tool.id} title="{tool.label} ({tool.shortcut})" aria-pressed={capeState.activeTool === tool.id}>
      <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
        {#if tool.id === 'line'}<path d={tool.icon} stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" />{:else}<path d={tool.icon} />{/if}
      </svg>
      <span class="tool-shortcut">{tool.shortcut}</span>
    </button>
  {/each}
  <div class="toolbar-divider"></div>
  <button class="tool-btn" class:active={capeState.mirrorMode} onclick={() => capeState.mirrorMode = !capeState.mirrorMode} title="Miroir (M)" aria-pressed={capeState.mirrorMode}>
    <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2v20M9 6l-6 6 6 6V6Zm6 0v12l6-6-6-6Z" /></svg>
    <span class="tool-shortcut">M</span>
  </button>
  {#if capeState.activeTool === 'rect'}
    <button class="tool-btn tool-btn--sub" onclick={() => capeState.rectFillMode = capeState.rectFillMode === 'filled' ? 'outline' : 'filled'} title={capeState.rectFillMode === 'filled' ? 'Rempli' : 'Contour'}>
      <svg width="18" height="18" viewBox="0 0 24 24">{#if capeState.rectFillMode === 'filled'}<rect x="3" y="3" width="18" height="18" rx="2" fill="currentColor" />{:else}<rect x="3" y="3" width="18" height="18" rx="2" fill="none" stroke="currentColor" stroke-width="2" />{/if}</svg>
    </button>
  {/if}
</div>

<style>
  .toolbar { display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px; padding: 8px; background: var(--surface-1, #edf5fa); border-radius: var(--radius-md, 12px); border: 1px solid var(--line-0, rgba(46, 94, 143, 0.34)); }
  .toolbar-divider { grid-column: 1 / -1; }
  .tool-btn { position: relative; display: flex; align-items: center; justify-content: center; width: 40px; height: 40px; border: 2px solid transparent; border-radius: var(--radius-sm, 8px); background: transparent; color: var(--ink-1, #2d4a65); cursor: pointer; transition: background var(--ease, 160ms ease), border-color var(--ease, 160ms ease), color var(--ease, 160ms ease); padding: 0; font-family: inherit; }
  .tool-btn:hover { background: rgba(94, 144, 255, 0.08); color: var(--blue-0, #5e90ff); }
  .tool-btn.active { background: rgba(94, 144, 255, 0.15); border-color: var(--blue-0, #5e90ff); color: var(--blue-0, #5e90ff); }
  .tool-shortcut { position: absolute; bottom: 1px; right: 3px; font-size: 0.55rem; font-weight: 700; font-family: 'JetBrains Mono', monospace; opacity: 0.5; line-height: 1; }
  .tool-btn--sub { width: 36px; height: 36px; align-self: center; }
  .toolbar-divider { height: 1px; margin: 4px 0; background: var(--line-0, rgba(46, 94, 143, 0.34)); }
</style>
