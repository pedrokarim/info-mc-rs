import type { StyledChar, McStyle } from './types';
import { DEFAULT_STYLE, cloneStyle } from './types';
import { codeToHex, namedToHex } from './colors';

// ── Parse legacy string (§ or & codes) ──────────────────────────────

export function parseLegacyString(input: string): StyledChar[] {
  const chars: StyledChar[] = [];
  let style: McStyle = cloneStyle(DEFAULT_STYLE);
  let i = 0;

  while (i < input.length) {
    const ch = input[i];

    if ((ch === '§' || ch === '&') && i + 1 < input.length) {
      const code = input[i + 1].toLowerCase();

      if (isColorCode(code)) {
        // Color code resets all formatting
        style = {
          ...DEFAULT_STYLE,
          color: codeToHex(code),
        };
        i += 2;
        continue;
      }

      if (isFormatCode(code)) {
        style = cloneStyle(style);
        applyFormatCode(style, code);
        i += 2;
        continue;
      }

      if (code === 'r') {
        style = cloneStyle(DEFAULT_STYLE);
        i += 2;
        continue;
      }

      // Unknown code, treat as literal
    }

    chars.push({ char: ch, style: cloneStyle(style) });
    i++;
  }

  return chars;
}

function isColorCode(code: string): boolean {
  return /^[0-9a-f]$/.test(code) || /^[g-jps-v]$/.test(code);
}

function isFormatCode(code: string): boolean {
  return /^[klmno]$/.test(code);
}

function applyFormatCode(style: McStyle, code: string): void {
  switch (code) {
    case 'k': style.obfuscated = true; break;
    case 'l': style.bold = true; break;
    case 'm': style.strikethrough = true; break;
    case 'n': style.underlined = true; break;
    case 'o': style.italic = true; break;
  }
}

// ── Parse JSON Text Component ───────────────────────────────────────

export function parseJsonComponent(input: string): StyledChar[] {
  try {
    const json = JSON.parse(input);
    return parseComponent(json, cloneStyle(DEFAULT_STYLE));
  } catch {
    // If not valid JSON, try as legacy string
    return parseLegacyString(input);
  }
}

function parseComponent(value: unknown, parentStyle: McStyle): StyledChar[] {
  if (typeof value === 'string') {
    return parseLegacyString(value);
  }

  if (Array.isArray(value)) {
    const chars: StyledChar[] = [];
    for (const item of value) {
      chars.push(...parseComponent(item, parentStyle));
    }
    return chars;
  }

  if (typeof value === 'object' && value !== null) {
    const obj = value as Record<string, unknown>;
    const style = cloneStyle(parentStyle);

    // Apply color
    if (typeof obj.color === 'string') {
      const color = obj.color as string;
      if (color.startsWith('#')) {
        style.color = color.toUpperCase();
      } else {
        const hex = namedToHex(color);
        if (hex) style.color = hex;
      }
    }

    // Apply formatting
    if (typeof obj.bold === 'boolean') style.bold = obj.bold;
    if (typeof obj.italic === 'boolean') style.italic = obj.italic;
    if (typeof obj.underlined === 'boolean') style.underlined = obj.underlined;
    if (typeof obj.strikethrough === 'boolean') style.strikethrough = obj.strikethrough;
    if (typeof obj.obfuscated === 'boolean') style.obfuscated = obj.obfuscated;

    const chars: StyledChar[] = [];

    // Parse text field
    if (typeof obj.text === 'string' && obj.text.length > 0) {
      const text = obj.text as string;
      // Text itself may contain legacy codes
      const parsed = parseLegacyString(text);
      // Apply component style to parsed chars that have default style
      for (const ch of parsed) {
        if (ch.style.color === null && style.color !== null) {
          ch.style.color = style.color;
        }
        if (style.bold && !ch.style.bold) ch.style.bold = true;
        if (style.italic && !ch.style.italic) ch.style.italic = true;
        if (style.underlined && !ch.style.underlined) ch.style.underlined = true;
        if (style.strikethrough && !ch.style.strikethrough) ch.style.strikethrough = true;
        if (style.obfuscated && !ch.style.obfuscated) ch.style.obfuscated = true;
      }
      chars.push(...parsed);
    }

    // Parse extra children
    if (Array.isArray(obj.extra)) {
      for (const child of obj.extra) {
        chars.push(...parseComponent(child, style));
      }
    }

    return chars;
  }

  return [];
}
