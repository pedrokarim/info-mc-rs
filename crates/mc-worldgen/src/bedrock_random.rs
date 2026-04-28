/// Bedrock Edition RNG — MT19937 (Mersenne Twister 32-bit).
/// Exact port of chunkbase's MT implementation (class Er + class M).
/// Used for Bedrock structure placement, slime chunks, etc.

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908B0DF;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7FFFFFFF;

pub struct MT19937 {
    mt: [u32; N],
    mti: usize,
}

impl MT19937 {
    pub fn new(seed: u32) -> Self {
        let mut rng = Self {
            mt: [0; N],
            mti: N + 1,
        };
        rng.init_seed(seed);
        rng
    }

    /// init_seed(seed) — matches chunkbase/standard MT19937 init_genrand.
    pub fn init_seed(&mut self, seed: u32) {
        self.mt[0] = seed;
        self.mti = 1;
        while self.mti < N {
            let prev = self.mt[self.mti - 1];
            let t = prev ^ (prev >> 30);
            // (((t & 0xFFFF0000) >> 16) * 1812433253 << 16) + (t & 0xFFFF) * 1812433253 + mti
            let hi = (t >> 16).wrapping_mul(1812433253) << 16;
            let lo = (t & 0xFFFF).wrapping_mul(1812433253);
            self.mt[self.mti] = hi.wrapping_add(lo).wrapping_add(self.mti as u32);
            self.mti += 1;
        }
    }

    /// init_by_array — alternative seeding from a key array.
    pub fn init_by_array(&mut self, key: &[u32]) {
        self.init_seed(19650218);
        let key_len = key.len();
        let mut i = 1usize;
        let mut j = 0usize;
        let mut k = if N > key_len { N } else { key_len };

        while k > 0 {
            let prev = self.mt[i - 1];
            let s = prev ^ (prev >> 30);
            let hi = (s >> 16).wrapping_mul(1664525) << 16;
            let lo = (s & 0xFFFF).wrapping_mul(1664525);
            let mixed = hi.wrapping_add(lo);
            self.mt[i] = (self.mt[i] ^ mixed)
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
            if j >= key_len {
                j = 0;
            }
            k -= 1;
        }

        k = N - 1;
        while k > 0 {
            let prev = self.mt[i - 1];
            let s = prev ^ (prev >> 30);
            let hi = (s >> 16).wrapping_mul(1566083941) << 16;
            let lo = (s & 0xFFFF).wrapping_mul(1566083941);
            let mixed = hi.wrapping_add(lo);
            self.mt[i] = (self.mt[i] ^ mixed).wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
            k -= 1;
        }

        self.mt[0] = 0x80000000;
    }

    /// random_int() — the core MT19937 output function, returns 32-bit u32.
    pub fn random_int(&mut self) -> u32 {
        let mag01 = [0u32, MATRIX_A];
        if self.mti >= N {
            if self.mti == N + 1 {
                self.init_seed(5489);
            }
            for kk in 0..(N - M) {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + M] ^ (y >> 1) ^ mag01[(y & 1) as usize];
            }
            for kk in (N - M)..(N - 1) {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + M - N] ^ (y >> 1) ^ mag01[(y & 1) as usize];
            }
            let y = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            self.mt[N - 1] = self.mt[M - 1] ^ (y >> 1) ^ mag01[(y & 1) as usize];
            self.mti = 0;
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;

        // Tempering
        y ^= y >> 11;
        y ^= (y << 7) & 2636928640; // 0x9D2C5680
        y ^= (y << 15) & 4022730752; // 0xEFC60000
        y ^= y >> 18;

        y
    }

    /// Float in [0, 1) — uses 32-bit resolution like Bedrock.
    pub fn random(&mut self) -> f64 {
        self.random_int() as f64 * (1.0 / 4294967296.0)
    }
}

/// BedrockRandom — matches chunkbase's class M.
/// Wraps MT19937 with a Java-Random-like API.
pub struct BedrockRandom {
    rng: MT19937,
}

impl BedrockRandom {
    pub fn new(seed: i32) -> Self {
        Self {
            rng: MT19937::new(seed as u32),
        }
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.rng.init_seed(seed as u32);
    }

    /// nextInt() — returns a 31-bit non-negative int.
    /// Matches chunkbase: `rng.random_int() >>> 1`.
    pub fn next_int_unbounded(&mut self) -> i32 {
        (self.rng.random_int() >> 1) as i32
    }

    /// nextInt(bound) — returns int in [0, bound).
    /// Matches chunkbase: `os(rng.random_int()) % bound`
    /// where `os(x) = V.fromInt(x).and(0xFFFFFFFF).toNumber()` which is just `x as u32`.
    /// NOTE: Bedrock's nextInt does NOT use rejection sampling (unlike Java).
    pub fn next_int(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "bound must be positive");
        ((self.rng.random_int()) % (bound as u32)) as i32
    }

    /// nextIntRange(min, max) — int in [min, max).
    pub fn next_int_range(&mut self, min: i32, max: i32) -> i32 {
        if min < max {
            min + self.next_int(max - min)
        } else {
            min
        }
    }

    /// nextFloat() — float in [0, 1), fround-truncated (f32 precision).
    pub fn next_float(&mut self) -> f32 {
        self.rng.random() as f32
    }

    /// nextDouble() — 32-bit resolution double in [0, 1).
    pub fn next_double(&mut self) -> f64 {
        self.rng.random()
    }

    /// nextBoolean() — uses bit 27 of random_int (NOT the high bit like Java).
    pub fn next_boolean(&mut self) -> bool {
        (self.rng.random_int() & 0x8000000) != 0
    }

    /// Raw access to the underlying MT19937 for exact chunkbase compatibility.
    pub fn random_int(&mut self) -> u32 {
        self.rng.random_int()
    }
}
