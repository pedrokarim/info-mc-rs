/// Minecraft structure placement — exact cubiomes algorithm.
/// Two placement methods:
/// - getFeaturePos: single nextInt (most structures)
/// - getLargeStructurePos: double nextInt triangular (monuments, mansions, end cities)

use crate::java_random::JavaRandom;
use crate::xoroshiro::Xoroshiro;

#[derive(Debug, Clone, Copy)]
pub struct StructureConfig {
    pub salt: i32,
    pub region_size: i32,  // = spacing in chunks
    pub chunk_range: i32,  // random range within region
    pub placement: PlacementType,
    pub rarity: f32,       // for DecoratorRarity placement (e.g. 1/700 for End Gateway)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementType {
    Feature,         // getFeaturePos: single nextInt per axis
    Large,           // getLargeStructurePos: double nextInt per axis (triangular)
    LargeEndCity,    // getLargeStructurePos + distance check >= 1008 blocks
    DecoratorRarity, // per-chunk decorator with rarity check (End Gateway)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StructureType {
    Village = 0,
    DesertTemple = 1,
    JungleTemple = 2,
    SwampHut = 3,
    Igloo = 4,
    OceanMonument = 5,
    WoodlandMansion = 6,
    PillagerOutpost = 7,
    Stronghold = 8,
    OceanRuin = 9,
    Shipwreck = 10,
    BuriedTreasure = 11,
    RuinedPortal = 12,
    AncientCity = 13,
    TrailRuin = 14,
    TrialChamber = 15,
    NetherFortress = 16,
    Bastion = 17,
    Mineshaft = 18,
    Dungeon = 19,
    DesertWell = 20,
    Fossil = 21,
    Spawn = 22,
    EndCity = 23,
    EndGateway = 24,
    EndCityShip = 25,
}

impl StructureType {
    /// Structure config from cubiomes (salt, regionSize, chunkRange, placement type).
    pub fn config(self) -> StructureConfig {
        use PlacementType::*;
        let (salt, region_size, chunk_range, placement, rarity): (i32, i32, i32, PlacementType, f32) = match self {
            Self::Village =>         (10387312,  34, 26, Feature, 0.0),
            Self::DesertTemple =>    (14357617,  32, 24, Feature, 0.0),
            Self::JungleTemple =>    (14357619,  32, 24, Feature, 0.0),
            Self::SwampHut =>        (14357620,  32, 24, Feature, 0.0),
            Self::Igloo =>           (14357618,  32, 24, Feature, 0.0),
            Self::OceanMonument =>   (10387313,  32, 27, Large,   0.0),
            Self::WoodlandMansion => (10387319,  80, 60, Large,   0.0),
            Self::PillagerOutpost => (165745296, 32, 24, Feature, 0.0),
            Self::Stronghold =>      (0,         1,  0,  Feature, 0.0),
            Self::OceanRuin =>       (14357621,  20, 12, Feature, 0.0),
            Self::Shipwreck =>       (165745295, 24, 20, Feature, 0.0),
            Self::BuriedTreasure =>  (0,         1,  0,  Feature, 0.0),
            Self::RuinedPortal =>    (34222645,  40, 25, Feature, 0.0),
            Self::AncientCity =>     (20083232,  24, 16, Feature, 0.0),
            Self::TrailRuin =>       (83469867,  34, 26, Feature, 0.0),
            Self::TrialChamber =>    (94251327,  34, 22, Feature, 0.0),
            Self::NetherFortress =>  (30084232,  27, 23, Feature, 0.0),
            Self::Bastion =>         (30084232,  27, 23, Feature, 0.0),
            Self::Mineshaft =>       (0,         1,  0,  Feature, 0.0),
            Self::Dungeon =>         (0,         1,  0,  Feature, 0.0),
            Self::DesertWell =>      (40002,     1,  0,  Feature, 0.0),
            Self::Fossil =>          (0,         1,  0,  Feature, 0.0),
            Self::Spawn =>           (0,         1,  0,  Feature, 0.0),
            Self::EndCity =>         (10387313,  20, 9,  LargeEndCity,    0.0),
            // End Gateway: salt=40000, rarity=1/700 per chunk (1.18+ Xoroshiro decorator)
            Self::EndGateway =>      (40000,     1,  1,  DecoratorRarity, 1.0 / 700.0),
            // End City Ship is a variant of End City — same placement, different render
            Self::EndCityShip =>     (10387313,  20, 9,  LargeEndCity,    0.0),
        };
        StructureConfig { salt, region_size, chunk_range, placement, rarity }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Village => "village",
            Self::DesertTemple => "desert-temple",
            Self::JungleTemple => "jungle-temple",
            Self::SwampHut => "witch-hut",
            Self::Igloo => "igloo",
            Self::OceanMonument => "ocean-monument",
            Self::WoodlandMansion => "mansion",
            Self::PillagerOutpost => "pillager-outpost",
            Self::Stronghold => "stronghold",
            Self::OceanRuin => "ocean-ruin",
            Self::Shipwreck => "shipwreck",
            Self::BuriedTreasure => "buried-treasure",
            Self::RuinedPortal => "ruined-portal",
            Self::AncientCity => "ancient-city",
            Self::TrailRuin => "trail-ruin",
            Self::TrialChamber => "trial-chamber",
            Self::NetherFortress => "nether-fortress",
            Self::Bastion => "bastion-treasure",
            Self::Mineshaft => "mineshaft",
            Self::Dungeon => "dungeon",
            Self::DesertWell => "desert-well",
            Self::Fossil => "fossil",
            Self::Spawn => "spawn",
            Self::EndCity => "end-city",
            Self::EndGateway => "end-gateway",
            Self::EndCityShip => "end-city-ship",
        }
    }

    pub fn overworld_types() -> &'static [StructureType] {
        &[
            Self::Village, Self::DesertTemple, Self::JungleTemple, Self::SwampHut,
            Self::Igloo, Self::OceanMonument, Self::WoodlandMansion, Self::PillagerOutpost,
            Self::OceanRuin, Self::Shipwreck, Self::RuinedPortal, Self::AncientCity,
            Self::TrailRuin, Self::TrialChamber, Self::DesertWell, Self::Fossil,
        ]
    }

    pub fn nether_types() -> &'static [StructureType] {
        &[Self::NetherFortress, Self::Bastion, Self::RuinedPortal]
    }

    pub fn end_types() -> &'static [StructureType] {
        &[Self::EndCity, Self::EndGateway]
    }
}

