<script lang="ts">
  import type { StyledChar, McStyle, EditorLine } from '$lib/utils/motd';
  import { cloneStyle, toPreviewHtml } from '$lib/utils/motd';

  let {
    line = $bindable({ chars: [] }),
    brush,
    label = 'Ligne 1',
    onselectionchange,
  }: {
    line: EditorLine;
    brush: McStyle;
    label?: string;
    onselectionchange?: (start: number, end: number) => void;
  } = $props();

  let textarea: HTMLTextAreaElement;
  let selStart = $state(0);
  let selEnd = $state(0);
  let focused = $state(false);

  // Derived: plain text from chars
  const plainText = $derived(line.chars.map((c) => c.char).join(''));

  // Derived: preview HTML
  const previewHtml = $derived(toPreviewHtml(line.chars));

  // Derived: char count
  const charCount = $derived(line.chars.length);

  // Sync textarea selection → state
  function syncSelection() {
    if (!textarea) return;
    selStart = textarea.selectionStart;
    selEnd = textarea.selectionEnd;
    onselectionchange?.(selStart, selEnd);
  }

  // Handle text input: sync textarea text → styled chars
  function handleInput() {
    const newText = textarea.value;
    const oldChars = line.chars;
    const newChars: StyledChar[] = [];

    // Diff-based approach: figure out what changed
    const oldText = oldChars.map((c) => c.char).join('');

    if (newText === oldText) return;

    // Find common prefix length
    let prefixLen = 0;
    while (prefixLen < oldText.length && prefixLen < newText.length && oldText[prefixLen] === newText[prefixLen]) {
      prefixLen++;
    }

    // Find common suffix length
    let suffixLen = 0;
    while (
      suffixLen < (oldText.length - prefixLen) &&
      suffixLen < (newText.length - prefixLen) &&
      oldText[oldText.length - 1 - suffixLen] === newText[newText.length - 1 - suffixLen]
    ) {
      suffixLen++;
    }

    // Keep prefix chars as-is
    for (let i = 0; i < prefixLen; i++) {
      newChars.push(oldChars[i]);
    }

    // Inserted chars get current brush style
    const insertedCount = newText.length - prefixLen - suffixLen;
    for (let i = 0; i < insertedCount; i++) {
      newChars.push({
        char: newText[prefixLen + i],
        style: cloneStyle(brush),
      });
    }

    // Keep suffix chars as-is
    for (let i = oldText.length - suffixLen; i < oldText.length; i++) {
      newChars.push(oldChars[i]);
    }

    line.chars = newChars;
    line = line;

    // Restore selection after state update
    requestAnimationFrame(() => syncSelection());
  }

  // Build overlay HTML with selection highlight
  function buildOverlayHtml(chars: StyledChar[], start: number, end: number, hasFocus: boolean): string {
    if (chars.length === 0) return '';

    const parts: string[] = [];

    for (let i = 0; i < chars.length; i++) {
      const { char, style } = chars[i];
      const inSelection = hasFocus && start !== end && i >= start && i < end;
      const escaped = char.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

      const styles: string[] = [];
      if (style.color) styles.push(`color:${style.color}`);
      if (style.bold) styles.push('font-weight:bold');
      if (style.italic) styles.push('font-style:italic');
      const deco: string[] = [];
      if (style.underlined) deco.push('underline');
      if (style.strikethrough) deco.push('line-through');
      if (deco.length) styles.push(`text-decoration:${deco.join(' ')}`);
      if (inSelection) styles.push('background:rgba(94,144,255,0.35);border-radius:2px');

      const cls = style.obfuscated ? ' class="mc-obfuscated"' : '';

      if (styles.length > 0 || cls) {
        parts.push(`<span${cls} style="${styles.join(';')}">${escaped}</span>`);
      } else {
        parts.push(escaped);
      }
    }

    return parts.join('');
  }

  const overlayHtml = $derived(buildOverlayHtml(line.chars, selStart, selEnd, focused));
</script>

<div class="editor-line">
  <div class="line-header">
    <span class="line-label">{label}</span>
    <span class="char-count" class:warn={charCount > 40}>{charCount}/45</span>
  </div>

  <div class="editor-surface" class:focused>
    <!-- Styled overlay -->
    <div class="overlay" aria-hidden="true">
      {#if line.chars.length === 0}
        <span class="placeholder">Tapez votre MOTD ici...</span>
      {:else}
        {@html overlayHtml}
      {/if}
    </div>

    <!-- Hidden textarea for text input -->
    <textarea
      bind:this={textarea}
      class="hidden-input"
      value={plainText}
      oninput={handleInput}
      onselect={syncSelection}
      onclick={syncSelection}
      onkeyup={syncSelection}
      onfocus={() => { focused = true; syncSelection(); }}
      onblur={() => { focused = false; }}
      rows={1}
      spellcheck={false}
      autocomplete="off"
      maxlength={45}
    ></textarea>
  </div>
</div>

<style>
  .editor-line {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .line-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .line-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .char-count {
    font-family: 'Minecraft', 'JetBrains Mono', monospace;
    font-size: 0.65rem;
    font-weight: 600;
    color: var(--ink-2, #5a7894);
  }

  .char-count.warn {
    color: var(--danger, #b83b3b);
  }

  .editor-surface {
    position: relative;
    background:
      linear-gradient(rgba(0, 0, 0, 0.55), rgba(0, 0, 0, 0.55)),
      url('/images/ui/mc_dirt.jpg');
    background-size: auto, 128px 128px;
    background-repeat: repeat;
    image-rendering: pixelated;
    border: 2px solid #1a1a1a;
    border-radius: 6px;
    min-height: 36px;
    cursor: text;
    transition: border-color 160ms ease;
  }

  .editor-surface.focused {
    border-color: var(--blue-0, #5e90ff);
  }

  .overlay {
    padding: 8px 12px;
    font-family: 'Minecraft', 'JetBrains Mono', 'Courier New', monospace;
    font-size: 0.82rem;
    line-height: 1.4;
    color: #aaaaaa;
    white-space: pre;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, 0.4);
    pointer-events: none;
    min-height: 1.4em;
  }

  .overlay :global(.mc-obfuscated) {
    animation: mc-scramble 100ms infinite;
    display: inline-block;
  }

  @keyframes mc-scramble {
    0% { opacity: 1; }
    50% { opacity: 0.7; }
    100% { opacity: 1; }
  }

  .placeholder {
    color: #999999;
    font-style: italic;
  }

  .hidden-input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    font-family: 'Minecraft', 'JetBrains Mono', monospace;
    font-size: 0.82rem;
    padding: 8px 12px;
    border: none;
    background: transparent;
    color: transparent;
    caret-color: transparent;
    resize: none;
    outline: none;
    overflow: hidden;
  }
</style>
