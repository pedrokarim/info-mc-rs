/* tslint:disable */
/* eslint-disable */

/**
 * Main WASM-exported world generator.
 */
export class WorldGen {
    free(): void;
    [Symbol.dispose](): void;
    static biome_color(id: number): number;
    static biome_name(id: number): string;
    /**
     * Legacy per-chunk API (still used by worker).
     */
    compute_chunks(chunk_coords: Int32Array, resolution: number): Uint8Array;
    /**
     * Find structures in a block-coordinate area.
     * Returns a flat array: [type_id, block_x_hi, block_x_lo, block_z_hi, block_z_lo, ...]
     * (Using hi/lo i16 pairs because wasm-bindgen doesn't support tuples easily)
     */
    find_structures(block_x: number, block_z: number, block_w: number, block_h: number): Int32Array;
    /**
     * Compute biome IDs for an area (without RGBA conversion).
     * Returns width*height biome IDs.
     */
    get_biome_area(x: number, z: number, width: number, height: number, step: number): Uint8Array;
    /**
     * Compute an area of biomes and return RGBA pixels directly.
     * This is the main entry point — like chunkbase's get_noise_biome_area.
     *
     * Parameters:
     * - `x`, `z`: top-left corner in **block coordinates**
     * - `width`, `height`: tile size in samples (e.g. 64)
     * - `step`: block distance between samples (e.g. 4 = biome resolution, 16 = one per chunk)
     *
     * Returns a flat Uint8Array of width*height*4 bytes (RGBA).
     */
    get_biome_area_rgba(x: number, z: number, width: number, height: number, step: number): Uint8Array;
    get_biome_at(block_x: number, block_z: number): number;
    /**
     * Compute slime chunks for an area. Returns 1 byte per chunk (0 or 1).
     */
    get_slime_area(chunk_x: number, chunk_z: number, width: number, height: number): Uint8Array;
    is_slime_chunk(chunk_x: number, chunk_z: number): boolean;
    /**
     * Create a new world generator.
     * - `version`: "1.21", "1.20", etc.
     * - `dimension`: "overworld", "nether", or "end"
     * - `edition`: "java" or "bedrock"
     */
    constructor(seed_hi: number, seed_lo: number, version: string, dimension: string, edition: string);
    static parse_seed(input: string): Int32Array;
    /**
     * Get structure type name by ID.
     */
    static structure_name(type_id: number): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_worldgen_free: (a: number, b: number) => void;
    readonly worldgen_biome_color: (a: number) => number;
    readonly worldgen_biome_name: (a: number) => [number, number];
    readonly worldgen_compute_chunks: (a: number, b: number, c: number, d: number) => [number, number];
    readonly worldgen_find_structures: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly worldgen_get_biome_area: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly worldgen_get_biome_area_rgba: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly worldgen_get_biome_at: (a: number, b: number, c: number) => number;
    readonly worldgen_get_slime_area: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly worldgen_is_slime_chunk: (a: number, b: number, c: number) => number;
    readonly worldgen_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
    readonly worldgen_parse_seed: (a: number, b: number) => [number, number];
    readonly worldgen_structure_name: (a: number) => [number, number];
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
