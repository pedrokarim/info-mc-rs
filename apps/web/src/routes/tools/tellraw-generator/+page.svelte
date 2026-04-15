<script lang="ts">
  import type { McStyle, StyledChar, EditorLine, CommandType } from '$lib/utils/motd';
  import {
    DEFAULT_STYLE, cloneStyle,
    toPreviewHtml, toJsonComponent, toCommand,
    interpolateColors, rainbowColors,
    parseLegacyString, parseJsonComponent,
  } from '$lib/utils/motd';
  import Card from '$lib/components/ui/Card.svelte';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';
  import McColorPalette from '$lib/components/ui/McColorPalette.svelte';
  import FormatToolbar from '$lib/components/ui/FormatToolbar.svelte';
  import GradientPicker from '$lib/components/ui/GradientPicker.svelte';
  import ColorPicker from '$lib/components/ui/ColorPicker.svelte';
  import MotdEditorInput from '$lib/components/editor/MotdEditorInput.svelte';

  // ── State ──
  let line = $state<EditorLine>({ chars: [] });
  let brush = $state<McStyle>(cloneStyle(DEFAULT_STYLE));
  let selStart = $state(0);
  let selEnd = $state(0);
  let gradientStops = $state(['#FF5555', '#5555FF']);
  let customColor = $state({ r: 255, g: 85, b: 85, a: 255 });

  // ── Command config ──
  let commandType = $state<CommandType>('tellraw');
  let target = $state('@a');
  let copied = $state(false);

  const commandTypes: { label: string; value: CommandType }[] = [
    { label: '/tellraw', value: 'tellraw' },
    { label: '/title', value: 'title' },
    { label: '/title subtitle', value: 'subtitle' },
    { label: '/title actionbar', value: 'actionbar' },
  ];

  const targets = ['@a', '@p', '@s', '@r'];

  // ── Derived ──
  const previewHtml = $derived(toPreviewHtml(line.chars));
  const jsonOutput = $derived(JSON.stringify(toJsonComponent(line.chars), null, 2));
  const commandOutput = $derived(toCommand(line.chars, commandType, target));

  // ── Helpers ──
  function hasSelection() { return selStart !== selEnd; }

  function applyToSelection(fn: (style: McStyle) => void) {
    if (!hasSelection()) return;
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    for (let i = start; i < end && i < line.chars.length; i++) {
      line.chars[i].style = cloneStyle(line.chars[i].style);
      fn(line.chars[i].style);
    }
    line = { ...line };
  }

  function onColorSelect(hex: string) {
    brush = { ...brush, color: hex };
    if (hasSelection()) applyToSelection((s) => { s.color = hex; });
  }

  function onCustomColorChange(color: { r: number; g: number; b: number; a: number }) {
    const hex = '#' + [color.r, color.g, color.b].map(v => v.toString(16).padStart(2, '0')).join('').toUpperCase();
    brush = { ...brush, color: hex };
    if (hasSelection()) applyToSelection((s) => { s.color = hex; });
  }

  type FormatKey = 'bold' | 'italic' | 'underlined' | 'strikethrough' | 'obfuscated';

  function onFormatToggle(format: FormatKey) {
    brush = { ...brush, [format]: !brush[format] };
    if (hasSelection()) {
      const newVal = brush[format];
      applyToSelection((s) => { s[format] = newVal; });
    }
  }

  function onFormatReset() {
    brush = cloneStyle(DEFAULT_STYLE);
    if (hasSelection()) {
      applyToSelection((s) => {
        s.color = null; s.bold = false; s.italic = false;
        s.underlined = false; s.strikethrough = false; s.obfuscated = false;
      });
    }
  }

  function onGradientApply(stops: string[]) {
    if (!hasSelection()) return;
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    const count = end - start;
    if (count <= 0) return;
    const colors = interpolateColors(stops, count);
    for (let i = 0; i < count && start + i < line.chars.length; i++) {
      line.chars[start + i].style = cloneStyle(line.chars[start + i].style);
      line.chars[start + i].style.color = colors[i];
    }
    line = { ...line };
  }

  function onRainbow() {
    if (!hasSelection()) return;
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    const count = end - start;
    if (count <= 0) return;
    const colors = rainbowColors(count);
    for (let i = 0; i < count && start + i < line.chars.length; i++) {
      line.chars[start + i].style = cloneStyle(line.chars[start + i].style);
      line.chars[start + i].style.color = colors[i];
    }
    line = { ...line };
  }

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(commandOutput);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch { /* clipboard fail */ }
  }

  async function copyJson() {
    try {
      await navigator.clipboard.writeText(jsonOutput);
    } catch { /* clipboard fail */ }
  }

  const brushPreviewColor = $derived(brush.color ?? '#AAAAAA');
