#![allow(warnings, clippy::all)]

pub mod bedrock_random;
pub mod biome_tree;
pub mod biomes;
pub mod end;
pub mod end_city;
pub mod java_random;
pub mod legacy;
pub mod multinoise;
pub mod nether;
pub mod octave_noise;
pub mod perlin;
pub mod slime;
pub mod structures;
pub mod xoroshiro;

use biomes::Biome;
use end::EndBiomeSource;
use legacy::LegacyBiomeSource;
use multinoise::MultiNoiseBiomeSource;
use nether::NetherBiomeSource;
use wasm_bindgen::prelude::*;

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
    Nether(NetherBiomeSource),
    End(EndBiomeSource),
    Legacy(LegacyBiomeSource),
}

/// Main WASM-exported world generator.
#[wasm_bindgen]
pub struct WorldGen {
    inner: BiomeSourceInner,
    seed: i64,
    dimension: String,
    edition: structures::Edition,
}

#[wasm_bindgen]
impl WorldGen {
    /// Create a new world generator.
    /// - `version`: "1.21", "1.20", etc.
    /// - `dimension`: "overworld", "nether", or "end"
    /// - `edition`: "java" or "bedrock"
    #[wasm_bindgen(constructor)]
    pub fn new(
        seed_hi: i32,
        seed_lo: i32,
        version: &str,
        dimension: &str,
        edition: &str,
    ) -> WorldGen {
        let seed = ((seed_hi as i64) << 32) | (seed_lo as u32 as i64);

        let inner = match dimension {
            "nether" => BiomeSourceInner::Nether(NetherBiomeSource::new(seed)),
            "end" => BiomeSourceInner::End(EndBiomeSource::new(seed)),
            _ => {
                // Overworld
                if is_modern(version) {
                    BiomeSourceInner::Modern(MultiNoiseBiomeSource::new(seed))
                } else {
                    BiomeSourceInner::Legacy(LegacyBiomeSource::new(seed))
                }
            }
        };

        let ed = match edition {
            "bedrock" => structures::Edition::Bedrock,
            _ => structures::Edition::Java,
        };

        WorldGen {
            inner,
            seed,
            dimension: dimension.to_string(),
            edition: ed,
        }
    }

