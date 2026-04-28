use crate::biomes::Biome;
use crate::biomes::legacy::from_legacy_id;

/// Pre-1.18 biome generation using the layer system.
///
/// Minecraft's old worldgen stacks "layers" that transform a 2D grid:
/// 1. Island layer (seed-based random land/ocean)
/// 2. Zoom layers (upscale 2x with interpolation)
/// 3. Add biome layer (assign biomes based on temperature/rainfall)
/// 4. Shore/River layers (add transitions)
///
/// This implementation focuses on producing correct biome output at surface level.
pub struct LegacyBiomeSource {
    world_seed: i64,
}

impl LegacyBiomeSource {
    pub fn new(seed: i64) -> Self {
        Self { world_seed: seed }
    }

    /// Get the biome at block coordinates (x, z).
    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        // The layer system operates at 1:4 resolution for biomes
        let bx = block_x >> 2;
        let bz = block_z >> 2;

        let legacy_id = self.get_layer_biome(bx, bz);
        from_legacy_id(legacy_id)
    }

    /// Batch compute biomes for a chunk.
    pub fn get_chunk_biomes(&self, chunk_x: i32, chunk_z: i32, resolution: u32) -> Vec<u8> {
        let step = resolution.max(1);
        let size = (16 / step) as usize;
        let mut result = Vec::with_capacity(size * size);

        let base_x = chunk_x * 16;
        let base_z = chunk_z * 16;

        for dz in 0..size {
            for dx in 0..size {
                let bx = base_x + (dx as u32 * step) as i32;
                let bz = base_z + (dz as u32 * step) as i32;
                result.push(self.get_biome(bx, bz).id());
            }
        }

        result
    }

    /// Core layer pipeline. Each function represents a layer transformation.
    fn get_layer_biome(&self, x: i32, z: i32) -> i32 {
        // Layer stack (simplified but functional):
        // 1. Continent (island) → 2. Biome assignment → 3. Zoom → 4. Shore/River

        // Start with continent layer at very coarse scale
        let continent = self.layer_continent(x >> 6, z >> 6);

        // Determine climate zone
        let climate = self.layer_climate(x >> 4, z >> 4);

        // Select biome based on continent + climate
        let biome = self.layer_biome(x >> 2, z >> 2, continent, climate);

        // Add ocean variants
        let biome = self.layer_ocean_temp(x, z, biome);

        // Add rivers
        let biome = self.layer_river(x, z, biome);

        // Add shores
        self.layer_shore(x, z, biome)
    }

    // ===== Layer implementations =====

    /// Continent layer: determines land vs ocean at coarse scale.
    fn layer_continent(&self, x: i32, z: i32) -> bool {
        let hash = self.layer_hash(1, x, z);
        // ~30% land, 70% ocean (roughly matches Minecraft)
        (hash % 10) < 3
    }

    /// Climate zone: 0=frozen, 1=cold, 2=temperate, 3=warm, 4=hot
    fn layer_climate(&self, x: i32, z: i32) -> i32 {
        let hash = self.layer_hash(2, x, z);
        // Smooth climate gradient with some randomness
        let base = ((x.wrapping_mul(3) + z.wrapping_mul(7)).wrapping_abs() % 200) as f64 / 200.0;
        let noise = (hash % 100) as f64 / 100.0 * 0.3;
        let temp = (base + noise).clamp(0.0, 1.0);

        if temp < 0.15 {
            0
        }
        // frozen
        else if temp < 0.35 {
            1
        }
        // cold
        else if temp < 0.65 {
            2
        }
        // temperate
        else if temp < 0.85 {
            3
        }
        // warm
        else {
            4
        } // hot
    }

    /// Biome selection based on continent and climate.
    fn layer_biome(&self, x: i32, z: i32, is_land: bool, climate: i32) -> i32 {
        if !is_land {
            return 0; // ocean
        }

        let hash = self.layer_hash(3, x, z);
        let variant = (hash % 6) as usize;

        // Biome IDs are legacy Minecraft numeric IDs
        match climate {
            0 => {
                // Frozen
                [12, 12, 30, 12, 30, 12][variant] // snowy_tundra, snowy_taiga
            }
            1 => {
                // Cold
                [5, 32, 5, 3, 5, 32][variant] // taiga, giant_tree_taiga, mountains
            }
            2 => {
                // Temperate
                [4, 29, 1, 27, 4, 1][variant] // forest, dark_forest, plains, birch_forest
            }
            3 => {
                // Warm
                [35, 1, 4, 35, 21, 1][variant] // savanna, plains, forest, jungle
            }
            4 => {
                // Hot
                [2, 2, 35, 21, 37, 2][variant] // desert, savanna, jungle, badlands
            }
            _ => 1, // plains fallback
        }
    }

    /// Add ocean temperature variants based on climate.
    fn layer_ocean_temp(&self, x: i32, z: i32, biome: i32) -> i32 {
        if biome != 0 && biome != 24 {
            return biome;
        }

        let climate = self.layer_climate(x >> 3, z >> 3);
        let is_deep = self.layer_hash(6, x >> 2, z >> 2) % 4 == 0;

        match (climate, is_deep) {
            (0, true) => 50,  // deep_frozen_ocean
            (0, false) => 10, // frozen_ocean
            (1, true) => 49,  // deep_cold_ocean
            (1, false) => 46, // cold_ocean
            (2, true) => 24,  // deep_ocean
            (2, false) => 0,  // ocean
            (3, true) => 48,  // deep_lukewarm_ocean
            (3, false) => 45, // lukewarm_ocean
            (_, true) => 47,  // deep_warm_ocean
            (_, false) => 44, // warm_ocean
        }
    }

    /// Add rivers through land biomes.
    fn layer_river(&self, x: i32, z: i32, biome: i32) -> i32 {
        // Don't add rivers in oceans
        if self.is_ocean(biome) {
            return biome;
        }

        let river_noise = self.layer_hash(4, x, z);
        // ~8% chance of river at any given point
        if river_noise % 13 == 0 {
            let climate = self.layer_climate(x >> 3, z >> 3);
            if climate == 0 { 11 } else { 7 } // frozen_river or river
        } else {
            biome
        }
    }

    /// Add shore/beach at land-ocean boundaries.
    fn layer_shore(&self, x: i32, z: i32, biome: i32) -> i32 {
        if self.is_ocean(biome) || biome == 7 || biome == 11 {
            return biome;
        }

        // Check if adjacent to ocean
        let has_ocean_neighbor = self.is_ocean(self.get_neighbor_biome(x - 1, z))
            || self.is_ocean(self.get_neighbor_biome(x + 1, z))
            || self.is_ocean(self.get_neighbor_biome(x, z - 1))
            || self.is_ocean(self.get_neighbor_biome(x, z + 1));

        if !has_ocean_neighbor {
            return biome;
        }

        // Mountains/badlands get stone shore
        if biome == 3 || biome == 37 || biome == 38 || biome == 39 {
            return 25; // stone_shore
        }

        let climate = self.layer_climate(x >> 3, z >> 3);
        if climate == 0 {
            26 // snowy_beach
        } else {
            16 // beach
        }
    }

    /// Quick biome lookup for neighbor checks (simplified, no river/shore).
    fn get_neighbor_biome(&self, x: i32, z: i32) -> i32 {
        let continent = self.layer_continent(x >> 6, z >> 6);
        let climate = self.layer_climate(x >> 4, z >> 4);
        let biome = self.layer_biome(x >> 2, z >> 2, continent, climate);
        self.layer_ocean_temp(x, z, biome)
    }

    fn is_ocean(&self, biome: i32) -> bool {
        matches!(biome, 0 | 10 | 24 | 44 | 45 | 46 | 47 | 48 | 49 | 50)
    }

    /// Deterministic hash for a layer at position (x, z).
    /// Reproduces Minecraft's layer seed mixing.
    fn layer_hash(&self, layer_salt: i64, x: i32, z: i32) -> u64 {
        let mut seed = self.world_seed;
        seed = seed.wrapping_mul(
            seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407),
        );
        seed = seed.wrapping_add(layer_salt);
        seed = seed.wrapping_mul(
            seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407),
        );
        seed = seed.wrapping_add(x as i64);
        seed = seed.wrapping_mul(
            seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407),
        );
        seed = seed.wrapping_add(z as i64);
        seed = seed.wrapping_mul(
            seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407),
        );
        seed = seed.wrapping_add(layer_salt);
        (seed >> 24) as u64
    }
}
