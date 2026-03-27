pub mod java_random;
pub mod perlin;
pub mod octave_noise;
pub mod slime;
pub mod biomes;
pub mod multinoise;
pub mod legacy;

use wasm_bindgen::prelude::*;
use biomes::Biome;
use multinoise::MultiNoiseBiomeSource;
use legacy::LegacyBiomeSource;

/// Version threshold: versions >= 1.18 use multi-noise, older use legacy layers.
fn is_modern(version: &str) -> bool {
    // Parse "1.XX" or "1.XX.Y" format
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 2 {
        if let Ok(minor) = parts[1].parse::<u32>() {
            return minor >= 18;
        }
    }
    true // default to modern
}

enum BiomeSourceInner {
    Modern(MultiNoiseBiomeSource),
    Legacy(LegacyBiomeSource),
}

/// Main WASM-exported world generator.
#[wasm_bindgen]
pub struct WorldGen {
    inner: BiomeSourceInner,
    seed: i64,
}

#[wasm_bindgen]
impl WorldGen {
    /// Create a new world generator for the given seed and Minecraft version.
    /// Version format: "1.18", "1.20", "1.7", etc.
    #[wasm_bindgen(constructor)]
    pub fn new(seed_hi: i32, seed_lo: i32, version: &str) -> WorldGen {
        // Reconstruct i64 from two i32s (WASM doesn't support i64 directly)
        let seed = ((seed_hi as i64) << 32) | (seed_lo as u32 as i64);

        let inner = if is_modern(version) {
            BiomeSourceInner::Modern(MultiNoiseBiomeSource::new(seed))
        } else {
            BiomeSourceInner::Legacy(LegacyBiomeSource::new(seed))
        };

        WorldGen { inner, seed }
    }

    /// Check if a chunk is a slime chunk.
    pub fn is_slime_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        slime::is_slime_chunk(self.seed, chunk_x, chunk_z)
    }

    /// Compute biomes for a batch of chunks.
    ///
    /// `chunk_coords`: flat array of [cx0, cz0, cx1, cz1, ...] pairs.
    /// `resolution`: block step size (1 = per-block, 4 = per-4-blocks).
    ///
    /// Returns a flat `Uint8Array` with data for each chunk sequentially:
    /// For each chunk: `[biome_id_0, biome_id_1, ..., biome_id_n, slime_flag]`
    /// where n = (16/resolution)^2 and slime_flag is 0 or 1.
    pub fn compute_chunks(&self, chunk_coords: &[i32], resolution: u32) -> Vec<u8> {
        let num_chunks = chunk_coords.len() / 2;
        let step = resolution.max(1);
        let grid_size = (16 / step) as usize;
        let biomes_per_chunk = grid_size * grid_size;
        // Each chunk: biomes + 1 byte for slime flag
        let mut result = Vec::with_capacity(num_chunks * (biomes_per_chunk + 1));

        for i in 0..num_chunks {
            let cx = chunk_coords[i * 2];
            let cz = chunk_coords[i * 2 + 1];

            // Compute biome grid
            let biomes = match &self.inner {
                BiomeSourceInner::Modern(src) => src.get_chunk_biomes(cx, cz, step),
                BiomeSourceInner::Legacy(src) => src.get_chunk_biomes(cx, cz, step),
            };

            result.extend_from_slice(&biomes);

            // Slime flag
            result.push(if slime::is_slime_chunk(self.seed, cx, cz) { 1 } else { 0 });
        }

        result
    }

    /// Get biome name by ID.
    pub fn biome_name(id: u8) -> String {
        Biome::from_id(id).name().to_string()
    }

    /// Get biome color by ID (0xRRGGBB).
    pub fn biome_color(id: u8) -> u32 {
        Biome::from_id(id).color()
    }

    /// Get biome at a specific block position.
    pub fn get_biome_at(&self, block_x: i32, block_z: i32) -> u8 {
        match &self.inner {
            BiomeSourceInner::Modern(src) => src.get_biome(block_x, block_z).id(),
            BiomeSourceInner::Legacy(src) => src.get_biome(block_x, block_z).id(),
        }
    }

    /// Parse a seed string (numeric or text) into high/low i32 parts.
    /// Returns [hi, lo] as a JS array.
    pub fn parse_seed(input: &str) -> Vec<i32> {
        let seed: i64 = if let Ok(n) = input.parse::<i64>() {
            n
        } else {
            java_random::JavaRandom::string_hash_code(input) as i64
        };
        vec![(seed >> 32) as i32, seed as i32]
    }
}
