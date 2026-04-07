pub mod java_random;
pub mod xoroshiro;
pub mod perlin;
pub mod octave_noise;
pub mod slime;
pub mod biomes;
pub mod biome_tree;
pub mod multinoise;
pub mod legacy;

use wasm_bindgen::prelude::*;
use biomes::Biome;
use multinoise::MultiNoiseBiomeSource;
use legacy::LegacyBiomeSource;

/// Version threshold: versions >= 1.18 use multi-noise, older use legacy layers.
fn is_modern(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 2 {
        if let Ok(minor) = parts[1].parse::<u32>() {
            return minor >= 18;
        }
    }
    true
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
    #[wasm_bindgen(constructor)]
    pub fn new(seed_hi: i32, seed_lo: i32, version: &str) -> WorldGen {
        let seed = ((seed_hi as i64) << 32) | (seed_lo as u32 as i64);

        let inner = if is_modern(version) {
            BiomeSourceInner::Modern(MultiNoiseBiomeSource::new(seed))
        } else {
            BiomeSourceInner::Legacy(LegacyBiomeSource::new(seed))
        };

        WorldGen { inner, seed }
    }

    /// Compute an area of biomes and return RGBA pixels directly.
    /// This is the main entry point — like chunkbase's get_noise_biome_area.
    ///
    /// Parameters:
    /// - `x`, `z`: top-left corner in **block coordinates**
    /// - `width`, `height`: tile size in samples (e.g. 64)
    /// - `step`: block distance between samples (e.g. 4 = biome resolution, 16 = one per chunk)
    ///
    /// Returns a flat Uint8Array of width*height*4 bytes (RGBA).
    pub fn get_biome_area_rgba(
        &self, x: i32, z: i32, width: u32, height: u32, step: u32,
    ) -> Vec<u8> {
        let step = step.max(1) as i32;
        let total = (width * height) as usize;
        let mut rgba = Vec::with_capacity(total * 4);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let bx = x + dx * step;
                let bz = z + dz * step;
                let biome = match &self.inner {
                    BiomeSourceInner::Modern(src) => src.get_biome(bx, bz),
                    BiomeSourceInner::Legacy(src) => src.get_biome(bx, bz),
                };
                let color = biome.color();
                rgba.push(((color >> 16) & 0xFF) as u8); // R
                rgba.push(((color >> 8) & 0xFF) as u8);  // G
                rgba.push((color & 0xFF) as u8);          // B
                rgba.push(255);                           // A
            }
        }

        rgba
    }

    /// Compute biome IDs for an area (without RGBA conversion).
    /// Returns width*height biome IDs.
    pub fn get_biome_area(
        &self, x: i32, z: i32, width: u32, height: u32, step: u32,
    ) -> Vec<u8> {
        let step = step.max(1) as i32;
        let total = (width * height) as usize;
        let mut ids = Vec::with_capacity(total);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let bx = x + dx * step;
                let bz = z + dz * step;
                let biome = match &self.inner {
                    BiomeSourceInner::Modern(src) => src.get_biome(bx, bz),
                    BiomeSourceInner::Legacy(src) => src.get_biome(bx, bz),
                };
                ids.push(biome.id());
            }
        }

        ids
    }

    /// Compute slime chunks for an area. Returns 1 byte per chunk (0 or 1).
    pub fn get_slime_area(
        &self, chunk_x: i32, chunk_z: i32, width: u32, height: u32,
    ) -> Vec<u8> {
        let total = (width * height) as usize;
        let mut result = Vec::with_capacity(total);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let cx = chunk_x + dx;
                let cz = chunk_z + dz;
                result.push(if slime::is_slime_chunk(self.seed, cx, cz) { 1 } else { 0 });
            }
        }

        result
    }

    pub fn is_slime_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        slime::is_slime_chunk(self.seed, chunk_x, chunk_z)
    }

    /// Legacy per-chunk API (still used by worker).
    pub fn compute_chunks(&self, chunk_coords: &[i32], resolution: u32) -> Vec<u8> {
        let num_chunks = chunk_coords.len() / 2;
        let step = resolution.max(1);
        let grid_size = (16 / step) as usize;
        let biomes_per_chunk = grid_size * grid_size;
        let mut result = Vec::with_capacity(num_chunks * (biomes_per_chunk + 1));

        for i in 0..num_chunks {
            let cx = chunk_coords[i * 2];
            let cz = chunk_coords[i * 2 + 1];
            let biomes = match &self.inner {
                BiomeSourceInner::Modern(src) => src.get_chunk_biomes(cx, cz, step),
                BiomeSourceInner::Legacy(src) => src.get_chunk_biomes(cx, cz, step),
            };
            result.extend_from_slice(&biomes);
            result.push(if slime::is_slime_chunk(self.seed, cx, cz) { 1 } else { 0 });
        }

        result
    }

    pub fn get_biome_at(&self, block_x: i32, block_z: i32) -> u8 {
        match &self.inner {
            BiomeSourceInner::Modern(src) => src.get_biome(block_x, block_z).id(),
            BiomeSourceInner::Legacy(src) => src.get_biome(block_x, block_z).id(),
        }
    }

    pub fn biome_name(id: u8) -> String {
        Biome::from_id(id).name().to_string()
    }

    pub fn biome_color(id: u8) -> u32 {
        Biome::from_id(id).color()
    }

    pub fn parse_seed(input: &str) -> Vec<i32> {
        let seed: i64 = if let Ok(n) = input.parse::<i64>() {
            n
        } else {
            java_random::JavaRandom::string_hash_code(input) as i64
        };
        vec![(seed >> 32) as i32, seed as i32]
    }
}
