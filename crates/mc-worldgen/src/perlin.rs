use crate::java_random::JavaRandom;

/// Ken Perlin's "improved noise" (2002) — the exact algorithm Minecraft uses.
/// The permutation table is initialized from a JavaRandom instance.
#[derive(Clone)]
pub struct ImprovedNoise {
    p: [u8; 512],
    pub x_offset: f64,
    pub y_offset: f64,
    pub z_offset: f64,
}

impl ImprovedNoise {
    pub fn new(random: &mut JavaRandom) -> Self {
        self::ImprovedNoise::with_offsets(random, true)
    }

    pub fn with_offsets(random: &mut JavaRandom, use_offsets: bool) -> Self {
        let x_offset = if use_offsets {
            random.next_double() * 256.0
        } else {
            0.0
        };
        let y_offset = if use_offsets {
            random.next_double() * 256.0
        } else {
            0.0
        };
        let z_offset = if use_offsets {
            random.next_double() * 256.0
        } else {
            0.0
        };

        let mut p = [0u8; 512];

        // Initialize with identity
        for i in 0..256 {
            p[i] = i as u8;
        }

        // Fisher-Yates shuffle using JavaRandom
        for i in 0..256 {
            let j = random.next_int((256 - i) as i32) as usize + i;
            p.swap(i, j);
        }

        // Duplicate for wrapping
        for i in 0..256 {
            p[i + 256] = p[i];
        }

        Self {
            p,
            x_offset,
            y_offset,
            z_offset,
        }
    }

    /// Sample the noise at (x, y, z). Returns a value roughly in [-1, 1].
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let xx = x + self.x_offset;
        let yy = y + self.y_offset;
        let zz = z + self.z_offset;

        let xi = floor(xx) as i32;
        let yi = floor(yy) as i32;
        let zi = floor(zz) as i32;

        let xf = xx - xi as f64;
        let yf = yy - yi as f64;
        let zf = zz - zi as f64;

        let u = fade(xf);
        let v = fade(yf);
        let w = fade(zf);

        let xi = (xi & 255) as usize;
        let yi = (yi & 255) as usize;
        let zi = (zi & 255) as usize;

        let a = self.p[xi] as usize + yi;
        let aa = self.p[a] as usize + zi;
        let ab = self.p[a + 1] as usize + zi;
        let b = self.p[xi + 1] as usize + yi;
        let ba = self.p[b] as usize + zi;
        let bb = self.p[b + 1] as usize + zi;

        lerp(
            w,
            lerp(
                v,
                lerp(
                    u,
                    grad(self.p[aa] as i32, xf, yf, zf),
                    grad(self.p[ba] as i32, xf - 1.0, yf, zf),
                ),
                lerp(
                    u,
                    grad(self.p[ab] as i32, xf, yf - 1.0, zf),
                    grad(self.p[bb] as i32, xf - 1.0, yf - 1.0, zf),
                ),
            ),
            lerp(
                v,
                lerp(
                    u,
                    grad(self.p[aa + 1] as i32, xf, yf, zf - 1.0),
                    grad(self.p[ba + 1] as i32, xf - 1.0, yf, zf - 1.0),
                ),
                lerp(
                    u,
                    grad(self.p[ab + 1] as i32, xf, yf - 1.0, zf - 1.0),
                    grad(self.p[bb + 1] as i32, xf - 1.0, yf - 1.0, zf - 1.0),
                ),
            ),
        )
    }

    /// 2D sample (y=0), used for biome noise at surface level.
    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        self.sample(x, 0.0, z)
    }
}

#[inline]
fn floor(x: f64) -> f64 {
    x.floor()
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

#[inline]
fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    // Standard 12-gradient table from Perlin's reference implementation
    match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let mut rng = JavaRandom::new(42);
        let noise = ImprovedNoise::new(&mut rng);
        let a = noise.sample(1.0, 2.0, 3.0);
        let mut rng2 = JavaRandom::new(42);
        let noise2 = ImprovedNoise::new(&mut rng2);
        let b = noise2.sample(1.0, 2.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_range() {
        let mut rng = JavaRandom::new(0);
        let noise = ImprovedNoise::new(&mut rng);
        for i in 0..100 {
            let v = noise.sample(i as f64 * 0.1, 0.0, 0.0);
            assert!(v >= -1.5 && v <= 1.5, "noise value out of range: {v}");
        }
    }
}
