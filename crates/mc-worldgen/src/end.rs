/// End biome generation — exact port of cubiomes mapEndBiome + getEndBiome.
/// Reference: cubiomes/biomenoise.c lines 379-488.
use crate::biomes::Biome;
use crate::java_random::JavaRandom;

/// Squared distance lookup table — cubiomes ds[26]: (25-2*i)² for i in 0..26.
const DS: [u32; 26] = [
    625, 529, 441, 361, 289, 225, 169, 121, 81, 49, 25, 9, 1, 1, 9, 25, 49, 81, 121, 169, 225, 289,
    361, 441, 529, 625,
];

pub struct EndNoise {
    /// Permutation table (257 entries, last is wrap of first)
    d: [u8; 257],
}

impl EndNoise {
    /// Initialize from world seed — exact cubiomes setEndSeed.
    /// Java LCG, skip 17292 calls, then perlinInit.
    pub fn new(seed: i64) -> Self {
        let mut rng = JavaRandom::new(seed);

        // skipNextN(s, 17292)
        for _ in 0..17292 {
            rng.next(32);
        }

        // perlinInit: read 3 doubles (a, b, c) - we discard them, only need d table
        let _a = rng.next_double() * 256.0;
        let _b = rng.next_double() * 256.0;
        let _c = rng.next_double() * 256.0;

        // Build permutation table
        let mut d = [0u8; 257];
        for i in 0..256u16 {
            d[i as usize] = i as u8;
        }
        for i in 0..256usize {
            let j = rng.next_int((256 - i) as i32) as usize + i;
            d.swap(i, j);
        }
        d[256] = d[0];

        Self { d }
    }

    /// 2D Simplex noise — exact cubiomes sampleSimplex2D.
    fn sample_simplex_2d(&self, x: f64, y: f64) -> f64 {
        let skew: f64 = 0.5 * (3.0_f64.sqrt() - 1.0);
        let unskew: f64 = (3.0 - 3.0_f64.sqrt()) / 6.0;

        let hf = (x + y) * skew;
        let hx = (x + hf).floor() as i32;
        let hz = (y + hf).floor() as i32;
        let mhxz = (hx as f64 + hz as f64) * unskew;
        let x0 = x - (hx as f64 - mhxz);
        let y0 = y - (hz as f64 - mhxz);

        let (offx, offz): (i32, i32) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - offx as f64 + unskew;
        let y1 = y0 - offz as f64 + unskew;
        let x2 = x0 - 1.0 + 2.0 * unskew;
        let y2 = y0 - 1.0 + 2.0 * unskew;

        let d = &self.d;
        // gi0 = d[0xff & (d[0xff & hz] + hx)]
        let mut gi0 = d[(hz & 0xFF) as usize] as i32;
        gi0 = d[((gi0 + hx) & 0xFF) as usize] as i32;

        let mut gi1 = d[((hz + offz) & 0xFF) as usize] as i32;
        gi1 = d[((gi1 + hx + offx) & 0xFF) as usize] as i32;

        let mut gi2 = d[((hz + 1) & 0xFF) as usize] as i32;
        gi2 = d[((gi2 + hx + 1) & 0xFF) as usize] as i32;

        let mut t = 0.0;
        t += simplex_grad(gi0 % 12, x0, y0, 0.5);
        t += simplex_grad(gi1 % 12, x1, y1, 0.5);
        t += simplex_grad(gi2 % 12, x2, y2, 0.5);
        70.0 * t
    }
}

/// Simplex gradient (cubiomes simplexGrad with z=0).
/// Uses indexedLerp gradient table evaluated at z=0.
#[inline]
fn simplex_grad(idx: i32, x: f64, y: f64, d: f64) -> f64 {
    let mut t = d - x * x - y * y;
    if t < 0.0 {
        return 0.0;
    }
    t *= t;
    // indexedLerp(idx, x, y, 0) — cubiomes' 16-case gradient with z=0
    let dot = match idx & 0xF {
        0 => x + y,   // a + b
        1 => -x + y,  // -a + b
        2 => x - y,   // a - b
        3 => -x - y,  // -a - b
        4 => x,       // a + c (c=0)
        5 => -x,      // -a + c
        6 => x,       // a - c
        7 => -x,      // -a - c
        8 => y,       // b + c
        9 => -y,      // -b + c
        10 => y,      // b - c
        11 => -y,     // -b - c
        12 => x + y,  // a + b (variant)
        13 => -y,     // -b + c
        14 => -x + y, // -a + b (variant)
        15 => -y,     // -b - c
        _ => 0.0,
    };
    t * t * dot
}

pub struct EndBiomeSource {
    noise: EndNoise,
}

impl EndBiomeSource {
    pub fn new(seed: i64) -> Self {
        Self {
            noise: EndNoise::new(seed),
        }
    }

