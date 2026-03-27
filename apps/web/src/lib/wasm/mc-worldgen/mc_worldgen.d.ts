/* tslint:disable */
/* eslint-disable */

/**
 * Main WASM-exported world generator.
 */
export class WorldGen {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get biome color by ID (0xRRGGBB).
     */
    static biome_color(id: number): number;
    /**
     * Get biome name by ID.
     */
    static biome_name(id: number): string;
    /**
     * Compute biomes for a batch of chunks.
     *
     * `chunk_coords`: flat array of [cx0, cz0, cx1, cz1, ...] pairs.
     * `resolution`: block step size (1 = per-block, 4 = per-4-blocks).
     *
     * Returns a flat `Uint8Array` with data for each chunk sequentially:
     * For each chunk: `[biome_id_0, biome_id_1, ..., biome_id_n, slime_flag]`
     * where n = (16/resolution)^2 and slime_flag is 0 or 1.
     */
    compute_chunks(chunk_coords: Int32Array, resolution: number): Uint8Array;
    /**
     * Get biome at a specific block position.
     */
    get_biome_at(block_x: number, block_z: number): number;
    /**
     * Check if a chunk is a slime chunk.
     */
    is_slime_chunk(chunk_x: number, chunk_z: number): boolean;
    /**
     * Create a new world generator for the given seed and Minecraft version.
     * Version format: "1.18", "1.20", "1.7", etc.
     */
    constructor(seed_hi: number, seed_lo: number, version: string);
    /**
     * Parse a seed string (numeric or text) into high/low i32 parts.
     * Returns [hi, lo] as a JS array.
     */
    static parse_seed(input: string): Int32Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_worldgen_free: (a: number, b: number) => void;
    readonly worldgen_biome_color: (a: number) => number;
    readonly worldgen_biome_name: (a: number) => [number, number];
    readonly worldgen_compute_chunks: (a: number, b: number, c: number, d: number) => [number, number];
    readonly worldgen_get_biome_at: (a: number, b: number, c: number) => number;
    readonly worldgen_is_slime_chunk: (a: number, b: number, c: number) => number;
    readonly worldgen_new: (a: number, b: number, c: number, d: number) => number;
    readonly worldgen_parse_seed: (a: number, b: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
