<script lang="ts">
  import type { Edition, McStyle, EditorLine, EditorState, StyledChar } from '$lib/utils/motd';
  import {
    DEFAULT_STYLE,
    cloneStyle,
    toPreviewHtml,
    interpolateColors,
    rainbowColors,
    parseLegacyString,
    parseJsonComponent,
  } from '$lib/utils/motd';
  import Card from '$lib/components/ui/Card.svelte';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';
  import McColorPalette from '$lib/components/ui/McColorPalette.svelte';
  import FormatToolbar from '$lib/components/ui/FormatToolbar.svelte';
  import GradientPicker from '$lib/components/ui/GradientPicker.svelte';
  import ColorPicker from '$lib/components/ui/ColorPicker.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import MotdEditorInput from '$lib/components/editor/MotdEditorInput.svelte';
  import ExportPanel from '$lib/components/editor/ExportPanel.svelte';
  import MotdPresetGrid from '$lib/components/ui/MotdPresetGrid.svelte';
  import type { MotdPreset } from '$lib/utils/motd';

  // ── Edition toggle ──────────────────────────────────────────────────
  let edition = $state<Edition>('java');

  // ── Editor state ────────────────────────────────────────────────────
  let lines = $state<EditorState>([{ chars: [] }, { chars: [] }]);

  // ── Current brush ───────────────────────────────────────────────────
  let brush = $state<McStyle>(cloneStyle(DEFAULT_STYLE));

  // ── Selection tracking ──────────────────────────────────────────────
  let activeLine = $state(0);
  let selStart = $state(0);
  let selEnd = $state(0);

  // ── Gradient state ──────────────────────────────────────────────────
  let gradientStops = $state(['#FF5555', '#5555FF']);

  // ── Import modal ────────────────────────────────────────────────────
  let importOpen = $state(false);
  let importText = $state('');
  let importFormat = $state<'legacy' | 'json'>('legacy');

  // ── Custom color picker state ───────────────────────────────────────
  let customColor = $state({ r: 255, g: 85, b: 85, a: 255 });

  // ── Derived: preview HTML ───────────────────────────────────────────
  const previewHtml1 = $derived(toPreviewHtml(lines[0].chars));
  const previewHtml2 = $derived(toPreviewHtml(lines[1].chars));
  const combinedPreview = $derived(
    previewHtml1 + (lines[1].chars.length > 0 ? '<br>' + previewHtml2 : ''),
  );

  // ── Helpers ─────────────────────────────────────────────────────────

  function hasSelection(): boolean {
    return selStart !== selEnd;
  }

  function applyToSelection(fn: (style: McStyle) => void) {
    if (!hasSelection()) return;
    const line = lines[activeLine];
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    for (let i = start; i < end && i < line.chars.length; i++) {
      line.chars[i].style = cloneStyle(line.chars[i].style);
      fn(line.chars[i].style);
    }
    lines = [...lines] as EditorState;
  }

  // ── Color selection ─────────────────────────────────────────────────

  function onColorSelect(hex: string) {
    brush = { ...brush, color: hex };
    if (hasSelection()) {
      applyToSelection((s) => { s.color = hex; });
    }
  }

  function onCustomColorChange(color: { r: number; g: number; b: number; a: number }) {
    const hex =
      '#' +
      [color.r, color.g, color.b]
        .map((v) => v.toString(16).padStart(2, '0'))
        .join('')
        .toUpperCase();
    brush = { ...brush, color: hex };
    if (hasSelection()) {
      applyToSelection((s) => { s.color = hex; });
    }
  }

  // ── Format toggle ──────────────────────────────────────────────────

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
        s.color = null;
        s.bold = false;
        s.italic = false;
        s.underlined = false;
        s.strikethrough = false;
        s.obfuscated = false;
      });
    }
  }

  // ── Gradient / Rainbow apply ────────────────────────────────────────

  function onGradientApply(stops: string[]) {
    if (!hasSelection()) return;
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    const count = end - start;
    if (count <= 0) return;

    const colors = interpolateColors(stops, count);
    const line = lines[activeLine];
    for (let i = 0; i < count && start + i < line.chars.length; i++) {
      line.chars[start + i].style = cloneStyle(line.chars[start + i].style);
      line.chars[start + i].style.color = colors[i];
    }
    lines = [...lines] as EditorState;
  }

  function onRainbow() {
    if (!hasSelection()) return;
    const start = Math.min(selStart, selEnd);
    const end = Math.max(selStart, selEnd);
    const count = end - start;
    if (count <= 0) return;

    const colors = rainbowColors(count);
    const line = lines[activeLine];
    for (let i = 0; i < count && start + i < line.chars.length; i++) {
      line.chars[start + i].style = cloneStyle(line.chars[start + i].style);
      line.chars[start + i].style.color = colors[i];
    }
    lines = [...lines] as EditorState;
  }

  // ── Edition switch ──────────────────────────────────────────────────

  function switchEdition(ed: Edition) {
    edition = ed;
    if (ed === 'bedrock') {
      // Strip Java-only formats
      for (const line of lines) {
        for (const ch of line.chars) {
          ch.style.underlined = false;
          ch.style.strikethrough = false;
        }
      }
      brush.underlined = false;
      brush.strikethrough = false;
      lines = [...lines] as EditorState;
    }
  }

  // ── Import ──────────────────────────────────────────────────────────

  function doImport() {
    const text = importText.trim();
    if (!text) return;

    let chars: StyledChar[];
    if (importFormat === 'json') {
      chars = parseJsonComponent(text);
    } else {
      chars = parseLegacyString(text);
    }

    // Split into 2 lines at newline
    const newlineIdx = chars.findIndex((c) => c.char === '\n');
    if (newlineIdx >= 0) {
      lines[0].chars = chars.slice(0, newlineIdx);
      lines[1].chars = chars.slice(newlineIdx + 1).filter((c) => c.char !== '\n');
    } else {
      lines[0].chars = chars;
      lines[1].chars = [];
    }
    lines = [...lines] as EditorState;

    importOpen = false;
    importText = '';
  }

  // ── Presets ─────────────────────────────────────────────────────────

  function loadPreset(preset: MotdPreset) {
    const chars = parseLegacyString(preset.motd);
    const newlineIdx = chars.findIndex((c) => c.char === '\n');
    if (newlineIdx >= 0) {
      lines[0].chars = chars.slice(0, newlineIdx);
      lines[1].chars = chars.slice(newlineIdx + 1).filter((c) => c.char !== '\n');
    } else {
      lines[0].chars = chars;
      lines[1].chars = [];
    }
    lines = [...lines] as EditorState;
  }

  // ── Brush preview color ─────────────────────────────────────────────
  const brushPreviewColor = $derived(brush.color ?? '#AAAAAA');