    /// Helper: get biome at block coords for any dimension.
    fn biome_at(&self, bx: i32, bz: i32) -> Biome {
        match &self.inner {
            BiomeSourceInner::Modern(src) => src.get_biome(bx, bz),
            BiomeSourceInner::Nether(src) => src.get_biome(bx, bz),
            BiomeSourceInner::End(src) => src.get_biome(bx, bz),
            BiomeSourceInner::Legacy(src) => src.get_biome(bx, bz),
        }
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
        &self,
        x: i32,
        z: i32,
        width: u32,
        height: u32,
        step: u32,
    ) -> Vec<u8> {
        let step = step.max(1) as i32;
        let total = (width * height) as usize;
        let mut rgba = Vec::with_capacity(total * 4);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let bx = x + dx * step;
                let bz = z + dz * step;
                let biome = self.biome_at(bx, bz);
                let color = biome.color();
                rgba.push(((color >> 16) & 0xFF) as u8); // R
                rgba.push(((color >> 8) & 0xFF) as u8); // G
                rgba.push((color & 0xFF) as u8); // B
                rgba.push(255); // A
            }
        }

        rgba
    }

    /// Compute biome IDs for an area (without RGBA conversion).
    /// Returns width*height biome IDs.
    pub fn get_biome_area(&self, x: i32, z: i32, width: u32, height: u32, step: u32) -> Vec<u8> {
        let step = step.max(1) as i32;
        let total = (width * height) as usize;
        let mut ids = Vec::with_capacity(total);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let bx = x + dx * step;
                let bz = z + dz * step;
                let biome = self.biome_at(bx, bz);
                ids.push(biome.id());
            }
        }

        ids
    }

    /// Compute slime chunks for an area. Returns 1 byte per chunk (0 or 1).
    pub fn get_slime_area(&self, chunk_x: i32, chunk_z: i32, width: u32, height: u32) -> Vec<u8> {
        let total = (width * height) as usize;
        let mut result = Vec::with_capacity(total);

        for dz in 0..height as i32 {
            for dx in 0..width as i32 {
                let cx = chunk_x + dx;
                let cz = chunk_z + dz;
                result.push(if slime::is_slime_chunk(self.seed, cx, cz) {
                    1
                } else {
                    0
                });
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
            let biomes: Vec<u8> = match &self.inner {
                BiomeSourceInner::Modern(src) => src.get_chunk_biomes(cx, cz, step),
                BiomeSourceInner::Nether(src) => src.get_chunk_biomes(cx, cz, step),
                BiomeSourceInner::End(src) => src.get_chunk_biomes(cx, cz, step),
                BiomeSourceInner::Legacy(src) => src.get_chunk_biomes(cx, cz, step),
            };
            result.extend_from_slice(&biomes);
            result.push(if slime::is_slime_chunk(self.seed, cx, cz) {
                1
            } else {
                0
            });
        }

        result
    }

    pub fn get_biome_at(&self, block_x: i32, block_z: i32) -> u8 {
        self.biome_at(block_x, block_z).id()
    }

    /// Find structures in a block-coordinate area.
    /// Returns a flat array: [type_id, block_x_hi, block_x_lo, block_z_hi, block_z_lo, ...]
    /// (Using hi/lo i16 pairs because wasm-bindgen doesn't support tuples easily)
    pub fn find_structures(
        &self,
        block_x: i32,
        block_z: i32,
        block_w: i32,
        block_h: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();

        let types = match self.dimension.as_str() {
            "nether" => structures::StructureType::nether_types(),
            "end" => structures::StructureType::end_types(),
            _ => structures::StructureType::overworld_types(),
        };
        for &st in types {
            let positions = structures::find_structures_in_area(
                self.seed,
                st,
                block_x,
                block_z,
                block_w,
                block_h,
                self.edition,
            );
            for (bx, bz) in positions {
                // End Cities spawn on End Midlands or End Highlands.
                // An End City is EITHER with ship OR without — never both.
                if st == structures::StructureType::EndCity {
                    let biome = self.biome_at(bx, bz);
                    if biome != Biome::EndHighlands && biome != Biome::EndMidlands {
                        continue;
                    }
                    // Determine if this End City has a ship
                    let cx = bx >> 4;
                    let cz = bz >> 4;
                    let type_id = if end_city::has_end_ship(self.seed, cx, cz) {
                        structures::StructureType::EndCityShip as u8 as i32
                    } else {
                        structures::StructureType::EndCity as u8 as i32
                    };
                    result.push(type_id);
                    result.push(bx);
                    result.push(bz);
                    continue;
                }
                // End Gateway only spawns on End Highlands
                if st == structures::StructureType::EndGateway {
                    let biome = self.biome_at(bx, bz);
                    if biome != Biome::EndHighlands {
                        continue;
                    }
                }
                result.push(st as u8 as i32);
                result.push(bx);
                result.push(bz);
            }
        }

        result
    }

    /// Get structure type name by ID.
    pub fn structure_name(type_id: u8) -> String {
        // Map back from u8 to StructureType
        let names = [
            "village",
            "desert-temple",
            "jungle-temple",
            "witch-hut",
            "igloo",
            "ocean-monument",
            "mansion",
            "pillager-outpost",
            "stronghold",
            "ocean-ruin",
            "shipwreck",
            "buried-treasure",
            "ruined-portal",
            "ancient-city",
            "trail-ruin",
            "trial-chamber",
            "nether-fortress",
            "bastion-treasure",
            "mineshaft",
            "dungeon",
            "desert-well",
            "fossil",
            "spawn",
            "end-city",
            "end-gateway",
            "end-city-ship",
        ];
        names
            .get(type_id as usize)
            .unwrap_or(&"unknown")
            .to_string()
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
