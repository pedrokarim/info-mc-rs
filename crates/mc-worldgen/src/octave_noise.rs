use crate::java_random::JavaRandom;
use crate::perlin::ImprovedNoise;

/// Octaved (fractal) noise — sums multiple Perlin noise layers at increasing frequencies.
/// This is how Minecraft generates its climate parameters (temperature, humidity, etc.).
#[derive(Clone)]
pub struct OctaveNoise {
    octaves: Vec<Option<ImprovedNoise>>,
    first_octave: i32,
    amplitudes: Vec<f64>,
}

impl OctaveNoise {
    /// Create octaved noise from a JavaRandom.
    ///
    /// - `first_octave`: the exponent for the first octave's frequency (e.g., -7 means freq = 2^7 = 128 block period)
    /// - `amplitudes`: weight for each octave (0 means skip that octave)
    pub fn new(random: &mut JavaRandom, first_octave: i32, amplitudes: &[f64]) -> Self {
        let n = amplitudes.len() as i32;
        let mut octaves = Vec::with_capacity(amplitudes.len());

        for i in 0..n {
            if amplitudes[i as usize] != 0.0 {
                octaves.push(Some(ImprovedNoise::new(random)));
            } else {
                // Still advance RNG state to stay in sync with Minecraft
                // Minecraft creates the noise object but doesn't use it — we skip
                // by advancing the RNG the same number of times
                Self::skip_noise_init(random);
                octaves.push(None);
            }
        }

        Self {
            octaves,
            first_octave,
            amplitudes: amplitudes.to_vec(),
        }
    }

    /// Advance random state by the same amount as creating an ImprovedNoise would.
    fn skip_noise_init(random: &mut JavaRandom) {
        // ImprovedNoise::new does: 3 next_double() + 256 next_int() for shuffle
        random.next_double();
        random.next_double();
        random.next_double();
        for i in 0..256 {
            random.next_int((256 - i) as i32);
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = self.frequency_at(0);
        let mut amplitude_scale = self.amplitude_scale();

        for (i, octave) in self.octaves.iter().enumerate() {
            if let Some(noise) = octave {
                let amp = self.amplitudes[i];
                if amp != 0.0 {
                    total += amp * noise.sample(x * frequency, y * frequency, z * frequency) * amplitude_scale;
                }
            }
            frequency *= 2.0;
            amplitude_scale /= 2.0;
        }

        total
    }

    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        self.sample(x, 0.0, z)
    }

    fn frequency_at(&self, _index: usize) -> f64 {
        2.0_f64.powi(-self.first_octave)
    }

    fn amplitude_scale(&self) -> f64 {
        2.0_f64.powi(self.first_octave)
    }

    /// Get the total amplitude (sum of absolute amplitudes) for normalization.
    pub fn max_value(&self) -> f64 {
        let mut total = 0.0;
        let mut amp = self.amplitude_scale();
        for a in &self.amplitudes {
            total += a.abs() * amp;
            amp /= 2.0;
        }
        total
    }
}

/// Pair of octave noises used in Minecraft's DoublePerlinNoiseSampler.
/// This is what Minecraft actually uses for climate parameters — it XORs two octave noises.
#[derive(Clone)]
pub struct DoublePerlinNoise {
    first: OctaveNoise,
    second: OctaveNoise,
    amplitude: f64,
}

impl DoublePerlinNoise {
    pub fn new(random: &mut JavaRandom, first_octave: i32, amplitudes: &[f64]) -> Self {
        let first = OctaveNoise::new(random, first_octave, amplitudes);
        let second = OctaveNoise::new(random, first_octave, amplitudes);

        // Amplitude normalization factor
        let max_val = first.max_value();
        let amplitude = if max_val != 0.0 {
            // Minecraft uses a specific normalization:
            // value / (max * (5.0/3.0))
            1.0 / (max_val * (5.0 / 3.0))
        } else {
            0.0
        };

        Self {
            first,
            second,
            amplitude,
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let x2 = x + self.second.octaves.first()
            .and_then(|o| o.as_ref())
            .map_or(0.0, |n| n.x_offset);
        let y2 = y + self.second.octaves.first()
            .and_then(|o| o.as_ref())
            .map_or(0.0, |n| n.y_offset);
        let z2 = z + self.second.octaves.first()
            .and_then(|o| o.as_ref())
            .map_or(0.0, |n| n.z_offset);

        (self.first.sample(x, y, z) + self.second.sample(x2, y2, z2)) * self.amplitude
    }

    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        self.sample(x, 0.0, z)
    }
}
