<script lang="ts">
  import type { EditorState } from '$lib/utils/motd';
  import { toLegacy, toJsonComponent, toMiniMessage, DEFAULT_STYLE } from '$lib/utils/motd';
  import type { StyledChar } from '$lib/utils/motd';
  import Tabs from '$lib/components/ui/Tabs.svelte';

  let {
    lines,
  }: {
    lines: EditorState;
  } = $props();

  let selectedTab = $state('legacy');
  let copied = $state(false);

  const tabs = [
    { label: '§ Legacy', value: 'legacy' },
    { label: '& Ampersand', value: 'ampersand' },
    { label: 'JSON Component', value: 'json' },
    { label: 'MiniMessage', value: 'minimessage' },
  ];

  function allChars(): StyledChar[] {
    const result: StyledChar[] = [...lines[0].chars];
    if (lines[1].chars.length > 0) {
      result.push({ char: '\n', style: { ...DEFAULT_STYLE } });
      result.push(...lines[1].chars);
    }
    return result;
  }

  const exportLegacy = $derived(
    lines.map((l) => toLegacy(l.chars, '§')).join('\n'),
  );

  const exportAmpersand = $derived(
    lines.map((l) => toLegacy(l.chars, '&')).join('\n'),
  );

  const exportJson = $derived(
    JSON.stringify(toJsonComponent(allChars()), null, 2),
  );

  const exportMiniMessage = $derived(
    lines.map((l) => toMiniMessage(l.chars)).join('<newline>'),
  );

  const currentExport = $derived(
    selectedTab === 'legacy' ? exportLegacy
    : selectedTab === 'ampersand' ? exportAmpersand
    : selectedTab === 'json' ? exportJson
    : exportMiniMessage,
  );

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(currentExport);
      copied = true;
      setTimeout(() => { copied = false; }, 1500);
    } catch {
      // fallback
      const ta = document.createElement('textarea');
      ta.value = currentExport;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
      copied = true;
      setTimeout(() => { copied = false; }, 1500);
    }
  }
</script>

<div class="export-panel">
  <div class="export-header">
    <span class="export-label">Export</span>
    <Tabs items={tabs} bind:selected={selectedTab} />
  </div>

  <div class="code-block">
    <pre class="code-content">{currentExport || '(vide)'}</pre>

    <button class="copy-btn" onclick={copyToClipboard} aria-label="Copier">
      {#if copied}
        Copie !
      {:else}
        Copier
      {/if}
    </button>
  </div>
</div>

<style>
  .export-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .export-header {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .export-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .code-block {
    position: relative;
    background: #1a1a2e;
    border: 2px solid #0d0d1a;
    border-radius: 8px;
    padding: 12px 16px;
    padding-right: 80px;
    overflow-x: auto;
  }

  .code-content {
    font-family: 'Minecraft', 'JetBrains Mono', 'Courier New', monospace;
    font-size: 0.75rem;
    line-height: 1.5;
    color: #e6edf3;
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .copy-btn {
    position: absolute;
    top: 8px;
    right: 8px;
    padding: 4px 12px;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(255, 255, 255, 0.08);
    color: #e6edf3;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.68rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 160ms ease;
  }

  .copy-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }
</style>
