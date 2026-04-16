/* @ts-self-types="./mc_worldgen.d.ts" */

/**
 * Main WASM-exported world generator.
 */
export class WorldGen {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WorldGenFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_worldgen_free(ptr, 0);
    }
    /**
     * @param {number} id
     * @returns {number}
     */
    static biome_color(id) {
        const ret = wasm.worldgen_biome_color(id);
        return ret >>> 0;
    }
    /**
     * @param {number} id
     * @returns {string}
     */
    static biome_name(id) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.worldgen_biome_name(id);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Legacy per-chunk API (still used by worker).
     * @param {Int32Array} chunk_coords
     * @param {number} resolution
     * @returns {Uint8Array}
     */
    compute_chunks(chunk_coords, resolution) {
        const ptr0 = passArray32ToWasm0(chunk_coords, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.worldgen_compute_chunks(this.__wbg_ptr, ptr0, len0, resolution);
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * Find structures in a block-coordinate area.
     * Returns a flat array: [type_id, block_x_hi, block_x_lo, block_z_hi, block_z_lo, ...]
     * (Using hi/lo i16 pairs because wasm-bindgen doesn't support tuples easily)
     * @param {number} block_x
     * @param {number} block_z
     * @param {number} block_w
     * @param {number} block_h
     * @returns {Int32Array}
     */
    find_structures(block_x, block_z, block_w, block_h) {
        const ret = wasm.worldgen_find_structures(this.__wbg_ptr, block_x, block_z, block_w, block_h);
        var v1 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Compute biome IDs for an area (without RGBA conversion).
     * Returns width*height biome IDs.
     * @param {number} x
     * @param {number} z
     * @param {number} width
     * @param {number} height
     * @param {number} step
     * @returns {Uint8Array}
     */
    get_biome_area(x, z, width, height, step) {
        const ret = wasm.worldgen_get_biome_area(this.__wbg_ptr, x, z, width, height, step);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
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
     * @param {number} x
     * @param {number} z
     * @param {number} width
     * @param {number} height
     * @param {number} step
     * @returns {Uint8Array}
     */
    get_biome_area_rgba(x, z, width, height, step) {
        const ret = wasm.worldgen_get_biome_area_rgba(this.__wbg_ptr, x, z, width, height, step);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {number} block_x
     * @param {number} block_z
     * @returns {number}
     */
    get_biome_at(block_x, block_z) {
        const ret = wasm.worldgen_get_biome_at(this.__wbg_ptr, block_x, block_z);
        return ret;
    }
    /**
     * Compute slime chunks for an area. Returns 1 byte per chunk (0 or 1).
     * @param {number} chunk_x
     * @param {number} chunk_z
     * @param {number} width
     * @param {number} height
     * @returns {Uint8Array}
     */
    get_slime_area(chunk_x, chunk_z, width, height) {
        const ret = wasm.worldgen_get_slime_area(this.__wbg_ptr, chunk_x, chunk_z, width, height);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {number} chunk_x
     * @param {number} chunk_z
     * @returns {boolean}
     */
    is_slime_chunk(chunk_x, chunk_z) {
        const ret = wasm.worldgen_is_slime_chunk(this.__wbg_ptr, chunk_x, chunk_z);
        return ret !== 0;
    }
    /**
     * Create a new world generator.
     * - `version`: "1.21", "1.20", etc.
     * - `dimension`: "overworld", "nether", or "end"
     * - `edition`: "java" or "bedrock"
     * @param {number} seed_hi
     * @param {number} seed_lo
     * @param {string} version
     * @param {string} dimension
     * @param {string} edition
     */
    constructor(seed_hi, seed_lo, version, dimension, edition) {
        const ptr0 = passStringToWasm0(version, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(dimension, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(edition, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.worldgen_new(seed_hi, seed_lo, ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        WorldGenFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} input
     * @returns {Int32Array}
     */
    static parse_seed(input) {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.worldgen_parse_seed(ptr0, len0);
        var v2 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v2;
    }
    /**
     * Get structure type name by ID.
     * @param {number} type_id
     * @returns {string}
     */
    static structure_name(type_id) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.worldgen_structure_name(type_id);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WorldGen.prototype[Symbol.dispose] = WorldGen.prototype.free;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./mc_worldgen_bg.js": import0,
    };
}

const WorldGenFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_worldgen_free(ptr >>> 0, 1));

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedInt32ArrayMemory0 = null;
function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedInt32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('mc_worldgen_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
