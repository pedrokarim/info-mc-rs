use crate::java_random::JavaRandom;

/// Determine if a chunk is a slime chunk for the given world seed.
/// This algorithm is identical across all Minecraft Java Edition versions.
pub fn is_slime_chunk(world_seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
    let cx = chunk_x as i64;
    let cz = chunk_z as i64;

    let scramble = world_seed
        .wrapping_add(cx.wrapping_mul(cx).wrapping_mul(0x4c1906))
        .wrapping_add(cx.wrapping_mul(0x5ac0db))
        .wrapping_add(cz.wrapping_mul(cz).wrapping_mul(0x4307a7))
        .wrapping_add(cz.wrapping_mul(0x5f24f))
        ^ 0x3ad8025f;

    let mut rng = JavaRandom::new(scramble);
    rng.next_int(10) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_slime_chunks() {
        // Seed 0: chunk (0,0) is NOT a slime chunk
        assert!(!is_slime_chunk(0, 0, 0));

        // Seed 0: sample chunks verified against Java Random reproduction
        assert!(!is_slime_chunk(0, -1, -1));
        assert!(!is_slime_chunk(0, -3, -1));
        assert!(!is_slime_chunk(0, 7, 2));
    }

    #[test]
    fn test_seed_12345() {
        // Seed 12345: chunk (0,0) test
        assert!(!is_slime_chunk(12345, 0, 0));
    }
}
