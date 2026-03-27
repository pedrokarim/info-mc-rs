/**
 * Lightweight NBT parser — pure browser JS, no Node.js dependencies.
 * Uses pako for gzip decompression + DataView for binary parsing.
 */
import pako from 'pako';

// ── Types ──────────────────────────────────────────────────────────
export interface NbtTag {
  type: string;
  name: string;
  value: unknown;
  children?: NbtTag[];
}

export interface StructureBlock {
  pos: [number, number, number];
  state: number;
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
}

// ── NBT Tag IDs ────────────────────────────────────────────────────
const TAG_END = 0;
const TAG_BYTE = 1;
const TAG_SHORT = 2;
const TAG_INT = 3;
const TAG_LONG = 4;
const TAG_FLOAT = 5;
const TAG_DOUBLE = 6;
const TAG_BYTE_ARRAY = 7;
const TAG_STRING = 8;
const TAG_LIST = 9;
const TAG_COMPOUND = 10;
const TAG_INT_ARRAY = 11;
const TAG_LONG_ARRAY = 12;

const TAG_NAMES: Record<number, string> = {
  [TAG_BYTE]: 'byte', [TAG_SHORT]: 'short', [TAG_INT]: 'int', [TAG_LONG]: 'long',
  [TAG_FLOAT]: 'float', [TAG_DOUBLE]: 'double', [TAG_BYTE_ARRAY]: 'byteArray',
  [TAG_STRING]: 'string', [TAG_LIST]: 'list', [TAG_COMPOUND]: 'compound',
  [TAG_INT_ARRAY]: 'intArray', [TAG_LONG_ARRAY]: 'longArray',
};

// ── Reader class ───────────────────────────────────────────────────
class NbtReader {
  private view: DataView;
  private offset = 0;
  private textDecoder = new TextDecoder('utf-8');
  private bytes: Uint8Array;

  constructor(data: Uint8Array) {
    this.bytes = data;
    this.view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  }

  private readByte(): number { return this.view.getInt8(this.offset++); }
  private readUByte(): number { return this.view.getUint8(this.offset++); }
  private readShort(): number { const v = this.view.getInt16(this.offset); this.offset += 2; return v; }
  private readUShort(): number { const v = this.view.getUint16(this.offset); this.offset += 2; return v; }
  private readInt(): number { const v = this.view.getInt32(this.offset); this.offset += 4; return v; }
  private readFloat(): number { const v = this.view.getFloat32(this.offset); this.offset += 4; return v; }
  private readDouble(): number { const v = this.view.getFloat64(this.offset); this.offset += 8; return v; }

  private readLong(): [number, number] {
    const hi = this.readInt();
    const lo = this.readInt();
    return [hi, lo];
  }

  private readString(): string {
    const len = this.readUShort();
    const str = this.textDecoder.decode(this.bytes.subarray(this.offset, this.offset + len));
    this.offset += len;
    return str;
  }

  private readPayload(tagType: number): any {
    switch (tagType) {
      case TAG_BYTE: return this.readByte();
      case TAG_SHORT: return this.readShort();
      case TAG_INT: return this.readInt();
      case TAG_LONG: return this.readLong();
      case TAG_FLOAT: return parseFloat(this.readFloat().toFixed(6));
      case TAG_DOUBLE: return parseFloat(this.readDouble().toFixed(10));
      case TAG_BYTE_ARRAY: {
        const len = this.readInt();
        const arr: number[] = [];
        for (let i = 0; i < len; i++) arr.push(this.readByte());
        return arr;
      }
      case TAG_STRING: return this.readString();
      case TAG_LIST: {
        const itemType = this.readUByte();
        const len = this.readInt();
        const items: any[] = [];
        for (let i = 0; i < len; i++) items.push(this.readPayload(itemType));
        return { itemType, items };
      }
      case TAG_COMPOUND: {
        const entries: Record<string, { type: number; value: any }> = {};
        while (true) {
          const type = this.readUByte();
          if (type === TAG_END) break;
          const name = this.readString();
          const value = this.readPayload(type);
          entries[name] = { type, value };
        }
        return entries;
      }
      case TAG_INT_ARRAY: {
        const len = this.readInt();
        const arr: number[] = [];
        for (let i = 0; i < len; i++) arr.push(this.readInt());
        return arr;
      }
      case TAG_LONG_ARRAY: {
        const len = this.readInt();
        const arr: [number, number][] = [];
        for (let i = 0; i < len; i++) arr.push(this.readLong());
        return arr;
      }
      default:
        throw new Error(`Unknown tag type: ${tagType}`);
    }
  }

