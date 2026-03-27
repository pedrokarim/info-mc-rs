use crate::biomes::{Biome, BiomeEntry, ClimatePoint, find_closest_biome, overworld_biome_entries_118};
use crate::java_random::JavaRandom;
use crate::octave_noise::DoublePerlinNoise;

/// Multi-noise biome source for Minecraft 1.18+.
/// Uses 6 climate noise parameters to select biomes from a 6D parameter space.
pub struct MultiNoiseBiomeSource {
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    continentalness: DoublePerlinNoise,
    erosion: DoublePerlinNoise,
    weirdness: DoublePerlinNoise,
    shift: DoublePerlinNoise,
    biome_entries: Vec<BiomeEntry>,
}

/// Noise parameter configuration (first octave exponent + amplitude per octave).
struct NoiseParams {
    first_octave: i32,
    amplitudes: &'static [f64],
}

// ===== Noise configs from vanilla Minecraft 1.18+ datapacks =====
static TEMPERATURE: NoiseParams = NoiseParams {
    first_octave: -10,
    amplitudes: &[1.5, 0.0, 1.0, 0.0, 0.0, 0.0],
};
static HUMIDITY: NoiseParams = NoiseParams {
    first_octave: -8,
    amplitudes: &[1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
};
static CONTINENTALNESS: NoiseParams = NoiseParams {
    first_octave: -9,
    amplitudes: &[1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
};
static EROSION: NoiseParams = NoiseParams {
    first_octave: -9,
    amplitudes: &[1.0, 1.0, 0.0, 1.0, 1.0],
};
static WEIRDNESS: NoiseParams = NoiseParams {
    first_octave: -7,
    amplitudes: &[1.0, 2.0, 1.0, 0.0, 0.0, 0.0],
};
static SHIFT: NoiseParams = NoiseParams {
    first_octave: -3,
    amplitudes: &[1.0, 1.0, 1.0, 0.0],
};

impl MultiNoiseBiomeSource {
    pub fn new(seed: i64) -> Self {
        // Each noise parameter uses a different seed offset.
        let temperature = Self::create_noise(seed, 0, &TEMPERATURE);
        let humidity = Self::create_noise(seed, 1, &HUMIDITY);
        let continentalness = Self::create_noise(seed, 2, &CONTINENTALNESS);
        let erosion = Self::create_noise(seed, 3, &EROSION);
        let weirdness = Self::create_noise(seed, 4, &WEIRDNESS);
        let shift = Self::create_noise(seed, 5, &SHIFT);

        Self {
            temperature,
            humidity,
            continentalness,
            erosion,
            weirdness,
            shift,
            biome_entries: overworld_biome_entries_118(),
        }
    }

    fn create_noise(seed: i64, index: i64, params: &NoiseParams) -> DoublePerlinNoise {
        let noise_seed = seed.wrapping_add(index.wrapping_mul(10000));
        let mut rng = JavaRandom::new(noise_seed);
        DoublePerlinNoise::new(&mut rng, params.first_octave, params.amplitudes)
    }

    /// Get the biome at block coordinates (x, z), sampling at surface level.
    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        // Minecraft samples at quarter-resolution (biome coords = block / 4)
        let x = block_x as f64 / 4.0;
        let z = block_z as f64 / 4.0;

        // Apply coordinate shift noise
        let shift_x = self.shift.sample_2d(x, z) * 4.0;
        let shift_z = self.shift.sample_2d(z, x) * 4.0;

        let sx = x + shift_x;
        let sz = z + shift_z;

        let point = ClimatePoint {
            temperature: self.temperature.sample_2d(sx, sz),
            humidity: self.humidity.sample_2d(sx, sz),
            continentalness: self.continentalness.sample_2d(sx, sz),
            erosion: self.erosion.sample_2d(sx, sz),
            weirdness: self.weirdness.sample_2d(sx, sz),
            depth: 0.0,
        };

        find_closest_biome(&point, &self.biome_entries)
    }

    /// Batch compute biomes for a chunk. Returns biome IDs in row-major order.
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
}
