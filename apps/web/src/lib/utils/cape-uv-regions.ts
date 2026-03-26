/**
 * UV region definitions for Minecraft cape textures.
 * Standard OptiFine format: 22×17 pixels (scale 1).
 */

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface CapeFace {
  id: string;
  label: string;
  rect: Rect;
  color: string;
}

/** Default dimensions for standard Mojang/OptiFine cape textures (64×32). */
export const CAPE_DIMS = { w: 64, h: 32 };

/**
 * The 6 faces of the cape texture at scale=1 (22×17).
 */
export const CAPE_FACES: CapeFace[] = [
  { id: 'front',  label: 'Face avant',   rect: { x: 1,  y: 1, w: 10, h: 16 }, color: '#4ECDC4' },
  { id: 'back',   label: 'Face arrière', rect: { x: 12, y: 1, w: 10, h: 16 }, color: '#FF6B6B' },
  { id: 'top',    label: 'Haut',         rect: { x: 1,  y: 0, w: 10, h: 1 },  color: '#45B7D1' },
  { id: 'bottom', label: 'Bas',          rect: { x: 11, y: 0, w: 10, h: 1 },  color: '#96CEB4' },
  { id: 'left',   label: 'Gauche',       rect: { x: 0,  y: 1, w: 1,  h: 16 }, color: '#FFEAA7' },
  { id: 'right',  label: 'Droite',       rect: { x: 11, y: 1, w: 1,  h: 16 }, color: '#DDA0DD' },
];

/**
 * Get UV face array for Three.js BoxGeometry (cape).
 * Face order: +x(right), -x(left), +y(top), -y(bottom), +z(front/outer), -z(back/inner).
 */
export function getCapeUVs(cs: number, tw: number, th: number): [number, number, number, number][] {
  return [
    [11 * cs, cs,     cs,      16 * cs],  // +x right
    [0,       cs,     cs,      16 * cs],  // -x left
    [cs,      0,      10 * cs, cs],        // +y top
    [11 * cs, cs,     10 * cs, -cs],       // -y bottom (flip)
    [cs,      cs,     10 * cs, 16 * cs],   // +z front (outer)
    [12 * cs, cs,     10 * cs, 16 * cs],   // -z back (inner)
  ];
}

/**
 * Get UV face array for elytra wings.
 */
export function getElytraUVs(cs: number, tw: number, th: number): [number, number, number, number][] {
  return [
    [(22 + 10) * cs, 2 * cs,          2 * cs,  20 * cs],  // +x right side
    [22 * cs,        2 * cs,          2 * cs,  20 * cs],  // -x left side
    [(22 + 2) * cs,  0,               10 * cs, 2 * cs],   // +y top
    [(22 + 2 + 10) * cs, 0,           10 * cs, 2 * cs],   // -y bottom
    [(22 + 2) * cs,  2 * cs,          10 * cs, 20 * cs],  // +z front (outer)
    [(22 + 2 + 10 + 2) * cs, 2 * cs,  10 * cs, 20 * cs],  // -z back (inner)
  ];
}

function isInRect(x: number, y: number, rect: Rect): boolean {
  return x >= rect.x && x < rect.x + rect.w && y >= rect.y && y < rect.y + rect.h;
}

/**
 * Find which cape face a pixel belongs to.
 */
export function findCapeFaceAt(x: number, y: number): CapeFace | undefined {
  return CAPE_FACES.find((f) => isInRect(x, y, f.rect));
}

/**
 * Check if a pixel is within any valid cape face region.
 */
export function isInCapeFace(x: number, y: number): boolean {
  return CAPE_FACES.some((f) => isInRect(x, y, f.rect));
}

/**
 * Mirror X within a cape face for symmetry painting.
 */
export function capeMirrorX(x: number, face: CapeFace): number {
  const local = x - face.rect.x;
  const mirrored = face.rect.w - 1 - local;
  return face.rect.x + mirrored;
}
