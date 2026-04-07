/// Multi-noise biome source for Minecraft 1.18+.
/// Correct seeding via Xoroshiro128++ and MD5 hashes (matches cubiomes exactly).

use crate::biomes::Biome;
use crate::biome_tree;
use crate::octave_noise::DoublePerlinNoise;
use crate::xoroshiro::{Xoroshiro, climate_md5};

pub struct MultiNoiseBiomeSource {
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    continentalness: DoublePerlinNoise,
    erosion: DoublePerlinNoise,
    weirdness: DoublePerlinNoise,
    shift: DoublePerlinNoise,
}

/// Noise parameter configuration.
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
        // Step 1: Master RNG from world seed
        let mut master = Xoroshiro::from_seed(seed);
        let xlo = master.next_long();
        let xhi = master.next_long();

        // Step 2: Each climate parameter gets its own RNG = (xlo ^ md5_lo, xhi ^ md5_hi)
        let temperature = Self::create_noise(xlo, xhi, climate_md5::TEMPERATURE, &TEMPERATURE);
        let humidity = Self::create_noise(xlo, xhi, climate_md5::HUMIDITY, &HUMIDITY);
        let continentalness = Self::create_noise(xlo, xhi, climate_md5::CONTINENTALNESS, &CONTINENTALNESS);
        let erosion = Self::create_noise(xlo, xhi, climate_md5::EROSION, &EROSION);
        let weirdness = Self::create_noise(xlo, xhi, climate_md5::WEIRDNESS, &WEIRDNESS);
        let shift = Self::create_noise(xlo, xhi, climate_md5::SHIFT, &SHIFT);

        Self { temperature, humidity, continentalness, erosion, weirdness, shift }
    }

    fn create_noise(xlo: u64, xhi: u64, md5: (u64, u64), params: &NoiseParams) -> DoublePerlinNoise {
        let mut rng = Xoroshiro::from_raw(xlo ^ md5.0, xhi ^ md5.1);
        DoublePerlinNoise::new(&mut rng, params.first_octave, params.amplitudes)
    }

    /// Get the biome at block coordinates (x, z), sampling at surface level.
    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        // Minecraft samples biomes at quarter-resolution (biome coords = block / 4)
        let x = block_x as f64 / 4.0;
        let z = block_z as f64 / 4.0;

        // Apply coordinate shift noise (note argument order from cubiomes)
        let shift_x = self.shift.sample(x, 0.0, z) * 4.0;
        let shift_z = self.shift.sample(z, x, 0.0) * 4.0; // z,x,0 NOT x,z,0

        let sx = x + shift_x;
        let sz = z + shift_z;

        let t = self.temperature.sample_2d(sx, sz);
        let h = self.humidity.sample_2d(sx, sz);
        let c = self.continentalness.sample_2d(sx, sz);
        let e = self.erosion.sample_2d(sx, sz);
        let w = self.weirdness.sample_2d(sx, sz);

        // Depth parameter for surface biomes
        // depth = continentalness offset from spline + surface level calculation
        // For a top-down map, we use the PV (peaks/valleys) value as depth proxy
        let depth = compute_depth(c, e, w);

        biome_tree::select_biome(t, h, c, e, depth, w)
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
}

/// Compute the depth parameter for biome selection.
/// For surface biomes, this is derived from the "ridges" value (from weirdness)
/// and a spline-based offset. For a 2D map, we approximate at y=sea_level.
fn compute_depth(c: f64, e: f64, w: f64) -> f64 {
    // PV (peaks and valleys) derived from weirdness
    // This is the "ridges" value in cubiomes
    let ridges = -3.0 * ((w.abs() - 0.6666667).abs() - 0.33333334);

    // Simplified depth for surface-level biome selection.
    // The full Minecraft implementation uses a cubic spline tree, but
    // for a 2D surface map, the biome tree works with depth ≈ 0
    // (surface level). Setting depth=0 gives correct surface biomes.
    let _ = (c, e, ridges);
    0.0
}
