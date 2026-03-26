/**
 * NBT parser wrapper for Minecraft structure files.
 * Uses prismarine-nbt for binary NBT parsing.
 */
import * as nbt from 'prismarine-nbt';

export interface NbtTag {
  type: string;
  name: string;
  value: unknown;
  children?: NbtTag[];
}

export interface StructureBlock {
  pos: [number, number, number];
  state: number;
  nbt?: Record<string, unknown>;
}

export interface StructurePaletteEntry {
  Name: string;
  Properties?: Record<string, string>;
}

export interface StructureData {
  size: [number, number, number];
  palette: StructurePaletteEntry[];
  blocks: StructureBlock[];
  dataVersion: number;
  entities: unknown[];
}

/**
 * Parse a raw NBT buffer (may be gzip compressed).
 */
export async function parseNbt(buffer: Uint8Array): Promise<{ parsed: nbt.NBT; type: string }> {
  const result = await nbt.parse(Buffer.from(buffer));
  return result;
}

/**
 * Convert prismarine-nbt parsed data into a flat tree for the tree viewer.
 */
export function nbtToTree(tag: any, name = 'root'): NbtTag {
  if (tag === null || tag === undefined) {
    return { type: 'null', name, value: null };
  }

  const tagType = tag.type;
  const tagValue = tag.value;

  if (tagType === 'compound') {
    const children: NbtTag[] = [];
    for (const [key, child] of Object.entries(tagValue as Record<string, any>)) {
      children.push(nbtToTree(child, key));
    }
    return { type: 'compound', name, value: `{${children.length}}`, children };
  }

  if (tagType === 'list') {
    const listValue = tagValue as { type: string; value: any[] };
    const children: NbtTag[] = [];
    for (let i = 0; i < listValue.value.length; i++) {
      children.push(nbtToTree({ type: listValue.type, value: listValue.value[i] }, `[${i}]`));
    }
    return { type: 'list', name, value: `[${children.length}]`, children };
  }

  if (tagType === 'byteArray' || tagType === 'intArray' || tagType === 'longArray') {
    const arr = tagValue as number[];
    return { type: tagType, name, value: `[${arr.length} entries]` };
  }

  // Primitive types: byte, short, int, long, float, double, string
  return { type: tagType, name, value: tagValue };
}

/**
 * Extract structure data from parsed NBT (Minecraft structure format).
 */
export function extractStructure(parsed: nbt.NBT): StructureData | null {
  try {
    const root = parsed.value as any;

    const sizeTag = root.size;
    if (!sizeTag) return null;
    const size: [number, number, number] = [
      sizeTag.value.value[0],
      sizeTag.value.value[1],
      sizeTag.value.value[2],
    ];

    const paletteTag = root.palette;
    const palette: StructurePaletteEntry[] = [];
    if (paletteTag) {
      for (const entry of paletteTag.value.value) {
        const name = entry.Name?.value ?? 'minecraft:air';
        const props: Record<string, string> = {};
        if (entry.Properties) {
          for (const [k, v] of Object.entries(entry.Properties.value as Record<string, any>)) {
            props[k] = v.value;
          }
        }
        palette.push({ Name: name, Properties: Object.keys(props).length > 0 ? props : undefined });
      }
    }

    const blocksTag = root.blocks;
    const blocks: StructureBlock[] = [];
    if (blocksTag) {
      for (const block of blocksTag.value.value) {
        const pos: [number, number, number] = [
          block.pos.value.value[0],
          block.pos.value.value[1],
          block.pos.value.value[2],
        ];
        const state = block.state?.value ?? 0;
        blocks.push({ pos, state });
      }
    }

    const dataVersion = root.DataVersion?.value ?? 0;
    const entities = root.entities?.value?.value ?? [];

    return { size, palette, blocks, dataVersion, entities };
  } catch {
    return null;
  }
}
