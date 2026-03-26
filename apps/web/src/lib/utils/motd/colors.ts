import type { Edition } from './types';

export interface McColorEntry {
  code: string;
  name: string;
  hex: string;
}

/** 16 standard Minecraft colors (Java & Bedrock) */
export const JAVA_COLORS: McColorEntry[] = [
  { code: '0', name: 'black', hex: '#000000' },
  { code: '1', name: 'dark_blue', hex: '#0000AA' },
  { code: '2', name: 'dark_green', hex: '#00AA00' },
  { code: '3', name: 'dark_aqua', hex: '#00AAAA' },
  { code: '4', name: 'dark_red', hex: '#AA0000' },
  { code: '5', name: 'dark_purple', hex: '#AA00AA' },
  { code: '6', name: 'gold', hex: '#FFAA00' },
  { code: '7', name: 'gray', hex: '#AAAAAA' },
  { code: '8', name: 'dark_gray', hex: '#555555' },
  { code: '9', name: 'blue', hex: '#5555FF' },
  { code: 'a', name: 'green', hex: '#55FF55' },
  { code: 'b', name: 'aqua', hex: '#55FFFF' },
  { code: 'c', name: 'red', hex: '#FF5555' },
  { code: 'd', name: 'light_purple', hex: '#FF55FF' },
  { code: 'e', name: 'yellow', hex: '#FFFF55' },
  { code: 'f', name: 'white', hex: '#FFFFFF' },
];

/** Bedrock-exclusive material colors */
export const BEDROCK_MATERIAL_COLORS: McColorEntry[] = [
  { code: 'g', name: 'minecoin_gold', hex: '#DDD605' },
  { code: 'h', name: 'material_quartz', hex: '#E3D4D1' },
  { code: 'i', name: 'material_iron', hex: '#CECACA' },
  { code: 'j', name: 'material_netherite', hex: '#443A3B' },
  { code: 'p', name: 'material_gold', hex: '#DEB12D' },
  { code: 'q', name: 'material_emerald', hex: '#119F36' },
  { code: 's', name: 'material_diamond', hex: '#2CBAA8' },
  { code: 't', name: 'material_lapis', hex: '#21497B' },
  { code: 'u', name: 'material_amethyst', hex: '#9A5CC6' },
  { code: 'v', name: 'material_resin', hex: '#EB7114' },
];

/** All color entries for a given edition */
export function getColors(edition: Edition): McColorEntry[] {
  return edition === 'bedrock'
    ? [...JAVA_COLORS, ...BEDROCK_MATERIAL_COLORS]
    : JAVA_COLORS;
}

const codeToHexMap = new Map<string, string>();
const hexToCodeMap = new Map<string, string>();
const nameToHexMap = new Map<string, string>();

for (const c of [...JAVA_COLORS, ...BEDROCK_MATERIAL_COLORS]) {
  codeToHexMap.set(c.code, c.hex);
  hexToCodeMap.set(c.hex.toUpperCase(), c.code);
  nameToHexMap.set(c.name, c.hex);
}

export function codeToHex(code: string): string | null {
  return codeToHexMap.get(code.toLowerCase()) ?? null;
}

export function hexToCode(hex: string): string | null {
  return hexToCodeMap.get(hex.toUpperCase()) ?? null;
}

export function namedToHex(name: string): string | null {
  return nameToHexMap.get(name) ?? null;
}

/** Find the nearest legacy code for a hex color (Euclidean distance in RGB) */
export function hexToNearestCode(hex: string, edition: Edition): string {
  const target = hexToRgb(hex);
  const palette = getColors(edition);
  let best = palette[0];
  let bestDist = Infinity;

  for (const c of palette) {
    const rgb = hexToRgb(c.hex);
    const dist =
      (target[0] - rgb[0]) ** 2 +
      (target[1] - rgb[1]) ** 2 +
      (target[2] - rgb[2]) ** 2;
    if (dist < bestDist) {
      bestDist = dist;
      best = c;
    }
  }

  return best.code;
}

export function hexToRgb(hex: string): [number, number, number] {
  const h = hex.replace('#', '');
  return [
    parseInt(h.slice(0, 2), 16),
    parseInt(h.slice(2, 4), 16),
    parseInt(h.slice(4, 6), 16),
  ];
}

export function rgbToHex(r: number, g: number, b: number): string {
  return (
    '#' +
    [r, g, b]
      .map((v) => Math.max(0, Math.min(255, Math.round(v))).toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()
  );
}
