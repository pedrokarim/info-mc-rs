<script lang="ts">
  import { STRUCTURE_PRESETS } from '$lib/data/structure-presets';

  let {
    onselect,
  }: {
    onselect?: (path: string) => void;
  } = $props();

  let expandedFolders = $state<Set<string>>(new Set());

  function toggleFolder(name: string) {
    if (expandedFolders.has(name)) {
      expandedFolders.delete(name);
    } else {
      expandedFolders.add(name);
    }
    expandedFolders = new Set(expandedFolders);
  }
</script>

<div class="preset-tree">
  <span class="tree-title">Structures Minecraft</span>
  <div class="tree-list">
    {#each STRUCTURE_PRESETS as folder}
      <button class="folder-header" onclick={() => toggleFolder(folder.name)}>
        <span class="folder-arrow" class:open={expandedFolders.has(folder.name)}>&#9654;</span>
        <span class="folder-icon">{folder.icon}</span>
        <span class="folder-label">{folder.label}</span>
        <span class="folder-count">{folder.files.length}</span>
      </button>

      {#if expandedFolders.has(folder.name)}
        <div class="folder-children">
          {#each folder.files as file}
            <button class="file-item" onclick={() => onselect?.(`/structures/${file.path}`)}>
              <span class="file-icon">📄</span>
              <span class="file-name">{file.name}</span>
            </button>
          {/each}
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .preset-tree { display: flex; flex-direction: column; gap: 6px; }
  .tree-title { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  .tree-list { display: flex; flex-direction: column; gap: 1px; max-height: 300px; overflow-y: auto; }

  .folder-header {
    display: flex; align-items: center; gap: 6px;
    width: 100%; padding: 5px 6px; border: none; border-radius: 4px;
    background: transparent; color: var(--ink-0, #0f253a);
    font-family: 'Chakra Petch', sans-serif; font-size: 0.78rem; font-weight: 600;
    cursor: pointer; text-align: left;
    transition: background 100ms ease;
  }
  .folder-header:hover { background: rgba(94, 144, 255, 0.06); }

  .folder-arrow {
    display: inline-block; width: 10px; font-size: 0.5rem;
    color: var(--ink-2, #5a7894); transition: transform 120ms ease;
    flex-shrink: 0; text-align: center;
  }
  .folder-arrow.open { transform: rotate(90deg); }

  .folder-icon { font-size: 0.85rem; flex-shrink: 0; }
  .folder-label { flex: 1; }
  .folder-count { font-size: 0.65rem; color: var(--ink-2, #5a7894); font-family: 'JetBrains Mono', monospace; }

  .folder-children {
    display: flex; flex-direction: column; gap: 1px;
    margin-left: 18px; padding-left: 8px;
    border-left: 1px solid var(--line-0, rgba(46, 94, 143, 0.2));
  }

  .file-item {
    display: flex; align-items: center; gap: 5px;
    width: 100%; padding: 3px 6px; border: none; border-radius: 3px;
    background: transparent; color: var(--ink-1, #2d4a65);
    font-family: 'JetBrains Mono', monospace; font-size: 0.7rem;
    cursor: pointer; text-align: left;
    transition: background 100ms ease, color 100ms ease;
  }
  .file-item:hover { background: rgba(94, 144, 255, 0.08); color: var(--blue-0, #5e90ff); }

  .file-icon { font-size: 0.7rem; flex-shrink: 0; }
  .file-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
