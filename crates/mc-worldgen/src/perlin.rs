/// Improved Perlin Noise — initialized from Xoroshiro128++ (Minecraft 1.18+).
/// Based on Ken Perlin's 2002 reference implementation.
/// Matches cubiomes' xPerlinInit / samplePerlin exactly.
use crate::xoroshiro::Xoroshiro;

#[derive(Clone)]
pub struct ImprovedNoise {
    perm: [u8; 257], // permutation table (256 + wrap)
    pub x_offset: f64,
    pub y_offset: f64,
    pub z_offset: f64,
    pub amplitude: f64,  // set by OctaveNoise
    pub lacunarity: f64, // set by OctaveNoise
    // Pre-computed y=0 values for fast 2D sampling
    h2: i32,
    d2: f64,
    t2: f64,
}

impl ImprovedNoise {
    /// Initialize from Xoroshiro128++ RNG (cubiomes xPerlinInit).
    pub fn new(rng: &mut Xoroshiro) -> Self {
        let x_offset = rng.next_double() * 256.0;
        let y_offset = rng.next_double() * 256.0;
        let z_offset = rng.next_double() * 256.0;

        // Initialize permutation table
        let mut perm = [0u8; 257];
        for i in 0..256u16 {
            perm[i as usize] = i as u8;
        }
        // Fisher-Yates shuffle using Xoroshiro
        for i in 0..256usize {
            let j = rng.next_int((256 - i) as u32) as usize + i;
            perm.swap(i, j);
        }
        perm[256] = perm[0]; // wrap

        // Pre-compute y=0 values
        let i2 = y_offset.floor();
        let d2 = y_offset - i2;
        let h2 = i2 as i32;
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);

        Self {
            perm,
            x_offset,
            y_offset,
            z_offset,
            amplitude: 1.0,
            lacunarity: 1.0,
            h2,
            d2,
            t2,
        }
    }

    /// Create from pre-built permutation table and offsets (for JavaRandom init).
    pub fn from_raw(perm: [u8; 257], x_offset: f64, y_offset: f64, z_offset: f64) -> Self {
        let i2 = y_offset.floor();
        let d2 = y_offset - i2;
        let h2 = i2 as i32;
        let t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);

        Self {
            perm,
            x_offset,
            y_offset,
            z_offset,
            amplitude: 1.0,
            lacunarity: 1.0,
            h2,
            d2,
            t2,
        }
    }

    /// Sample 3D Perlin noise at (x, y, z).
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let dx = x + self.x_offset;
        let dy = y + self.y_offset;
        let dz = z + self.z_offset;

        let i = dx.floor() as i32;
        let j = dy.floor() as i32;
        let k = dz.floor() as i32;

        let d0 = dx - i as f64;
        let d1 = dy - j as f64;
        let d2 = dz - k as f64;

        let t0 = fade(d0);
        let t1 = fade(d1);
        let t2 = fade(d2);

        self.sample_inner(i, j, k, d0, d1, d2, t0, t1, t2)
    }

    /// Optimized 2D sample (y=0): uses pre-computed y offset values.
    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        let dx = x + self.x_offset;
        let dz = z + self.z_offset;

        let i = dx.floor() as i32;
        let k = dz.floor() as i32;

        let d0 = dx - i as f64;
        let d2 = dz - k as f64;

        let t0 = fade(d0);
        let t2 = fade(d2);

        self.sample_inner(i, self.h2, k, d0, self.d2, d2, t0, self.t2, t2)
    }

    #[inline]
    fn sample_inner(
        &self,
        i: i32,
        j: i32,
        k: i32,
        d0: f64,
        d1: f64,
        d2: f64,
        t0: f64,
        t1: f64,
        t2: f64,
    ) -> f64 {
        let p = &self.perm;
        let ii = (i & 0xFF) as usize;
        let jj = (j & 0xFF) as usize;
        let kk = (k & 0xFF) as usize;

        let a = (p[ii] as usize).wrapping_add(jj);
        let b = (p[(ii + 1) & 0xFF] as usize).wrapping_add(jj);

        let aa = (p[a & 0xFF] as usize).wrapping_add(kk);
        let ab = (p[(a + 1) & 0xFF] as usize).wrapping_add(kk);
        let ba = (p[b & 0xFF] as usize).wrapping_add(kk);
        let bb = (p[(b + 1) & 0xFF] as usize).wrapping_add(kk);

        lerp(
            t2,
            lerp(
                t1,
                lerp(
                    t0,
                    grad(p[aa & 0xFF], d0, d1, d2),
                    grad(p[ba & 0xFF], d0 - 1.0, d1, d2),
                ),
                lerp(
                    t0,
                    grad(p[ab & 0xFF], d0, d1 - 1.0, d2),
                    grad(p[bb & 0xFF], d0 - 1.0, d1 - 1.0, d2),
                ),
            ),
            lerp(
                t1,
                lerp(
                    t0,
                    grad(p[(aa + 1) & 0xFF], d0, d1, d2 - 1.0),
                    grad(p[(ba + 1) & 0xFF], d0 - 1.0, d1, d2 - 1.0),
                ),
                lerp(
                    t0,
                    grad(p[(ab + 1) & 0xFF], d0, d1 - 1.0, d2 - 1.0),
                    grad(p[(bb + 1) & 0xFF], d0 - 1.0, d1 - 1.0, d2 - 1.0),
                ),
            ),
        )
    }
}

#[inline(always)]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline(always)]
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

/// Gradient function matching Minecraft's indexedLerp exactly.
#[inline(always)]
fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    match hash & 0xF {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        11 => -y - z,
        12 => x + y,  // Minecraft variant
        13 => -y + z, // Minecraft variant (not -x+y)
        14 => -x + y, // Minecraft variant
        15 => -y - z, // Minecraft variant (not -x-y)
        _ => unreachable!(),
    }
}