</script>

<svelte:head>
  <title>Générateur Tellraw Minecraft — MCInfo</title>
  <meta name="description" content="Créez des commandes /tellraw, /title et /actionbar Minecraft avec un éditeur visuel. Couleurs, formats, gradients et export JSON." />
</svelte:head>

<div class="tellraw-page">
  <div class="page-header">
    <h1 class="page-title">Générateur Tellraw</h1>
    <p class="page-subtitle">Créez des commandes /tellraw, /title et /actionbar avec un éditeur visuel</p>
  </div>

  <!-- Command config -->
  <div class="command-config">
    <div class="config-group">
      <span class="section-label">Commande</span>
      <div class="pill-row">
        {#each commandTypes as ct}
          <button class="pill" class:active={commandType === ct.value} onclick={() => { commandType = ct.value; }}>
            {ct.label}
          </button>
        {/each}
      </div>
    </div>
    <div class="config-group">
      <span class="section-label">Cible</span>
      <div class="pill-row">
        {#each targets as t}
          <button class="pill pill--sm" class:active={target === t} onclick={() => { target = t; }}>
            {t}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <!-- Main grid -->
  <div class="main-grid">
    <!-- Tools -->
    <Card variant="elevated" padding="md">
      <div class="tools-panel">
        <McColorPalette edition="java" selected={brush.color} onselect={onColorSelect} />
        <div class="custom-color-section">
          <span class="section-label">Couleur hex</span>
          <ColorPicker bind:color={customColor} showAlpha={false} onchange={onCustomColorChange} />
        </div>
        <FormatToolbar
          edition="java"
          activeFormats={{ bold: brush.bold, italic: brush.italic, underlined: brush.underlined, strikethrough: brush.strikethrough, obfuscated: brush.obfuscated }}
          ontoggle={onFormatToggle}
          onreset={onFormatReset}
        />
        <GradientPicker bind:stops={gradientStops} onapply={onGradientApply} onrainbow={onRainbow} />
      </div>
    </Card>

    <!-- Right column -->
    <div class="right-col">
      <!-- Preview -->
      <Card variant="elevated" padding="md">
        <div class="preview-section">
          <span class="section-label">Aperçu</span>
          <MotdBlock html={previewHtml} fallback="Tapez votre texte pour voir l'aperçu..." />
          <div class="brush-indicator">
            <span class="section-label">Style actif</span>
            <div class="brush-preview" style="color:{brushPreviewColor}">
              <span style="{brush.bold ? 'font-weight:bold;' : ''}{brush.italic ? 'font-style:italic;' : ''}{brush.underlined ? 'text-decoration:underline;' : ''}{brush.strikethrough ? 'text-decoration:line-through;' : ''}">
                {brush.obfuscated ? '????' : 'Abc123'}
              </span>
            </div>
          </div>
        </div>
      </Card>

      <!-- Editor (single line, no char limit) -->
      <Card variant="elevated" padding="md">
        <div class="editor-area">
          <span class="section-label">Éditeur</span>
          <MotdEditorInput
            bind:line={line}
            {brush}
            label="Texte"
            onselectionchange={(start, end) => { selStart = start; selEnd = end; }}
          />
        </div>
      </Card>

      <!-- Command output -->
      <Card variant="elevated" padding="md">
        <div class="output-section">
          <span class="section-label">Commande</span>
          <div class="command-box">
            <code class="command-text">{commandOutput}</code>
            <button class="copy-btn" onclick={copyCommand}>
              {copied ? 'Copié !' : 'Copier'}
            </button>
          </div>
        </div>
      </Card>

      <!-- JSON output -->
      <Card variant="elevated" padding="md">
        <div class="output-section">
          <div class="output-header">
            <span class="section-label">JSON Component</span>
            <button class="copy-btn copy-btn--sm" onclick={copyJson}>Copier</button>
          </div>
          <pre class="json-box">{jsonOutput}</pre>
        </div>
      </Card>
    </div>
  </div>
</div>

<style>
  .tellraw-page {
    width: var(--layout-width, min(1160px, calc(100% - 2rem)));
    margin: 0 auto;
    padding: 2rem 0 4rem;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .page-header { display: flex; flex-direction: column; gap: 0.3rem; }
  .page-title { font-family: 'Teko', sans-serif; font-size: 2.4rem; font-weight: 700; line-height: 1; color: var(--ink-0, #0f253a); margin: 0; }
  .page-subtitle { font-size: 0.85rem; color: var(--ink-2, #5a7894); margin: 0; }

  /* ── Command config ── */
  .command-config {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .config-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .pill-row {
    display: flex;
    gap: 0;
    border: 1px solid rgba(70, 113, 166, 0.35);
    border-radius: 6px;
    overflow: hidden;
  }

  .pill {
    padding: 0.4rem 0.8rem;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.75rem;
    font-weight: 600;
    border: none;
    background: rgba(255, 255, 255, 0.5);
    color: var(--ink-1, #2d4a65);
    cursor: pointer;
    transition: background 120ms ease, color 120ms ease;
  }

  .pill:not(:last-child) { border-right: 1px solid rgba(70, 113, 166, 0.25); }
  .pill:hover { background: rgba(94, 144, 255, 0.08); }
  .pill.active { background: var(--blue-0, #5e90ff); color: #fff; }
  .pill--sm { padding: 0.35rem 0.6rem; font-size: 0.72rem; }

  /* ── Main grid ── */
  .main-grid { display: grid; grid-template-columns: 260px 1fr; gap: 1.2rem; align-items: start; }
  @media (max-width: 768px) { .main-grid { grid-template-columns: 1fr; } }

  .tools-panel { display: flex; flex-direction: column; gap: 16px; }
  .custom-color-section { display: flex; flex-direction: column; gap: 6px; }
  .right-col { display: flex; flex-direction: column; gap: 1.2rem; }

  /* ── Preview ── */
  .preview-section { display: flex; flex-direction: column; gap: 10px; }
  .brush-indicator { display: flex; flex-direction: column; gap: 4px; }
  .brush-preview {
    font-family: 'Minecraft', 'JetBrains Mono', monospace;
    font-size: 0.85rem;
    background: #2b2b2b;
    border-radius: 4px;
    padding: 4px 10px;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, 0.4);
  }

  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .editor-area { display: flex; flex-direction: column; gap: 12px; }

  /* ── Command output ── */
  .output-section { display: flex; flex-direction: column; gap: 8px; }
  .output-header { display: flex; justify-content: space-between; align-items: center; }

  .command-box {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #1a1a2e;
    border-radius: 6px;
    padding: 10px 12px;
    overflow-x: auto;
  }

  .command-text {
    flex: 1;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    color: #e0e0e0;
    white-space: nowrap;
    word-break: break-all;
  }

  .copy-btn {
    padding: 0.35rem 0.8rem;
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: rgba(94, 144, 255, 0.2);
    color: #fff;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.72rem;
    font-weight: 600;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 120ms ease;
  }

  .copy-btn:hover { background: rgba(94, 144, 255, 0.4); }
  .copy-btn--sm { padding: 0.25rem 0.6rem; font-size: 0.65rem; background: rgba(94, 144, 255, 0.1); border-color: rgba(70, 113, 166, 0.3); color: var(--ink-1, #2d4a65); }
  .copy-btn--sm:hover { background: rgba(94, 144, 255, 0.2); }

  .json-box {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.72rem;
    background: #1a1a2e;
    color: #c0c0c0;
    border-radius: 6px;
    padding: 10px 12px;
    margin: 0;
    overflow-x: auto;
    max-height: 200px;
    overflow-y: auto;
    white-space: pre;
  }
</style>
