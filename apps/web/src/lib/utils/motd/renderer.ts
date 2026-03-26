import type { StyledChar, McStyle } from './types';
import { DEFAULT_STYLE, stylesEqual, isDefaultStyle } from './types';
import { hexToCode, namedToHex } from './colors';

// ── Preview HTML (mirrors mc-motd Rust crate output) ────────────────

function escapeHtml(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

export function toPreviewHtml(chars: StyledChar[]): string {
  if (chars.length === 0) return '';

  const parts: string[] = [];
  let i = 0;

  while (i < chars.length) {
    const style = chars[i].style;
    let text = '';

    // Group consecutive chars with identical style
    while (i < chars.length && stylesEqual(chars[i].style, style)) {
      text += chars[i].char;
      i++;
    }

    const escaped = escapeHtml(text);

    if (isDefaultStyle(style)) {
      parts.push(escaped);
      continue;
    }

    if (style.obfuscated) {
      parts.push(`<span class="mc-obfuscated">${escaped}</span>`);
      continue;
    }

    const styles: string[] = [];
    if (style.color) styles.push(`color:${style.color}`);
    if (style.bold) styles.push('font-weight:bold');
    if (style.italic) styles.push('font-style:italic');
    const deco: string[] = [];
    if (style.underlined) deco.push('underline');
    if (style.strikethrough) deco.push('line-through');
    if (deco.length) styles.push(`text-decoration:${deco.join(' ')}`);

    if (styles.length === 0 && !style.obfuscated) {
      parts.push(escaped);
    } else {
      parts.push(`<span style="${styles.join(';')}">${escaped}</span>`);
    }
  }

  return parts.join('');
}

// ── Legacy format (§ or &) ──────────────────────────────────────────

export function toLegacy(chars: StyledChar[], prefix: '§' | '&' = '§'): string {
  if (chars.length === 0) return '';

  const parts: string[] = [];
  let prevStyle: McStyle = { ...DEFAULT_STYLE };

  for (const { char, style } of chars) {
    if (!stylesEqual(style, prevStyle)) {
      // Emit codes for style change
      const codes = styleToCodes(style);
      for (const code of codes) {
        parts.push(prefix + code);
      }
      prevStyle = { ...style };
    }
    parts.push(char);
  }

  return parts.join('');
}

function styleToCodes(style: McStyle): string[] {
  const codes: string[] = [];

  // Color code (resets formatting in legacy)
  if (style.color) {
    const code = hexToCode(style.color);
    if (code) {
      codes.push(code);
    }
    // For hex colors without a legacy code, we can't represent them in legacy format
    // Fall back to nearest code would lose precision, so we just use the nearest
    if (!code) {
      // Hex colors in legacy are not standard, skip color
    }
  }

  // Formatting codes (must come after color since color resets them)
  if (style.bold) codes.push('l');
  if (style.italic) codes.push('o');
  if (style.underlined) codes.push('n');
  if (style.strikethrough) codes.push('m');
  if (style.obfuscated) codes.push('k');

  return codes;
}

// ── JSON Text Component ─────────────────────────────────────────────

interface JsonComponent {
  text: string;
  color?: string;
  bold?: boolean;
  italic?: boolean;
  underlined?: boolean;
  strikethrough?: boolean;
  obfuscated?: boolean;
}

export function toJsonComponent(chars: StyledChar[]): object {
  if (chars.length === 0) {
    return { text: '' };
  }

  // Group consecutive chars by style
  const spans: { text: string; style: McStyle }[] = [];
  let i = 0;

  while (i < chars.length) {
    const style = chars[i].style;
    let text = '';
    while (i < chars.length && stylesEqual(chars[i].style, style)) {
      text += chars[i].char;
      i++;
    }
    spans.push({ text, style });
  }

  if (spans.length === 1) {
    return styleToJsonComponent(spans[0].text, spans[0].style);
  }

  const extra: JsonComponent[] = spans.map((s) => styleToJsonComponent(s.text, s.style));

  return { text: '', extra };
}

function styleToJsonComponent(text: string, style: McStyle): JsonComponent {
  const comp: JsonComponent = { text };

  if (style.color) {
    // Use named color if possible, otherwise hex
    const code = hexToCode(style.color);
    if (code) {
      const entry = findNameByHex(style.color);
      if (entry) comp.color = entry;
      else comp.color = style.color.toLowerCase();
    } else {
      comp.color = style.color.toLowerCase();
    }
  }

  if (style.bold) comp.bold = true;
  if (style.italic) comp.italic = true;
  if (style.underlined) comp.underlined = true;
  if (style.strikethrough) comp.strikethrough = true;
  if (style.obfuscated) comp.obfuscated = true;

  return comp;
}

function findNameByHex(hex: string): string | null {
  const upper = hex.toUpperCase();
  // Search through known colors
  const entries: [string, string][] = [
    ['black', '#000000'], ['dark_blue', '#0000AA'], ['dark_green', '#00AA00'],
    ['dark_aqua', '#00AAAA'], ['dark_red', '#AA0000'], ['dark_purple', '#AA00AA'],
    ['gold', '#FFAA00'], ['gray', '#AAAAAA'], ['dark_gray', '#555555'],
    ['blue', '#5555FF'], ['green', '#55FF55'], ['aqua', '#55FFFF'],
    ['red', '#FF5555'], ['light_purple', '#FF55FF'], ['yellow', '#FFFF55'],
    ['white', '#FFFFFF'],
  ];
  for (const [name, h] of entries) {
    if (h.toUpperCase() === upper) return name;
  }
  return null;
}

// ── MiniMessage (Kyori Adventure) ───────────────────────────────────

export function toMiniMessage(chars: StyledChar[]): string {
  if (chars.length === 0) return '';

  // Group consecutive chars by style
  const spans: { text: string; style: McStyle }[] = [];
  let i = 0;

  while (i < chars.length) {
    const style = chars[i].style;
    let text = '';
    while (i < chars.length && stylesEqual(chars[i].style, style)) {
      text += chars[i].char;
      i++;
    }
    spans.push({ text, style });
  }

  const parts: string[] = [];

  for (const { text, style } of spans) {
    const openTags: string[] = [];
    const closeTags: string[] = [];

    if (style.obfuscated) {
      openTags.push('<obfuscated>');
      closeTags.unshift('</obfuscated>');
    }
    if (style.bold) {
      openTags.push('<bold>');
      closeTags.unshift('</bold>');
    }
    if (style.italic) {
      openTags.push('<italic>');
      closeTags.unshift('</italic>');
    }
    if (style.underlined) {
      openTags.push('<underlined>');
      closeTags.unshift('</underlined>');
    }
    if (style.strikethrough) {
      openTags.push('<strikethrough>');
      closeTags.unshift('</strikethrough>');
    }
    if (style.color) {
      const name = findNameByHex(style.color);
      if (name) {
        openTags.push(`<${name}>`);
        closeTags.unshift(`</${name}>`);
      } else {
        openTags.push(`<color:${style.color.toLowerCase()}>`);
        closeTags.unshift('</color>');
      }
    }

    // Escape MiniMessage special chars in text
    const escaped = text.replace(/</g, '\\<').replace(/>/g, '\\>');

    parts.push(openTags.join('') + escaped + closeTags.join(''));
  }

  return parts.join('');
}
