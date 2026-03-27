/// Java's `java.util.Random` — a 48-bit Linear Congruential Generator.
/// Bit-perfect reproduction of Java's implementation for Minecraft seed-based generation.

const MULTIPLIER: i64 = 0x5DEECE66D;
const ADDEND: i64 = 0xB;
const MASK: i64 = (1 << 48) - 1;

#[derive(Clone)]
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    pub fn new(seed: i64) -> Self {
        Self {
            seed: (seed ^ MULTIPLIER) & MASK,
        }
    }

    /// Create with raw internal seed (no XOR scramble).
    pub fn from_internal(seed: i64) -> Self {
        Self { seed: seed & MASK }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MASK;
    }

    /// Advance LCG and return top `bits` bits as i32.
    pub fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND)) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    pub fn next_int(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "bound must be positive");

        // Power of two fast path
        if (bound & (bound - 1)) == 0 {
            return ((bound as i64).wrapping_mul(self.next(31) as i64) >> 31) as i32;
        }

        // Rejection sampling (exact Java behavior)
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits - val + (bound - 1) >= 0 {
                return val;
            }
        }
    }

    pub fn next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    pub fn next_double(&mut self) -> f64 {
        let hi = (self.next(26) as i64) << 27;
        let lo = self.next(27) as i64;
        (hi + lo) as f64 / ((1_i64 << 53) as f64)
    }

    /// Skip `n` calls to next(). Used by OctaveNoise to advance state for skipped octaves.
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next(1);
        }
    }

    /// Java's `String.hashCode()` for text seeds.
    pub fn string_hash_code(s: &str) -> i32 {
        let mut hash: i32 = 0;
        for c in s.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as i32);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_sequence() {
        // Java: new Random(12345).nextInt(100) sequence
        let mut rng = JavaRandom::new(12345);
        // Verify first few nextInt(100) match Java output
        let vals: Vec<i32> = (0..5).map(|_| rng.next_int(100)).collect();
        assert_eq!(vals, vec![51, 80, 41, 28, 55]);
    }

    #[test]
    fn test_string_hash() {
        // Java: "Hello".hashCode() == 69609650
        assert_eq!(JavaRandom::string_hash_code("Hello"), 69609650);
    }

    #[test]
    fn test_next_long() {
        let mut rng = JavaRandom::new(0);
        let v = rng.next_long();
        // Java: new Random(0).nextLong() == -4962768465676381896
        assert_eq!(v, -4962768465676381896);
    }
}
