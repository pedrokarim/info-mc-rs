/// Octave noise and DoublePerlinNoise for Minecraft 1.18+.
/// Matches cubiomes' xOctaveInit / xDoublePerlinInit / sampleDoublePerlin exactly.
use crate::perlin::ImprovedNoise;
use crate::xoroshiro::{Xoroshiro, octave_md5};

/// Octaved (fractal) noise — sums multiple Perlin noise layers at increasing frequencies.
#[derive(Clone)]
pub struct OctaveNoise {
    pub octaves: Vec<ImprovedNoise>, // only non-zero amplitude octaves
}

impl OctaveNoise {
    /// Create octaved noise from Xoroshiro RNG.
    ///
    /// - `rng`: the Xoroshiro to read xlo/xhi from (consumes 2 next_long calls)
    /// - `first_octave`: e.g. -10 for temperature
    /// - `amplitudes`: weight for each octave (0 means skip)
    ///
    /// Matches cubiomes xOctaveInit exactly:
    /// - Reads xlo, xhi once from rng
    /// - Each octave's perlin is seeded from (xlo ^ octave_md5_lo, xhi ^ octave_md5_hi)
    /// - Skipped octaves (amplitude=0) consume NO RNG
    /// - lacunarity = 2^(-first_octave) * 2^i for octave i
    /// - persistence = len / (2^len - 1) * 0.5^i for octave i
    pub fn new(rng: &mut Xoroshiro, first_octave: i32, amplitudes: &[f64]) -> Self {
        let len = amplitudes.len();

        // Read the two keys for this OctaveNoise
        let xlo = rng.next_long();
        let xhi = rng.next_long();

        // Compute initial lacunarity and persistence
        // lacunarity = 2^(first_octave), NOT 2^(-first_octave)!
        // For first_octave=-9: lacunarity = 1/512 (low frequency, large features)
        let mut lacunarity = 2.0_f64.powi(first_octave);
        // persist = 2^(len-1) / (2^len - 1) — NOT len/(2^len-1) !
        // This ensures the max possible octave sum = 1.0
        let mut persistence = if len > 0 {
            (1u64 << (len - 1)) as f64 / ((1u64 << len) - 1) as f64
        } else {
            0.0
        };

        let mut octaves = Vec::new();

        for i in 0..len {
            if amplitudes[i] != 0.0 {
                // Seed this octave's perlin from XOR with octave MD5
                let octave_num = first_octave + i as i32;
                let (md5_lo, md5_hi) = octave_md5(octave_num);
                let mut perlin_rng = Xoroshiro::from_raw(xlo ^ md5_lo, xhi ^ md5_hi);
                let mut noise = ImprovedNoise::new(&mut perlin_rng);
                noise.amplitude = amplitudes[i] * persistence;
                noise.lacunarity = lacunarity;
                octaves.push(noise);
            }
            // Whether skipped or not, advance lacunarity/persistence
            lacunarity *= 2.0;
            persistence *= 0.5;
        }

        Self { octaves }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut total = 0.0;
        for octave in &self.octaves {
            let lf = octave.lacunarity;
            total += octave.amplitude * octave.sample(x * lf, y * lf, z * lf);
        }
        total
    }

    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        let mut total = 0.0;
        for octave in &self.octaves {
            let lf = octave.lacunarity;
            total += octave.amplitude * octave.sample_2d(x * lf, z * lf);
        }
        total
    }
}

/// DoublePerlinNoise — pair of OctaveNoise used by Minecraft for climate parameters.
/// The second noise (octB) is sampled at coordinates scaled by 337/331.
#[derive(Clone)]
pub struct DoublePerlinNoise {
    oct_a: OctaveNoise,
    oct_b: OctaveNoise,
    amplitude: f64,
}

impl DoublePerlinNoise {
    /// Create from Xoroshiro RNG.
    /// rng is consumed sequentially: first for oct_a, then for oct_b.
    pub fn new(rng: &mut Xoroshiro, first_octave: i32, amplitudes: &[f64]) -> Self {
        let oct_a = OctaveNoise::new(rng, first_octave, amplitudes);
        let oct_b = OctaveNoise::new(rng, first_octave, amplitudes);

        // Compute effective_len: trim leading and trailing zero amplitudes
        let first_nonzero = amplitudes.iter().position(|&a| a != 0.0).unwrap_or(0);
        let last_nonzero = amplitudes.iter().rposition(|&a| a != 0.0).unwrap_or(0);
        let effective_len = if amplitudes.is_empty() {
            0
        } else {
            last_nonzero - first_nonzero + 1
        };

        // Amplitude = (5/3) * effective_len / (effective_len + 1)
        let amplitude = if effective_len > 0 {
            (5.0 / 3.0) * effective_len as f64 / (effective_len + 1) as f64
        } else {
            0.0
        };

        Self {
            oct_a,
            oct_b,
            amplitude,
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        const F: f64 = 337.0 / 331.0;
        let v = self.oct_a.sample(x, y, z) + self.oct_b.sample(x * F, y * F, z * F);
        v * self.amplitude
    }

    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        const F: f64 = 337.0 / 331.0;
        let v = self.oct_a.sample_2d(x, z) + self.oct_b.sample_2d(x * F, z * F);
        v * self.amplitude
    }
}
