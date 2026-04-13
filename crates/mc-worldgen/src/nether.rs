/// Nether biome generation (1.16+).
/// Uses multi-noise with 2 parameters: temperature and humidity.
/// Much simpler than overworld — only 5 biomes.

use crate::biomes::Biome;
use crate::octave_noise::DoublePerlinNoise;
use crate::xoroshiro::{Xoroshiro, climate_md5};

pub struct NetherBiomeSource {
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
}

/// Nether noise params — simpler than overworld.
struct NoiseParams {
    first_octave: i32,
    amplitudes: &'static [f64],
}

static NETHER_TEMP: NoiseParams = NoiseParams {
    first_octave: -7,
    amplitudes: &[1.0, 1.0],
};

static NETHER_HUMID: NoiseParams = NoiseParams {
    first_octave: -7,
    amplitudes: &[1.0, 1.0],
};

/// Nether biome climate points (temperature, humidity).
/// Each biome occupies a region in 2D noise space.
struct NetherBiomePoint {
    biome: Biome,
    temp: f64,
    humid: f64,
}

const NETHER_BIOMES: [NetherBiomePoint; 5] = [
    NetherBiomePoint { biome: Biome::NetherWastes,    temp: 0.0,  humid: 0.0 },
    NetherBiomePoint { biome: Biome::SoulSandValley,  temp: 0.0,  humid: -0.5 },
    NetherBiomePoint { biome: Biome::CrimsonForest,   temp: 0.4,  humid: 0.0 },
    NetherBiomePoint { biome: Biome::WarpedForest,    temp: 0.0,  humid: 0.5 },
    NetherBiomePoint { biome: Biome::BasaltDeltas,    temp: -0.5, humid: 0.0 },
];

impl NetherBiomeSource {
    pub fn new(seed: i64) -> Self {
        let mut master = Xoroshiro::from_seed(seed);
        let xlo = master.next_long();
        let xhi = master.next_long();

        let temperature = Self::create_noise(xlo, xhi, climate_md5::TEMPERATURE, &NETHER_TEMP);
        let humidity = Self::create_noise(xlo, xhi, climate_md5::HUMIDITY, &NETHER_HUMID);

        Self { temperature, humidity }
    }

    fn create_noise(xlo: u64, xhi: u64, md5: (u64, u64), params: &NoiseParams) -> DoublePerlinNoise {
        let mut rng = Xoroshiro::from_raw(xlo ^ md5.0, xhi ^ md5.1);
        DoublePerlinNoise::new(&mut rng, params.first_octave, params.amplitudes)
    }

    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        let x = block_x as f64 / 4.0;
        let z = block_z as f64 / 4.0;

        let t = self.temperature.sample_2d(x, z);
        let h = self.humidity.sample_2d(x, z);

        // Find closest biome in 2D space
        let mut best = Biome::NetherWastes;
        let mut best_dist = f64::MAX;
        for bp in &NETHER_BIOMES {
            let dt = t - bp.temp;
            let dh = h - bp.humid;
            let dist = dt * dt + dh * dh;
            if dist < best_dist {
                best_dist = dist;
                best = bp.biome;
            }
        }
        best
    }

    pub fn get_chunk_biomes(&self, chunk_x: i32, chunk_z: i32, step: u32) -> Vec<u8> {
        let step = step.max(1);
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
