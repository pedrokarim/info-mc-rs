/// Minecraft structure placement — exact chunkbase algorithm.
/// Supports Java and Bedrock editions with different RNG and configs.

use crate::java_random::JavaRandom;
use crate::bedrock_random::BedrockRandom;
use crate::xoroshiro::Xoroshiro;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edition {
    Java,
    Bedrock,
}

#[derive(Debug, Clone, Copy)]
pub struct StructureConfig {
    pub salt: i32,
    pub region_size: i32,   // = spacing in chunks
    pub chunk_range: i32,   // random range within region (= spacing - separation)
    pub placement: PlacementType,
    pub rarity: f32,        // for DecoratorRarity placement (e.g. 1/700 for End Gateway)
    pub linear_separation: bool, // true = single nextInt, false = triangular (double + avg)
    /// Force a specific RNG type regardless of edition.
    /// None = use edition's default. Some(Java) = always Java LCG. Some(Bedrock) = always MT.
    pub force_rng: Option<Edition>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementType {
    Feature,         // region-based placement (spacing/separation)
    LargeEndCity,    // Feature + distance check >= 1008 blocks
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
    /// Structure config from chunkbase — edition-aware.
    /// Reference: chunkbase BDEVBixY4VJy.js (worker file)
    pub fn config(self, edition: Edition) -> StructureConfig {
        use PlacementType::*;

        // Helpers for concise definitions
        let feature = |salt, spacing, sep, linear: bool| StructureConfig {
            salt, region_size: spacing, chunk_range: spacing - sep,
            placement: Feature, rarity: 0.0,
            linear_separation: linear, force_rng: None,
        };
        let feature_force_java = |salt, spacing, sep, linear: bool| StructureConfig {
            salt, region_size: spacing, chunk_range: spacing - sep,
            placement: Feature, rarity: 0.0,
            linear_separation: linear, force_rng: Some(Edition::Java),
        };
        let none = StructureConfig {
            salt: 0, region_size: 1, chunk_range: 0,
            placement: Feature, rarity: 0.0,
            linear_separation: false, force_rng: None,
        };

        let is_bedrock = edition == Edition::Bedrock;

        match self {
            // Village — standard feature, same for both editions
            Self::Village =>         feature(10387312,  34, 8,  true),
            Self::DesertTemple =>    feature(14357617,  32, 8,  true),
            Self::JungleTemple =>    feature(14357619,  32, 8,  true),
            Self::SwampHut =>        feature(14357620,  32, 8,  true),
            Self::Igloo =>           feature(14357618,  32, 8,  true),
            // Ocean Monument — triangular
            Self::OceanMonument =>   feature(10387313,  32, 5,  false),
            // Woodland Mansion — triangular
            Self::WoodlandMansion => feature(10387319,  80, 20, false),
            // Pillager Outpost — different configs per edition!
            Self::PillagerOutpost => if is_bedrock {
                feature(165745296, 80, 24, false)
            } else {
                feature(165745296, 32, 8,  true)
            },
            // Ocean Ruin — Java has {12, 7, triangular}, Bedrock has {20, 8, linear}
            Self::OceanRuin => if is_bedrock {
                feature(14357621, 20, 8, true)
            } else {
                feature(14357621, 12, 7, false)
            },
            // Shipwreck
            Self::Shipwreck =>       feature(165745295, 24, 4,  true),
            // Ruined Portal — different salt per edition!
            Self::RuinedPortal => if is_bedrock {
                feature(40552231, 40, 15, true)
            } else {
                feature(34222645, 40, 15, true)
            },
            // Ancient City — triangular on Bedrock, linear on Java
            Self::AncientCity =>     feature(20083232,  24, 8,  !is_bedrock),
            // Trail Ruin — linear on Java (force Java RNG)
            Self::TrailRuin => feature_force_java(83469867, 34, 8, true),
            // Trial Chamber — always forces Java RNG regardless of edition
            Self::TrialChamber => feature_force_java(94251327, 34, 12, true),
            // Nether structures
            Self::NetherFortress =>  feature(30084232,  27, 4,  true),
            Self::Bastion =>         feature(30084232,  27, 4,  true),
            // End City — triangular, same for both editions
            Self::EndCity => StructureConfig {
                salt: 10387313, region_size: 20, chunk_range: 9,
                placement: LargeEndCity, rarity: 0.0,
                linear_separation: false, force_rng: None,
            },
            // End City Ship — same placement as EndCity
            Self::EndCityShip => StructureConfig {
                salt: 10387313, region_size: 20, chunk_range: 9,
                placement: LargeEndCity, rarity: 0.0,
                linear_separation: false, force_rng: None,
            },
            // End Gateway — decorator with rarity (1.18+ Xoroshiro)
            Self::EndGateway => StructureConfig {
                salt: 40000, region_size: 1, chunk_range: 1,
                placement: DecoratorRarity, rarity: 1.0 / 700.0,
                linear_separation: false, force_rng: None,
            },
            // Unimplemented (special): strongholds, mineshafts, dungeons, etc.
            Self::Stronghold | Self::BuriedTreasure | Self::Mineshaft
            | Self::Dungeon | Self::DesertWell | Self::Fossil | Self::Spawn => none,
        }
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

/// RNG abstraction — wraps Java LCG or Bedrock MT19937.
/// Matches chunkbase's function Wi which switches on edition.
enum StructureRng {
    Java(JavaRandom),
    Bedrock(BedrockRandom),
}

impl StructureRng {
    fn next_int(&mut self, bound: i32) -> i32 {
        match self {
            Self::Java(r) => r.next_int(bound),
            Self::Bedrock(r) => r.next_int(bound),
        }
    }
}

/// Create the structure RNG for a region, using the right edition.
/// Matches chunkbase's `Ti` function.
fn make_structure_rng(
    world_seed: i64,
    region_x: i32,
    region_z: i32,
    salt: i32,
    edition: Edition,
) -> StructureRng {
    let s = (world_seed as u64)
        .wrapping_add((region_x as u64).wrapping_mul(341873128712))
        .wrapping_add((region_z as u64).wrapping_mul(132897987541))
        .wrapping_add(salt as u64);

    match edition {
        Edition::Java => StructureRng::Java(JavaRandom::new(s as i64)),
        Edition::Bedrock => {
            // Bedrock uses only the lower 32 bits as seed (toInt())
            StructureRng::Bedrock(BedrockRandom::new(s as i32))
        }
    }
}

/// Region-based structure placement — matches chunkbase's `Wi` function.
/// Uses either linear or triangular distribution based on config.
fn get_region_pos(
    config: &StructureConfig,
    seed: i64,
    region_x: i32,
    region_z: i32,
    edition: Edition,
) -> (i32, i32) {
    // Force specific RNG if set, otherwise use the edition default
    let effective_edition = config.force_rng.unwrap_or(edition);
    let mut rng = make_structure_rng(seed, region_x, region_z, config.salt, effective_edition);

    let r = config.chunk_range;
    let (cx, cz) = if config.linear_separation {
        // Linear: single nextInt per axis
        (rng.next_int(r), rng.next_int(r))
    } else {
        // Triangular: floor((nextInt + nextInt) / 2)
        let ax = rng.next_int(r);
        let bx = rng.next_int(r);
        let az = rng.next_int(r);
        let bz = rng.next_int(r);
        ((ax + bx) >> 1, (az + bz) >> 1)
    };

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
    edition: Edition,
) -> Option<(i32, i32)> {
    let config = structure.config(edition);
    if config.chunk_range <= 0 { return None; }

    let (bx, bz) = match config.placement {
        PlacementType::Feature => get_region_pos(&config, seed, region_x, region_z, edition),
        PlacementType::LargeEndCity => {
            let (bx, bz) = get_region_pos(&config, seed, region_x, region_z, edition);
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
    edition: Edition,
) -> Vec<(i32, i32)> {
    let config = structure.config(edition);
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
            if let Some((bx, bz)) = get_structure_pos(seed, structure, rx, rz, edition) {
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
