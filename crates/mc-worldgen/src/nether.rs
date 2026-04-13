/// Nether biome generation (1.16+) — exact cubiomes algorithm.
///
/// Uses Java Random (NOT Xoroshiro) for noise seeding.
/// Two DoublePerlin noises: temperature (seed) and humidity (seed+1).
/// 5 biome climate points with distance offsets.

use crate::biomes::Biome;
use crate::java_random::JavaRandom;
use crate::perlin::ImprovedNoise;

/// DoublePerlin using Java Random (legacy, for Nether).
/// This is the pre-1.18 DoublePerlin that uses JavaRandom seeding.
struct LegacyDoublePerlin {
    oct_a: Vec<ImprovedNoise>,
    oct_b: Vec<ImprovedNoise>,
    amplitude: f64,
}

impl LegacyDoublePerlin {
    /// Create from Java Random seed, with octave range [omin, omin+len-1].
    fn new(seed: i64, omin: i32, len: i32) -> Self {
        let mut rng = JavaRandom::new(seed);

        let mut oct_a = Vec::new();
        let mut oct_b = Vec::new();

        let end = omin + len - 1;
        let persist_start = 1.0 / ((1_i64 << len) - 1) as f64;
        let lacuna_start = 2.0_f64.powi(end);

        // If end == 0, init first octave then continue
        let start_idx;
        if end == 0 {
            let mut noise = Self::init_perlin(&mut rng);
            noise.amplitude = persist_start;
            noise.lacunarity = lacuna_start;
            oct_a.push(noise);
            start_idx = 1;
        } else {
            // Skip RNG state for negative octaves
            Self::skip_n(&mut rng, (-end * 262) as i64);
            start_idx = 0;
        }

        let mut persist = persist_start * if end == 0 { 2.0 } else { 1.0 };
        let mut lacuna = lacuna_start * if end == 0 { 0.5 } else { 1.0 };

        // Adjust if we started from idx 1
        if start_idx == 1 {
            // persist and lacuna already doubled/halved above
        }

        for i in start_idx..len {
            let mut noise = Self::init_perlin(&mut rng);
            let idx = i;
            let p = persist_start * 2.0_f64.powi(idx);
            let l = lacuna_start * 0.5_f64.powi(idx);
            noise.amplitude = p;
            noise.lacunarity = l;
            oct_a.push(noise);
        }

        // Second set of octaves
        for i in 0..len {
            let mut noise = Self::init_perlin(&mut rng);
            let p = persist_start * 2.0_f64.powi(i);
            let l = lacuna_start * 0.5_f64.powi(i);
            noise.amplitude = p;
            noise.lacunarity = l;
            oct_b.push(noise);
        }

        // Amplitude normalization: (5/3) * len / (len + 1)
        let amplitude = (5.0 / 3.0) * len as f64 / (len + 1) as f64;

        Self { oct_a, oct_b, amplitude }
    }

    /// perlinInit using JavaRandom (matches cubiomes exactly).
    fn init_perlin(rng: &mut JavaRandom) -> ImprovedNoise {
        let x_offset = rng.next_double() * 256.0;
        let y_offset = rng.next_double() * 256.0;
        let z_offset = rng.next_double() * 256.0;

        let mut perm = [0u8; 257];
        for i in 0..256u16 {
            perm[i as usize] = i as u8;
        }
        for i in 0..256usize {
            let j = rng.next_int((256 - i) as i32) as usize + i;
            perm.swap(i, j);
        }
        perm[256] = perm[0];

        ImprovedNoise::from_raw(perm, x_offset, y_offset, z_offset)
    }

    fn skip_n(rng: &mut JavaRandom, n: i64) {
        for _ in 0..n.unsigned_abs() {
            rng.next(32);
        }
    }

    fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        const F: f64 = 337.0 / 331.0;

        let mut va = 0.0;
        for oct in &self.oct_a {
            let lf = oct.lacunarity;
            va += oct.amplitude * oct.sample(x * lf, y * lf, z * lf);
        }

        let mut vb = 0.0;
        for oct in &self.oct_b {
            let lf = oct.lacunarity;
            vb += oct.amplitude * oct.sample(x * lf * F, y * lf * F, z * lf * F);
        }

        (va + vb) * self.amplitude
    }
}

pub struct NetherBiomeSource {
    temperature: LegacyDoublePerlin,
    humidity: LegacyDoublePerlin,
}

/// Nether biome climate points with distance offsets (from cubiomes).
/// Format: (temp, humidity, offset_sq, biome)
const NETHER_POINTS: [(f64, f64, f64, Biome); 5] = [
    (0.0,  0.0,  0.0,            Biome::NetherWastes),
    (0.0, -0.5,  0.0,            Biome::SoulSandValley),
    (0.4,  0.0,  0.0,            Biome::CrimsonForest),
    (0.0,  0.5,  0.375 * 0.375,  Biome::WarpedForest),   // offset!
    (-0.5, 0.0,  0.175 * 0.175,  Biome::BasaltDeltas),    // offset!
];

impl NetherBiomeSource {
    pub fn new(seed: i64) -> Self {
        // cubiomes: setSeed(&s, seed) for temp, setSeed(&s, seed+1) for humidity
        let temperature = LegacyDoublePerlin::new(seed, -7, 2);
        let humidity = LegacyDoublePerlin::new(seed.wrapping_add(1), -7, 2);

        Self { temperature, humidity }
    }

    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        // Nether samples at biome coords (block / 4), y=0
        let x = block_x as f64 / 4.0;
        let z = block_z as f64 / 4.0;

        let temp = self.temperature.sample(x, 0.0, z) as f32;
        let humid = self.humidity.sample(x, 0.0, z) as f32;

        // Find closest biome (with distance offset)
        let mut best_id = 0;
        let mut best_dist = f32::MAX;

        for (i, &(t, h, off, _)) in NETHER_POINTS.iter().enumerate() {
            let dx = t as f32 - temp;
            let dy = h as f32 - humid;
            let dsq = dx * dx + dy * dy + off as f32;
            if dsq < best_dist {
                best_dist = dsq;
                best_id = i;
            }
        }

        NETHER_POINTS[best_id].3
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
