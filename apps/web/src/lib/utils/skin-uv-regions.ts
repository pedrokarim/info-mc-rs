/**
 * UV region definitions for Minecraft skin textures (64×64).
 * Extracted from the SkinViewer3D component's UV mapping table.
 */

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface BodyPartRegion {
  id: string;
  label: string;
  base: Rect;
  overlay: Rect;
  color: string;
}

/**
 * Body part regions on the 64×64 skin texture.
 * Each region is the bounding box encompassing all 6 UV faces for that part.
 * Classic (4px arms) layout — slim adjusts arm widths from 4 to 3.
 */
export const BODY_PARTS: BodyPartRegion[] = [
  {
    id: 'head',
    label: 'Tête',
    base: { x: 0, y: 0, w: 32, h: 16 },
    overlay: { x: 32, y: 0, w: 32, h: 16 },
    color: '#FF6B6B',
  },
  {
    id: 'body',
    label: 'Torse',
    base: { x: 16, y: 16, w: 24, h: 16 },
    overlay: { x: 16, y: 32, w: 24, h: 16 },
    color: '#4ECDC4',
  },
  {
    id: 'rarm',
    label: 'Bras droit',
    base: { x: 40, y: 16, w: 16, h: 16 },
    overlay: { x: 40, y: 32, w: 16, h: 16 },
    color: '#45B7D1',
  },
  {
    id: 'larm',
    label: 'Bras gauche',
    base: { x: 32, y: 48, w: 16, h: 16 },
    overlay: { x: 48, y: 48, w: 16, h: 16 },
    color: '#96CEB4',
  },
  {
    id: 'rleg',
    label: 'Jambe droite',
    base: { x: 0, y: 16, w: 16, h: 16 },
    overlay: { x: 0, y: 32, w: 16, h: 16 },
    color: '#FFEAA7',
  },
  {
    id: 'lleg',
    label: 'Jambe gauche',
    base: { x: 16, y: 48, w: 16, h: 16 },
    overlay: { x: 0, y: 48, w: 16, h: 16 },
    color: '#DDA0DD',
  },
];

/**
 * UV face table for Three.js BoxGeometry.
 * Face order: +x(right), -x(left), +y(top), -y(bottom), +z(front), -z(back)
 * [x, y, w, h] in texture pixels.
 */
export function getUVTable(slim: boolean): Record<string, [number, number, number, number][]> {
  const AW = slim ? 3 : 4;
  return {
    head:    [[16,8,8,8],[0,8,8,8],[8,0,8,8],[16,0,8,8],[8,8,8,8],[24,8,8,8]],
    hat:     [[48,8,8,8],[32,8,8,8],[40,0,8,8],[48,0,8,8],[40,8,8,8],[56,8,8,8]],
    body:    [[28,20,4,12],[16,20,4,12],[20,16,8,4],[28,16,8,4],[20,20,8,12],[32,20,8,12]],
    jacket:  [[28,36,4,12],[16,36,4,12],[20,32,8,4],[28,32,8,4],[20,36,8,12],[32,36,8,12]],
    rarm:    [[44+AW,20,4,12],[40,20,4,12],[44,16,AW,4],[44+AW,16,AW,4],[44,20,AW,12],[44+AW+4,20,AW,12]],
    rsleeve: [[44+AW,36,4,12],[40,36,4,12],[44,32,AW,4],[44+AW,32,AW,4],[44,36,AW,12],[44+AW+4,36,AW,12]],
    larm:    [[36+AW,52,4,12],[32,52,4,12],[36,48,AW,4],[36+AW,48,AW,4],[36,52,AW,12],[36+AW+4,52,AW,12]],
    lsleeve: [[52+AW,52,4,12],[48,52,4,12],[52,48,AW,4],[52+AW,48,AW,4],[52,52,AW,12],[52+AW+4,52,AW,12]],
    rleg:    [[8,20,4,12],[0,20,4,12],[4,16,4,4],[8,16,4,4],[4,20,4,12],[12,20,4,12]],
    rpant:   [[8,36,4,12],[0,36,4,12],[4,32,4,4],[8,32,4,4],[4,36,4,12],[12,36,4,12]],
    lleg:    [[24,52,4,12],[16,52,4,12],[20,48,4,4],[24,48,4,4],[20,52,4,12],[28,52,4,12]],
    lpant:   [[8,52,4,12],[0,52,4,12],[4,48,4,4],[8,48,4,4],[4,52,4,12],[12,52,4,12]],
  };
}

/**
 * Get all body part regions matching a layer type.
 */
export function getRegionForPart(partId: string): BodyPartRegion | undefined {
  return BODY_PARTS.find((p) => p.id === partId);
}

/**
 * Check if a pixel coordinate (x, y) falls within a given rect.
 */
export function isInRect(x: number, y: number, rect: Rect): boolean {
  return x >= rect.x && x < rect.x + rect.w && y >= rect.y && y < rect.y + rect.h;
}

/**
 * Find which body part a pixel belongs to, for a given layer.
 */
export function findBodyPartAt(
  x: number,
  y: number,
  layer: 'base' | 'overlay'
): BodyPartRegion | undefined {
  return BODY_PARTS.find((p) => isInRect(x, y, layer === 'base' ? p.base : p.overlay));
}

/**
 * Check if a pixel falls within ANY body part region for the given layer.
 */
export function isInAnyRegion(x: number, y: number, layer: 'base' | 'overlay'): boolean {
  return BODY_PARTS.some((p) => isInRect(x, y, layer === 'base' ? p.base : p.overlay));
}