  parse(): { type: number; name: string; value: any } {
    const type = this.readUByte();
    if (type !== TAG_COMPOUND) throw new Error(`Expected root compound, got type ${type}`);
    const name = this.readString();
    const value = this.readPayload(TAG_COMPOUND);
    return { type, name, value };
  }
}

// ── Public API ─────────────────────────────────────────────────────

/**
 * Parse raw NBT bytes (auto-detects gzip).
 */
export function parseNbt(data: Uint8Array): { type: number; name: string; value: any } {
  // Check for gzip magic bytes (1f 8b)
  let bytes = data;
  if (data[0] === 0x1f && data[1] === 0x8b) {
    bytes = pako.ungzip(data);
  }
  const reader = new NbtReader(bytes);
  return reader.parse();
}

/**
 * Convert parsed NBT root into a tree for the tree viewer.
 */
export function nbtToTree(parsed: { type: number; name: string; value: any }): NbtTag {
  return compoundEntryToTag(parsed.name || 'root', TAG_COMPOUND, parsed.value);
}

function compoundEntryToTag(name: string, type: number, value: any): NbtTag {
  const typeName = TAG_NAMES[type] ?? 'unknown';

  if (type === TAG_COMPOUND) {
    const entries = value as Record<string, { type: number; value: any }>;
    const children: NbtTag[] = [];
    for (const [key, entry] of Object.entries(entries)) {
      children.push(compoundEntryToTag(key, entry.type, entry.value));
    }
    return { type: 'compound', name, value: `{${children.length}}`, children };
  }

  if (type === TAG_LIST) {
    const { itemType, items } = value as { itemType: number; items: any[] };
    const children: NbtTag[] = [];
    for (let i = 0; i < items.length; i++) {
      children.push(compoundEntryToTag(`[${i}]`, itemType, items[i]));
    }
    return { type: 'list', name, value: `[${children.length}]`, children };
  }

  if (type === TAG_BYTE_ARRAY || type === TAG_INT_ARRAY || type === TAG_LONG_ARRAY) {
    return { type: typeName, name, value: `[${(value as any[]).length} entries]` };
  }

  if (type === TAG_LONG) {
    const [hi, lo] = value as [number, number];
    return { type: 'long', name, value: `${hi < 0 ? '-' : ''}${Math.abs(hi) * 0x100000000 + (lo >>> 0)}` };
  }

  return { type: typeName, name, value };
}

/**
 * Extract Minecraft structure data from parsed NBT.
 */
export function extractStructure(parsed: { type: number; name: string; value: any }): StructureData | null {
  try {
    const root = parsed.value as Record<string, { type: number; value: any }>;

    const sizeEntry = root['size'];
    if (!sizeEntry || sizeEntry.type !== TAG_LIST) return null;
    const sizeItems = sizeEntry.value.items as number[];
    const size: [number, number, number] = [sizeItems[0], sizeItems[1], sizeItems[2]];

    const paletteEntry = root['palette'];
    const palette: StructurePaletteEntry[] = [];
    if (paletteEntry && paletteEntry.type === TAG_LIST) {
      for (const item of paletteEntry.value.items as Record<string, { type: number; value: any }>[]) {
        const name = item['Name']?.value ?? 'minecraft:air';
        const props: Record<string, string> = {};
        if (item['Properties']) {
          const propEntries = item['Properties'].value as Record<string, { type: number; value: any }>;
          for (const [k, v] of Object.entries(propEntries)) {
            props[k] = String(v.value);
          }
        }
        palette.push({ Name: name, Properties: Object.keys(props).length > 0 ? props : undefined });
      }
    }

    const blocksEntry = root['blocks'];
    const blocks: StructureBlock[] = [];
    if (blocksEntry && blocksEntry.type === TAG_LIST) {
      for (const item of blocksEntry.value.items as Record<string, { type: number; value: any }>[]) {
        const posItems = item['pos']?.value?.items as number[];
        if (!posItems) continue;
        const pos: [number, number, number] = [posItems[0], posItems[1], posItems[2]];
        const state = item['state']?.value ?? 0;
        blocks.push({ pos, state });
      }
    }

    const dataVersion = root['DataVersion']?.value ?? 0;

    return { size, palette, blocks, dataVersion };
  } catch {
    return null;
  }
}