</script>

<svelte:head>
  <title>Editeur MOTD Minecraft — MCInfo</title>
  <meta name="description" content="Créez et personnalisez votre MOTD Minecraft avec support Java Edition, Bedrock Edition, gradients, MiniMessage (Kyori Adventure) et export multi-format." />
</svelte:head>

<div class="motd-editor-page">
  <!-- Header -->
  <div class="page-header">
    <nav class="breadcrumb" aria-label="Breadcrumb">
      <a href="/">Accueil</a>
      <span class="sep">/</span>
      <span>Outils</span>
      <span class="sep">/</span>
      <span class="current">Editeur MOTD</span>
    </nav>

    <h1 class="page-title">Editeur MOTD Minecraft</h1>
    <p class="page-subtitle">
      Créez votre MOTD avec support complet Java & Bedrock, gradients, MiniMessage et export multi-format.
    </p>
  </div>

  <!-- Edition toggle -->
  <div class="edition-toggle">
    <button
      class="edition-btn"
      class:active={edition === 'java'}
      onclick={() => switchEdition('java')}
    >
      Java Edition
    </button>
    <button
      class="edition-btn"
      class:active={edition === 'bedrock'}
      onclick={() => switchEdition('bedrock')}
    >
      Bedrock Edition
    </button>
  </div>

  <!-- Main layout: tools (left) + editor/preview/export (right) -->
  <div class="main-grid">
    <!-- Tools panel -->
    <Card variant="elevated" padding="md">
      <div class="tools-panel">
        <McColorPalette
          {edition}
          selected={brush.color}
          onselect={(hex) => onColorSelect(hex)}
        />

        {#if edition === 'java'}
          <div class="custom-color-section">
            <span class="section-label">Couleur hex personnalisee</span>
            <ColorPicker
              bind:color={customColor}
              showAlpha={false}
              onchange={onCustomColorChange}
            />
          </div>
        {/if}

        <FormatToolbar
          {edition}
          activeFormats={{
            bold: brush.bold,
            italic: brush.italic,
            underlined: brush.underlined,
            strikethrough: brush.strikethrough,
            obfuscated: brush.obfuscated,
          }}
          ontoggle={onFormatToggle}
          onreset={onFormatReset}
        />

        <GradientPicker
          bind:stops={gradientStops}
          onapply={onGradientApply}
          onrainbow={onRainbow}
        />
      </div>
    </Card>

    <!-- Right column: preview + editor + export -->
    <div class="right-col">
      <!-- Preview -->
      <Card variant="elevated" padding="md">
        <div class="preview-section">
          <span class="section-label">Apercu en direct</span>
          <MotdBlock html={combinedPreview} fallback="Tapez votre MOTD pour voir l'apercu..." />

          <div class="brush-indicator">
            <span class="section-label">Style actif</span>
            <div class="brush-preview" style="color:{brushPreviewColor}">
              <span
                style="
                  {brush.bold ? 'font-weight:bold;' : ''}
                  {brush.italic ? 'font-style:italic;' : ''}
                  {brush.underlined ? 'text-decoration:underline;' : ''}
                  {brush.strikethrough ? 'text-decoration:line-through;' : ''}
                "
              >
                {brush.obfuscated ? '????' : 'Abc123'}
              </span>
            </div>
          </div>
        </div>
      </Card>

      <!-- Editor -->
      <Card variant="elevated" padding="md">
        <div class="editor-area">
          <span class="section-label">Editeur</span>
          <MotdEditorInput
            bind:line={lines[0]}
            {brush}
            label="Ligne 1"
            onselectionchange={(start, end) => { activeLine = 0; selStart = start; selEnd = end; }}
          />
          <MotdEditorInput
            bind:line={lines[1]}
            {brush}
            label="Ligne 2"
            onselectionchange={(start, end) => { activeLine = 1; selStart = start; selEnd = end; }}
          />
        </div>
      </Card>

      <!-- Export -->
      <Card variant="elevated" padding="md">
        <ExportPanel {lines} />
      </Card>

      <!-- Presets -->
      <Card variant="elevated" padding="md">
        <MotdPresetGrid {edition} onselect={loadPreset} />
      </Card>
    </div>
  </div>

  <!-- Import button -->
  <div class="import-row">
    <button class="import-btn" onclick={() => { importOpen = true; }}>
      Importer un MOTD existant
    </button>
  </div>

  <!-- Import modal -->
  <Modal bind:open={importOpen} title="Importer un MOTD">
    <div class="import-form">
      <div class="import-format-row">
        <label class="import-radio">
          <input type="radio" bind:group={importFormat} value="legacy" />
          <span>Legacy (§ / &)</span>
        </label>
        <label class="import-radio">
          <input type="radio" bind:group={importFormat} value="json" />
          <span>JSON Component</span>
        </label>
      </div>
      <textarea
        class="import-textarea"
        bind:value={importText}
        placeholder={importFormat === 'json'
          ? '{"text":"","extra":[{"text":"Hello","color":"gold","bold":true}]}'
          : '§6§lHypixel §eNetwork'}
        rows={4}
      ></textarea>
      <button class="import-submit" onclick={doImport}>Importer</button>
    </div>
  </Modal>
</div>

<style>
  .motd-editor-page {
    width: var(--layout-width, min(1160px, calc(100% - 2rem)));
    margin: 0 auto;
    padding: 2rem 0 4rem;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  /* ── Header ────────────────────────────────────────────────────────── */

  .page-header {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .breadcrumb {
    font-size: 0.72rem;
    color: var(--ink-2, #5a7894);
    display: flex;
    gap: 0.3em;
    align-items: center;
  }

  .breadcrumb a {
    color: var(--blue-0, #5e90ff);
    text-decoration: none;
  }
  .breadcrumb a:hover { text-decoration: underline; }

  .breadcrumb .sep { opacity: 0.4; }
  .breadcrumb .current { font-weight: 600; color: var(--ink-1, #2d4a65); }

  .page-title {
    font-family: 'Teko', sans-serif;
    font-size: 2.4rem;
    font-weight: 700;
    line-height: 1;
    color: var(--ink-0, #0f253a);
    margin: 0;
  }

  .page-subtitle {
    font-size: 0.85rem;
    color: var(--ink-2, #5a7894);
    margin: 0;
  }

  /* ── Edition toggle ────────────────────────────────────────────────── */

  .edition-toggle {
    display: flex;
    gap: 0;
    border-radius: var(--radius-sm, 8px);
    overflow: hidden;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    width: fit-content;
  }

  .edition-btn {
    padding: 0.45rem 1.2rem;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.82rem;
    font-weight: 600;
    border: none;
    cursor: pointer;
    background: var(--surface-1, #edf5fa);
    color: var(--ink-1, #2d4a65);
    transition: background 160ms ease, color 160ms ease;
  }

  .edition-btn.active {
    background: var(--blue-0, #5e90ff);
    color: #fff;
  }

  .edition-btn:not(.active):hover {
    background: var(--surface-2, #d8ebf6);
  }

  /* ── Main grid ─────────────────────────────────────────────────────── */

  .main-grid {
    display: grid;
    grid-template-columns: 260px 1fr;
    gap: 1.2rem;
    align-items: start;
  }

  @media (max-width: 768px) {
    .main-grid {
      grid-template-columns: 1fr;
    }
  }

  /* ── Tools panel ───────────────────────────────────────────────────── */

  .tools-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .custom-color-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  /* ── Right column ──────────────────────────────────────────────────── */

  .right-col {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  /* ── Preview panel ─────────────────────────────────────────────────── */

  .preview-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .brush-indicator {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .brush-preview {
    font-family: 'Minecraft', 'JetBrains Mono', monospace;
    font-size: 0.85rem;
    background: #2b2b2b;
    border-radius: 4px;
    padding: 4px 10px;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, 0.4);
  }

  /* ── Section label (reused) ────────────────────────────────────────── */

  .section-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  /* ── Editor area ───────────────────────────────────────────────────── */

  .editor-area {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  /* ── Import ────────────────────────────────────────────────────────── */

  .import-row {
    display: flex;
    justify-content: center;
  }

  .import-btn {
    padding: 0.5rem 1.5rem;
    border-radius: var(--radius-sm, 8px);
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    background: var(--surface-1, #edf5fa);
    color: var(--ink-1, #2d4a65);
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    transition: border-color 160ms ease, background 160ms ease;
  }

  .import-btn:hover {
    border-color: var(--blue-0, #5e90ff);
    background: var(--surface-2, #d8ebf6);
  }

  .import-form {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .import-format-row {
    display: flex;
    gap: 16px;
  }

  .import-radio {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--ink-1, #2d4a65);
    cursor: pointer;
  }

  .import-textarea {
    width: 100%;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    padding: 10px 12px;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-sm, 8px);
    background: var(--surface-1, #edf5fa);
    color: var(--ink-0, #0f253a);
    resize: vertical;
    outline: none;
    box-sizing: border-box;
  }

  .import-textarea:focus {
    border-color: var(--blue-0, #5e90ff);
  }

  .import-submit {
    align-self: flex-end;
    padding: 0.5rem 1.5rem;
    border-radius: var(--radius-sm, 8px);
    border: none;
    background: var(--blue-0, #5e90ff);
    color: #fff;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 160ms ease;
  }

  .import-submit:hover {
    background: var(--blue-1, #345fcd);
  }
</style>