/// getFeaturePos — single nextInt per axis (cubiomes exact).
/// Uses Java LCG directly for speed.
fn get_feature_pos(config: &StructureConfig, seed: i64, region_x: i32, region_z: i32) -> (i32, i32) {
    let s = (seed as u64)
        .wrapping_add((region_x as u64).wrapping_mul(341873128712))
        .wrapping_add((region_z as u64).wrapping_mul(132897987541))
        .wrapping_add(config.salt as u64);

    let mut rng = JavaRandom::new(s as i64);

    let r = config.chunk_range;
    let cx = if r > 0 { rng.next_int(r) } else { 0 };
    let cz = if r > 0 { rng.next_int(r) } else { 0 };

    let bx = ((region_x as i64 * config.region_size as i64 + cx as i64) << 4) as i32;
    let bz = ((region_z as i64 * config.region_size as i64 + cz as i64) << 4) as i32;
    (bx, bz)
}

/// getLargeStructurePos — double nextInt per axis (triangular distribution).
/// Used by Ocean Monument, Woodland Mansion, End City.
fn get_large_structure_pos(config: &StructureConfig, seed: i64, region_x: i32, region_z: i32) -> (i32, i32) {
    const K: u64 = 0x5DEECE66D;
    const M: u64 = (1u64 << 48) - 1;
    const B: u64 = 0xB;

    let mut s = (seed as u64)
        .wrapping_add((region_x as u64).wrapping_mul(341873128712))
        .wrapping_add((region_z as u64).wrapping_mul(132897987541))
        .wrapping_add(config.salt as u64);

    // setSeed equivalent: s = (s ^ K) & M
    s = (s ^ K) & M;

    let r = config.chunk_range as u64;

    // x = (next() % r + next() % r) / 2  — triangular distribution
    s = (s.wrapping_mul(K).wrapping_add(B)) & M;
    let mut cx = (s >> 17) % r;
    s = (s.wrapping_mul(K).wrapping_add(B)) & M;
    cx = cx.wrapping_add((s >> 17) % r);

    s = (s.wrapping_mul(K).wrapping_add(B)) & M;
    let mut cz = (s >> 17) % r;
    s = (s.wrapping_mul(K).wrapping_add(B)) & M;
    cz = cz.wrapping_add((s >> 17) % r);

    cx >>= 1;
    cz >>= 1;

    let bx = ((region_x as i64 * config.region_size as i64 + cx as i64) << 4) as i32;
    let bz = ((region_z as i64 * config.region_size as i64 + cz as i64) << 4) as i32;
    (bx, bz)
}

