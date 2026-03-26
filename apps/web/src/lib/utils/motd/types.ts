export type Edition = 'java' | 'bedrock';

export interface McStyle {
  color: string | null;
  bold: boolean;
  italic: boolean;
  underlined: boolean;
  strikethrough: boolean;
  obfuscated: boolean;
}

export interface StyledChar {
  char: string;
  style: McStyle;
}

export interface EditorLine {
  chars: StyledChar[];
}

export type EditorState = [EditorLine, EditorLine];

export const DEFAULT_STYLE: McStyle = {
  color: null,
  bold: false,
  italic: false,
  underlined: false,
  strikethrough: false,
  obfuscated: false,
};

export function cloneStyle(s: McStyle): McStyle {
  return { ...s };
}

export function stylesEqual(a: McStyle, b: McStyle): boolean {
  return (
    a.color === b.color &&
    a.bold === b.bold &&
    a.italic === b.italic &&
    a.underlined === b.underlined &&
    a.strikethrough === b.strikethrough &&
    a.obfuscated === b.obfuscated
  );
}

export function isDefaultStyle(s: McStyle): boolean {
  return stylesEqual(s, DEFAULT_STYLE);
}
