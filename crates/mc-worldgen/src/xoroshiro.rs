/// Xoroshiro128++ RNG — the RNG used by Minecraft 1.18+ for noise generation.
/// This replaces java.util.Random for climate/biome noise.

#[derive(Clone, Debug)]
pub struct Xoroshiro {
    pub lo: u64,
    pub hi: u64,
}

impl Xoroshiro {
    /// Initialize from a world seed using Stafford's mixer (splitmix64 variant).
    /// This is the equivalent of cubiomes' xSetSeed().
    pub fn from_seed(seed: i64) -> Self {
        let value = seed as u64;

        const XL: u64 = 0x9e3779b97f4a7c15;
        const XH: u64 = 0x6a09e667f3bcc909;
        const A: u64 = 0xbf58476d1ce4e5b9;
        const B: u64 = 0x94d049bb133111eb;

        let mut l = value ^ XH;
        let mut h = l.wrapping_add(XL);

        l = (l ^ (l >> 30)).wrapping_mul(A);
        h = (h ^ (h >> 30)).wrapping_mul(A);
        l = (l ^ (l >> 27)).wrapping_mul(B);
        h = (h ^ (h >> 27)).wrapping_mul(B);
        l ^= l >> 31;
        h ^= h >> 31;

        Self { lo: l, hi: h }
    }

    /// Create from raw lo/hi state (used when XOR-ing with MD5 hashes).
    pub fn from_raw(lo: u64, hi: u64) -> Self {
        Self { lo, hi }
    }

    /// Xoroshiro128++ next — generates a 64-bit value.
    pub fn next_long(&mut self) -> u64 {
        let l = self.lo;
        let h = self.hi;
        let n = l.wrapping_add(h).rotate_left(17).wrapping_add(l);
        let h = h ^ l;
        self.lo = l.rotate_left(49) ^ h ^ (h << 21);
        self.hi = h.rotate_left(28);
        n
    }

    /// Next double in [0, 1).
    pub fn next_double(&mut self) -> f64 {
        (self.next_long() >> 11) as f64 * 1.1102230246251565e-16
    }

    /// Next int in [0, n) using unsigned multiplication rejection sampling.
    pub fn next_int(&mut self, n: u32) -> u32 {
        let mut r = (self.next_long() as u32 as u64).wrapping_mul(n as u64);
        if (r as u32) < n {
            let threshold = n.wrapping_neg() % n; // (~n + 1) % n
            while (r as u32) < threshold {
                r = (self.next_long() as u32 as u64).wrapping_mul(n as u64);
            }
        }
        (r >> 32) as u32
    }
}

/// Compute MD5 hash and return as (lo, hi) u64 pair.
/// lo = first 8 bytes little-endian, hi = last 8 bytes little-endian.
pub fn md5_to_u64_pair(data: &[u8]) -> (u64, u64) {
    let digest = md5_hash(data);
    let lo = u64::from_le_bytes([
        digest[0], digest[1], digest[2], digest[3],
        digest[4], digest[5], digest[6], digest[7],
    ]);
    let hi = u64::from_le_bytes([
        digest[8], digest[9], digest[10], digest[11],
        digest[12], digest[13], digest[14], digest[15],
    ]);
    (lo, hi)
}

// ===== Minimal MD5 implementation (RFC 1321) =====
// We only need this for short strings, so no optimization needed.

fn md5_hash(data: &[u8]) -> [u8; 16] {
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    // Pre-processing: add padding
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    // Process each 512-bit (64-byte) block
    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
        }

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64u32 {
            let (f, g) = match i {
                0..=15 => ((b & c) | ((!b) & d), i as usize),
                16..=31 => ((d & b) | ((!d) & c), ((5 * i + 1) % 16) as usize),
                32..=47 => (b ^ c ^ d, ((3 * i + 5) % 16) as usize),
                _ => (c ^ (b | (!d)), ((7 * i) % 16) as usize),
            };

            let f = f.wrapping_add(a).wrapping_add(K[i as usize]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i as usize]));
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&a0.to_le_bytes());
    result[4..8].copy_from_slice(&b0.to_le_bytes());
    result[8..12].copy_from_slice(&c0.to_le_bytes());
    result[12..16].copy_from_slice(&d0.to_le_bytes());
    result
}

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

// ===== Pre-computed MD5 hashes for Minecraft resource locations =====

/// MD5 hashes for climate parameter resource locations.
/// Format: (lo, hi) where lo = first 8 bytes LE, hi = last 8 bytes LE.
pub mod climate_md5 {
    pub const SHIFT: (u64, u64) = (0x080518cf6af25384, 0x3f3dfb40a54febd5);
    pub const TEMPERATURE: (u64, u64) = (0x5c7e6b29735f0d7f, 0xf7d86f1bbc734988);
    pub const HUMIDITY: (u64, u64) = (0x81bb4d22e8dc168e, 0xf1c8b4bea16303cd);
    pub const CONTINENTALNESS: (u64, u64) = (0x83886c9d0ae3a662, 0xafa638a61b42e8ad);
    pub const EROSION: (u64, u64) = (0xd02491e6058f6fd8, 0x4792512c94c17a80);
    pub const WEIRDNESS: (u64, u64) = (0xefc8ef4d36102b34, 0x1beeeb324a0f24ea);
}

/// MD5 hashes for octave strings "octave_-12" through "octave_0".
/// Index 0 = "octave_-12", index 12 = "octave_0".
pub const OCTAVE_MD5: [(u64, u64); 13] = [
    (0xb198de63a8012672, 0x7b84cad43ef7b5a8), // octave_-12
    (0x0fd787bfbc403ec3, 0x74a4a31ca21b48b8), // octave_-11
    (0x36d326eed40efeb2, 0x5be9ce18223c636a), // octave_-10
    (0x082fe255f8be6631, 0x4e96119e22dedc81), // octave_-9
    (0x0ef68ec68504005e, 0x48b6bf93a2789640), // octave_-8
    (0xf11268128982754f, 0x257a1d670430b0aa), // octave_-7
    (0xe51c98ce7d1de664, 0x5f9478a733040c45), // octave_-6
    (0x6d7b49e7e429850a, 0x2e3063c622a24777), // octave_-5
    (0xbd90d5377ba1b762, 0xc07317d419a7548d), // octave_-4
    (0x53d39c6752dac858, 0xbcd1c5a80ab65b3e), // octave_-3
    (0xb4a24d7a84e7677b, 0x023ff9668e89b5c4), // octave_-2
    (0xdffa22b534c5f608, 0xb9b67517d3665ca9), // octave_-1
    (0xd50708086cef4d7c, 0x6e1651ecc7f43309), // octave_0
];

/// Get the MD5 pair for a given octave number (-12..0).
/// The OCTAVE_MD5 array is indexed as: octave_number + 12.
pub fn octave_md5(octave_num: i32) -> (u64, u64) {
    let idx = (octave_num + 12) as usize;
    if idx < OCTAVE_MD5.len() {
        OCTAVE_MD5[idx]
    } else {
        // For octaves outside pre-computed range, compute on the fly
        let s = format!("octave_{}", octave_num);
        md5_to_u64_pair(s.as_bytes())
    }
}
