import { hexToRgb, rgbToHex } from './colors';

/** Interpolate between multiple color stops, returning `count` hex colors */
export function interpolateColors(stops: string[], count: number): string[] {
  if (count <= 0) return [];
  if (count === 1) return [stops[0]];
  if (stops.length === 1) return Array(count).fill(stops[0]);

  const rgbStops = stops.map(hexToRgb);
  const result: string[] = [];
  const segments = stops.length - 1;

  for (let i = 0; i < count; i++) {
    const t = i / (count - 1);
    const segPos = t * segments;
    const segIdx = Math.min(Math.floor(segPos), segments - 1);
    const segT = segPos - segIdx;

    const [r1, g1, b1] = rgbStops[segIdx];
    const [r2, g2, b2] = rgbStops[segIdx + 1];

    result.push(
      rgbToHex(
        r1 + (r2 - r1) * segT,
        g1 + (g2 - g1) * segT,
        b1 + (b2 - b1) * segT,
      ),
    );
  }

  return result;
}

/** Generate rainbow colors (HSL hue sweep) */
export function rainbowColors(count: number, phase: number = 0): string[] {
  if (count <= 0) return [];
  const result: string[] = [];

  for (let i = 0; i < count; i++) {
    const hue = ((i / Math.max(count - 1, 1)) + phase) % 1;
    const [r, g, b] = hslToRgb(hue * 360, 100, 50);
    result.push(rgbToHex(r, g, b));
  }

  return result;
}

function hslToRgb(h: number, s: number, l: number): [number, number, number] {
  s /= 100;
  l /= 100;
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  let r = 0, g = 0, b = 0;
  if (h < 60) { r = c; g = x; }
  else if (h < 120) { r = x; g = c; }
  else if (h < 180) { g = c; b = x; }
  else if (h < 240) { g = x; b = c; }
  else if (h < 300) { r = x; b = c; }
  else { r = c; b = x; }
  return [
    Math.round((r + m) * 255),
    Math.round((g + m) * 255),
    Math.round((b + m) * 255),
  ];
}
