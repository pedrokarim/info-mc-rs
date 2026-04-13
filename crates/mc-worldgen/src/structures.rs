/// Minecraft structure placement (1.18+).
/// Each structure type has a spacing/separation pair and a salt.
/// Placement is deterministic based on world seed + region coordinates.

use crate::java_random::JavaRandom;

/// Structure type with its placement parameters.
#[derive(Debug, Clone, Copy)]
pub struct StructureConfig {
    pub salt: i32,
    pub spacing: i32,   // region size in chunks
    pub separation: i32, // minimum distance in chunks
}

/// All supported structure types.
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
}

impl StructureType {
    pub fn config(self) -> StructureConfig {
        match self {
            // 1.18+ overworld structure configs (spacing, separation, salt)
            Self::Village =>         StructureConfig { spacing: 34, separation: 8,  salt: 10387312 },
            Self::DesertTemple =>    StructureConfig { spacing: 32, separation: 8,  salt: 14357617 },
            Self::JungleTemple =>    StructureConfig { spacing: 32, separation: 8,  salt: 14357619 },
            Self::SwampHut =>        StructureConfig { spacing: 32, separation: 8,  salt: 14357620 },
            Self::Igloo =>           StructureConfig { spacing: 32, separation: 8,  salt: 14357618 },
            Self::OceanMonument =>   StructureConfig { spacing: 32, separation: 5,  salt: 10387313 },
            Self::WoodlandMansion => StructureConfig { spacing: 80, separation: 20, salt: 10387319 },
            Self::PillagerOutpost => StructureConfig { spacing: 32, separation: 8,  salt: 165745296 },
            Self::Stronghold =>      StructureConfig { spacing: 1,  separation: 0,  salt: 0 }, // special
            Self::OceanRuin =>       StructureConfig { spacing: 20, separation: 8,  salt: 14357621 },
            Self::Shipwreck =>       StructureConfig { spacing: 24, separation: 4,  salt: 165745295 },
            Self::BuriedTreasure =>  StructureConfig { spacing: 1,  separation: 0,  salt: 0 }, // special
            Self::RuinedPortal =>    StructureConfig { spacing: 40, separation: 15, salt: 34222645 },
            Self::AncientCity =>     StructureConfig { spacing: 24, separation: 8,  salt: 20083232 },
            Self::TrailRuin =>       StructureConfig { spacing: 34, separation: 8,  salt: 83469867 },
            Self::TrialChamber =>    StructureConfig { spacing: 34, separation: 12, salt: 94251327 },
            Self::NetherFortress =>  StructureConfig { spacing: 27, separation: 4,  salt: 30084232 },
            Self::Bastion =>         StructureConfig { spacing: 27, separation: 4,  salt: 30084232 },
            Self::Mineshaft =>       StructureConfig { spacing: 1,  separation: 0,  salt: 0 }, // special
            Self::Dungeon =>         StructureConfig { spacing: 1,  separation: 0,  salt: 0 }, // special
            Self::DesertWell =>      StructureConfig { spacing: 44, separation: 14, salt: 17525864 },
            Self::Fossil =>          StructureConfig { spacing: 64, separation: 8,  salt: 14357921 },
            Self::Spawn =>           StructureConfig { spacing: 1,  separation: 0,  salt: 0 }, // special
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
        }
    }

    /// All overworld structure types.
    pub fn overworld_types() -> &'static [StructureType] {
        &[
            Self::Village,
            Self::DesertTemple,
            Self::JungleTemple,
            Self::SwampHut,
            Self::Igloo,
            Self::OceanMonument,
            Self::WoodlandMansion,
            Self::PillagerOutpost,
            Self::OceanRuin,
            Self::Shipwreck,
            Self::RuinedPortal,
            Self::AncientCity,
            Self::TrailRuin,
            Self::TrialChamber,
            Self::DesertWell,
            Self::Fossil,
        ]
    }

    pub fn nether_types() -> &'static [StructureType] {
        &[
            Self::NetherFortress,
            Self::Bastion,
            Self::RuinedPortal,
        ]
    }

    pub fn end_types() -> &'static [StructureType] {
        &[] // End Cities use a different algorithm (not region-based)
    }
}

/// Find the potential structure position in a given region.
/// Returns (chunk_x, chunk_z) of the candidate position, or None.
///
/// This is the standard MC structure placement algorithm:
/// 1. Divide the world into regions of `spacing` chunks
/// 2. In each region, pick a random position with margin `separation`
pub fn get_structure_pos(
    seed: i64,
    structure: StructureType,
    region_x: i32,
    region_z: i32,
) -> (i32, i32) {
    let config = structure.config();
    let spacing = config.spacing;
    let separation = config.separation;
    let salt = config.salt as i64;

    // Seed for this region
    let region_seed = region_x as i64 * 341873128712_i64
        + region_z as i64 * 132897987541_i64
        + seed
        + salt;

    let mut rng = JavaRandom::new(region_seed);

    let range = (spacing - separation) as i32;
    let cx = region_x * spacing + rng.next_int(range);

    // Must re-create RNG with same seed for Z (MC re-seeds per axis)
    let mut rng2 = JavaRandom::new(region_seed);
    rng2.next_int(range); // skip X value
    let cz = region_z * spacing + rng2.next_int(range);

    (cx, cz)
}

/// Find all structures of a given type within a block-coordinate area.
/// Returns a Vec of (block_x, block_z, structure_type_id) tuples.
pub fn find_structures_in_area(
    seed: i64,
    structure: StructureType,
    block_x: i32,
    block_z: i32,
    block_w: i32,
    block_h: i32,
) -> Vec<(i32, i32)> {
    let config = structure.config();
    if config.spacing <= 0 { return vec![]; }

    let chunk_x0 = block_x >> 4;
    let chunk_z0 = block_z >> 4;
    let chunk_x1 = (block_x + block_w) >> 4;
    let chunk_z1 = (block_z + block_h) >> 4;

    let region_x0 = floor_div(chunk_x0, config.spacing);
    let region_z0 = floor_div(chunk_z0, config.spacing);
    let region_x1 = floor_div(chunk_x1, config.spacing);
    let region_z1 = floor_div(chunk_z1, config.spacing);

    let mut results = Vec::new();

    for rx in region_x0..=region_x1 {
        for rz in region_z0..=region_z1 {
            let (cx, cz) = get_structure_pos(seed, structure, rx, rz);
            // Convert to block coords (center of chunk)
            let bx = cx * 16 + 8;
            let bz = cz * 16 + 8;
            if bx >= block_x && bx < block_x + block_w
                && bz >= block_z && bz < block_z + block_h
            {
                results.push((bx, bz));
            }
        }
    }

    results
}

fn floor_div(a: i32, b: i32) -> i32 {
    let d = a / b;
    if (a ^ b) < 0 && d * b != a { d - 1 } else { d }
}