    /// Get biome at block coordinates.
    /// EXACT port of chunkbase function 178 (biomeproviderend_get_noise_biome).
    /// Input is BIOME coordinates (block / 4), but we accept block coords and convert.
    pub fn get_biome(&self, block_x: i32, block_z: i32) -> Biome {
        // Convert block to biome coordinates (block / 4, signed shift)
        let x = block_x >> 2;
        let z = block_z >> 2;

        // Chunkbase computes: x_q = (x*4) >> 4 = x/4, z_q = (z*4) >> 4 = z/4
        // So this is biome coord >> 2 = chunk coord (block / 16)
        let x_q = (x << 2) >> 4; // = x / 4 effectively
        let z_q = (z << 2) >> 4;

        // Central island check: x_q² + z_q² < 4097
        let dist_sq = (x_q as i64) * (x_q as i64) + (z_q as i64) * (z_q as i64);
        if dist_sq < 4097 {
            return Biome::TheEnd;
        }

        // hx = (x*4) >> 3 | 1 = (x/2) | 1 (force odd)
        // hz = (z*4) >> 3 | 1
        let hx: i32 = ((x << 2) >> 3) | 1;
        let hz: i32 = ((z << 2) >> 3) | 1;

        // Compute height using chunkbase function 127
        let height = self.compute_end_height(hx, hz);

        // Biome thresholds from chunkbase function 178:
        //   height > 40 → end_highlands
        //   height >= 0 → end_midlands
        //   height < -20 → small_end_islands
        //   else → end_barrens
        if height > 40.0 {
            Biome::EndHighlands
        } else if height >= 0.0 {
            Biome::EndMidlands
        } else if height < -20.0 {
            Biome::SmallEndIslands
        } else {
            Biome::EndBarrens
        }
    }

    /// EXACT port of chunkbase function 127 (compute_end_height).
    /// Returns float height in [-100, 80].
    fn compute_end_height(&self, hx: i32, hz: i32) -> f32 {
        // half_x = hx / 2 (signed), x_remainder = hx - 2*half_x
        let half_z = hz / 2;
        let z_remainder = hz - 2 * half_z;
        let half_x = hx / 2;
        let x_remainder = hx - 2 * half_x;

        // Initial height from distance to origin
        // Normal path: integer multiplication
        let initial_dist_sq = (hz * hz + hx * hx) as f32;

        // result = clamp(100 - 8 * sqrt(initial_dist_sq), -100, 80)
        let mut result = 100.0_f32 - 8.0 * initial_dist_sq.sqrt();
        if result < -100.0 {
            result = -100.0;
        }
        if result > 80.0 {
            result = 80.0;
        }

        // Iterate 25x25 window: outer (X dir) ∈ [-12, 12], inner (Z dir) ∈ [-12, 12]
        for outer in -12_i32..=12 {
            let outer_plus_half_x = outer + half_x;
            let outer_abs_3439 = (outer_plus_half_x as f32).abs() * 3439.0;
            let outer_plus_half_x_f64 = outer_plus_half_x as f64;
            let outer_sq_i64 = (outer_plus_half_x as i64) * (outer_plus_half_x as i64);

            // f1 = (x_remainder - 2*outer)²
            let f1 = (x_remainder - outer * 2) as f32;
            let f1_sq = f1 * f1;

            for inner in -12_i32..=12 {
                let inner_plus_half_z = inner + half_z;
                let inner_sq_i64 = (inner_plus_half_z as i64) * (inner_plus_half_z as i64);
                let total_sq = inner_sq_i64 + outer_sq_i64;

                // Skip if too close to origin
                if total_sq < 4097 {
                    continue;
                }

                // Sample simplex noise at (outer+half_x, inner+half_z)
                let simplex = self
                    .noise
                    .sample_simplex_2d(outer_plus_half_x_f64, inner_plus_half_z as f64);
                if simplex >= -0.9 {
                    continue;
                }

                // Compute new height candidate
                let f2 = (z_remainder - inner * 2) as f32;
                let f2_sq = f2 * f2;
                let radius_sq = f1_sq + f2_sq;
                let dist = radius_sq.sqrt();

                // Elevation factor: ((|outer+half_x|*3439 + |inner+half_z|*147) % 13 + 9)
                let inner_abs_147 = (inner_plus_half_z as f32).abs() * 147.0;
                let raw = outer_abs_3439 + inner_abs_147;
                let mod13 = raw % 13.0;
                let elev_factor = mod13 + 9.0;

                // height = 100 - dist * elev_factor, clamped to [-100, 80]
                let mut new_height = 100.0_f32 - dist * elev_factor;
                if new_height < -100.0 {
                    new_height = -100.0;
                }
                if new_height > 80.0 {
                    new_height = 80.0;
                }

                // Take MAX of result and new_height
                // (closest island = highest height = highlands)
                if new_height > result {
                    result = new_height;
                }
            }
        }

        result
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
