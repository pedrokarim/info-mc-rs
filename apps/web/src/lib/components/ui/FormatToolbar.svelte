<script lang="ts">
  import type { Edition } from '$lib/utils/motd';
  import Tooltip from './Tooltip.svelte';

  type FormatKey = 'bold' | 'italic' | 'underlined' | 'strikethrough' | 'obfuscated';

  let {
    edition = 'java',
    activeFormats = { bold: false, italic: false, underlined: false, strikethrough: false, obfuscated: false },
    ontoggle,
    onreset,
  }: {
    edition?: Edition;
    activeFormats?: Record<FormatKey, boolean>;
    ontoggle?: (format: FormatKey) => void;
    onreset?: () => void;
  } = $props();

  interface FormatDef {
    key: FormatKey;
    label: string;
    icon: string;
    iconStyle: string;
    javaOnly: boolean;
    code: string;
  }

  const formats: FormatDef[] = [
    { key: 'bold', label: 'Gras', icon: 'B', iconStyle: 'font-weight:900', javaOnly: false, code: '§l' },
    { key: 'italic', label: 'Italique', icon: 'I', iconStyle: 'font-style:italic', javaOnly: false, code: '§o' },
    { key: 'underlined', label: 'Souligné', icon: 'U', iconStyle: 'text-decoration:underline', javaOnly: true, code: '§n' },
    { key: 'strikethrough', label: 'Barré', icon: 'S', iconStyle: 'text-decoration:line-through', javaOnly: true, code: '§m' },
    { key: 'obfuscated', label: 'Obfusqué', icon: '?', iconStyle: 'opacity:0.7', javaOnly: false, code: '§k' },
  ];

  const visibleFormats = $derived(
    edition === 'bedrock'
      ? formats.filter((f) => !f.javaOnly)
      : formats,
  );
</script>

<div class="format-toolbar">
  <span class="toolbar-label">Formatage</span>

  <div class="toolbar-row">
    {#each visibleFormats as fmt (fmt.key)}
      <Tooltip text="{fmt.label} ({fmt.code})">
        <button
          class="fmt-btn"
          class:active={activeFormats[fmt.key]}
          onclick={() => ontoggle?.(fmt.key)}
          aria-pressed={activeFormats[fmt.key]}
          aria-label={fmt.label}
        >
          <span style={fmt.iconStyle}>{fmt.icon}</span>
        </button>
      </Tooltip>
    {/each}

    <span class="separator"></span>

    <Tooltip text="Réinitialiser (§r)">
      <button class="fmt-btn reset-btn" onclick={() => onreset?.()} aria-label="Réinitialiser">
        R
      </button>
    </Tooltip>
  </div>
</div>

<style>
  .format-toolbar {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .toolbar-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .toolbar-row {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .separator {
    width: 1px;
    height: 24px;
    background: var(--line-0, rgba(46, 94, 143, 0.34));
    margin: 0 4px;
  }

  .fmt-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa);
    color: var(--ink-0, #0f253a);
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.8rem;
    font-weight: 700;
    cursor: pointer;
    padding: 0;
    transition:
      border-color 160ms ease,
      background 160ms ease,
      color 160ms ease,
      transform 160ms ease;
  }

  .fmt-btn:hover {
    border-color: var(--blue-0, #5e90ff);
    transform: scale(1.05);
  }

  .fmt-btn.active {
    background: var(--blue-0, #5e90ff);
    border-color: var(--blue-1, #345fcd);
    color: #fff;
  }

  .fmt-btn:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 2px;
  }

  .reset-btn {
    color: var(--danger, #b83b3b);
    border-color: rgba(184, 59, 59, 0.3);
  }

  .reset-btn:hover {
    background: rgba(184, 59, 59, 0.1);
    border-color: var(--danger, #b83b3b);
  }
</style>