/// Compute the population seed for a chunk (1.18+ Xoroshiro version).
/// Matches cubiomes getPopulationSeed for MC >= 1.18.
fn get_population_seed(world_seed: i64, x: i32, z: i32) -> i64 {
    let mut xr = Xoroshiro::from_seed(world_seed);
    let mut a = xr.next_long_j();
    let mut b = xr.next_long_j();
    a |= 1;
    b |= 1;
    ((x as i64).wrapping_mul(a as i64).wrapping_add((z as i64).wrapping_mul(b as i64)))
        ^ world_seed
}

/// Decorator placement (per-chunk with rarity check).
/// Used for End Gateway, Geode, Desert Well, etc. in 1.18+.
fn get_decorator_pos(config: &StructureConfig, seed: i64, chunk_x: i32, chunk_z: i32) -> Option<(i32, i32)> {
    let block_x = chunk_x * 16;
    let block_z = chunk_z * 16;

    // Get population seed for this chunk
    let pop_seed = get_population_seed(seed, block_x, block_z);

    // Initialize Xoroshiro with population_seed + salt
    let mut xr = Xoroshiro::from_seed(pop_seed.wrapping_add(config.salt as i64));

    // Rarity check
    if xr.next_float() >= config.rarity {
        return None;
    }

    // Position within chunk
    let dx = xr.next_int_j(16);
    let dz = xr.next_int_j(16);
    Some((block_x + dx, block_z + dz))
}

/// Find the structure position in a given region.
pub fn get_structure_pos(
    seed: i64,
    structure: StructureType,
    region_x: i32,
    region_z: i32,
) -> Option<(i32, i32)> {
    let config = structure.config();
    if config.chunk_range <= 0 { return None; }

    let (bx, bz) = match config.placement {
        PlacementType::Feature => get_feature_pos(&config, seed, region_x, region_z),
        PlacementType::Large => get_large_structure_pos(&config, seed, region_x, region_z),
        PlacementType::LargeEndCity => {
            let (bx, bz) = get_large_structure_pos(&config, seed, region_x, region_z);
            // End Cities must be >= 1008 blocks from origin
            if (bx as i64) * (bx as i64) + (bz as i64) * (bz as i64) < 1008 * 1008 {
                return None;
            }
            (bx, bz)
        }
        PlacementType::DecoratorRarity => {
            // Decorator features use per-chunk placement, not per-region.
            // Caller should use find_structures_in_area which handles this.
            return None;
        }
    };

    Some((bx, bz))
}

/// Find all structures of a given type within a block-coordinate area.
pub fn find_structures_in_area(
    seed: i64,
    structure: StructureType,
    block_x: i32,
    block_z: i32,
    block_w: i32,
    block_h: i32,
) -> Vec<(i32, i32)> {
    let config = structure.config();
    if config.chunk_range <= 0 { return vec![]; }

    // Decorator features (End Gateway, etc.) iterate per-chunk
    if config.placement == PlacementType::DecoratorRarity {
        let cx0 = block_x >> 4;
        let cz0 = block_z >> 4;
        let cx1 = (block_x + block_w) >> 4;
        let cz1 = (block_z + block_h) >> 4;
        let mut results = Vec::new();
        for cx in cx0..=cx1 {
            for cz in cz0..=cz1 {
                if let Some((bx, bz)) = get_decorator_pos(&config, seed, cx, cz) {
                    if bx >= block_x && bx < block_x + block_w
                        && bz >= block_z && bz < block_z + block_h
                    {
                        results.push((bx, bz));
                    }
                }
            }
        }
        return results;
    }

    // Standard region-based placement
    let region_size_blocks = (config.region_size as i64) << 4;
    let region_x0 = floor_div(block_x as i64, region_size_blocks) as i32;
    let region_z0 = floor_div(block_z as i64, region_size_blocks) as i32;
    let region_x1 = floor_div((block_x + block_w) as i64, region_size_blocks) as i32;
    let region_z1 = floor_div((block_z + block_h) as i64, region_size_blocks) as i32;

    let mut results = Vec::new();

    for rx in region_x0..=region_x1 {
        for rz in region_z0..=region_z1 {
            if let Some((bx, bz)) = get_structure_pos(seed, structure, rx, rz) {
                if bx >= block_x && bx < block_x + block_w
                    && bz >= block_z && bz < block_z + block_h
                {
                    results.push((bx, bz));
                }
            }
        }
    }

    results
}

fn floor_div(a: i64, b: i64) -> i64 {
    let d = a / b;
    if (a ^ b) < 0 && d * b != a { d - 1 } else { d }
}
