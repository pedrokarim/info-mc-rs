use serde::Serialize;

/// All Minecraft biome IDs (1.18+ overworld surface biomes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[repr(u8)]
pub enum Biome {
    Ocean = 0,
    DeepOcean = 1,
    FrozenOcean = 2,
    DeepFrozenOcean = 3,
    ColdOcean = 4,
    DeepColdOcean = 5,
    LukewarmOcean = 6,
    DeepLukewarmOcean = 7,
    WarmOcean = 8,
    Plains = 9,
    SunflowerPlains = 10,
    SnowyPlains = 11,
    IceSpikes = 12,
    Desert = 13,
    Swamp = 14,
    MangroveSwamp = 15,
    Forest = 16,
    FlowerForest = 17,
    BirchForest = 18,
    OldGrowthBirchForest = 19,
    DarkForest = 20,
    Taiga = 21,
    OldGrowthPineTaiga = 22,
    OldGrowthSpruceTaiga = 23,
    SnowyTaiga = 24,
    Savanna = 25,
    SavannaPlat = 26,
    WindsweptSavanna = 27,
    Jungle = 28,
    SparseJungle = 29,
    BambooJungle = 30,
    Badlands = 31,
    ErodedBadlands = 32,
    WoodedBadlands = 33,
    Meadow = 34,
    CherryGrove = 35,
    Grove = 36,
    SnowySlopes = 37,
    FrozenPeaks = 38,
    JaggedPeaks = 39,
    StonyPeaks = 40,
    WindsweptHills = 41,
    WindsweptGravellyHills = 42,
    WindsweptForest = 43,
    River = 44,
    FrozenRiver = 45,
    Beach = 46,
    SnowyBeach = 47,
    StonyShore = 48,
    MushroomFields = 49,

    // Legacy biomes (pre-1.18 only)
    LegacyJungleEdge = 50,
    LegacyDeepWarmOcean = 51,
    LegacyMountains = 52,
    LegacyGiantTreeTaiga = 53,

    Unknown = 255,
}

impl Biome {
    pub fn count() -> usize {
        54
    }

    pub fn from_id(id: u8) -> Self {
        if id < 54 {
            // Safety: all values 0..53 are valid enum variants
            unsafe { std::mem::transmute(id) }
        } else {
            Biome::Unknown
        }
    }

    pub fn id(self) -> u8 {
        self as u8
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Ocean => "Ocean",
            Self::DeepOcean => "Deep Ocean",
            Self::FrozenOcean => "Frozen Ocean",
            Self::DeepFrozenOcean => "Deep Frozen Ocean",
            Self::ColdOcean => "Cold Ocean",
            Self::DeepColdOcean => "Deep Cold Ocean",
            Self::LukewarmOcean => "Lukewarm Ocean",
            Self::DeepLukewarmOcean => "Deep Lukewarm Ocean",
            Self::WarmOcean => "Warm Ocean",
            Self::Plains => "Plains",
            Self::SunflowerPlains => "Sunflower Plains",
            Self::SnowyPlains => "Snowy Plains",
            Self::IceSpikes => "Ice Spikes",
            Self::Desert => "Desert",
            Self::Swamp => "Swamp",
            Self::MangroveSwamp => "Mangrove Swamp",
            Self::Forest => "Forest",
            Self::FlowerForest => "Flower Forest",
            Self::BirchForest => "Birch Forest",
            Self::OldGrowthBirchForest => "Old Growth Birch Forest",
            Self::DarkForest => "Dark Forest",
            Self::Taiga => "Taiga",
            Self::OldGrowthPineTaiga => "Old Growth Pine Taiga",
            Self::OldGrowthSpruceTaiga => "Old Growth Spruce Taiga",
            Self::SnowyTaiga => "Snowy Taiga",
            Self::Savanna => "Savanna",
            Self::SavannaPlat => "Savanna Plateau",
            Self::WindsweptSavanna => "Windswept Savanna",
            Self::Jungle => "Jungle",
            Self::SparseJungle => "Sparse Jungle",
            Self::BambooJungle => "Bamboo Jungle",
            Self::Badlands => "Badlands",
            Self::ErodedBadlands => "Eroded Badlands",
            Self::WoodedBadlands => "Wooded Badlands",
            Self::Meadow => "Meadow",
            Self::CherryGrove => "Cherry Grove",
            Self::Grove => "Grove",
            Self::SnowySlopes => "Snowy Slopes",
            Self::FrozenPeaks => "Frozen Peaks",
            Self::JaggedPeaks => "Jagged Peaks",
            Self::StonyPeaks => "Stony Peaks",
            Self::WindsweptHills => "Windswept Hills",
            Self::WindsweptGravellyHills => "Windswept Gravelly Hills",
            Self::WindsweptForest => "Windswept Forest",
            Self::River => "River",
            Self::FrozenRiver => "Frozen River",
            Self::Beach => "Beach",
            Self::SnowyBeach => "Snowy Beach",
            Self::StonyShore => "Stony Shore",
            Self::MushroomFields => "Mushroom Fields",
            Self::LegacyJungleEdge => "Jungle Edge",
            Self::LegacyDeepWarmOcean => "Deep Warm Ocean",
            Self::LegacyMountains => "Mountains",
            Self::LegacyGiantTreeTaiga => "Giant Tree Taiga",
            Self::Unknown => "Unknown",
        }
    }

    /// RGB color for map rendering.
    pub fn color(self) -> u32 {
        match self {
            Self::Ocean => 0x000070,
            Self::DeepOcean => 0x000030,
            Self::FrozenOcean => 0x7070D6,
            Self::DeepFrozenOcean => 0x404090,
            Self::ColdOcean => 0x2020D0,
            Self::DeepColdOcean => 0x202080,
            Self::LukewarmOcean => 0x0000AC,
            Self::DeepLukewarmOcean => 0x000050,
            Self::WarmOcean => 0x0000FF,
            Self::Plains => 0x8DB360,
            Self::SunflowerPlains => 0xB5DB88,
            Self::SnowyPlains => 0xFFFFFF,
            Self::IceSpikes => 0xB4DCDC,
            Self::Desert => 0xFA9418,
            Self::Swamp => 0x07F9B2,
            Self::MangroveSwamp => 0x67A54A,
            Self::Forest => 0x056621,
            Self::FlowerForest => 0x2D8E49,
            Self::BirchForest => 0x307444,
            Self::OldGrowthBirchForest => 0x589C6C,
            Self::DarkForest => 0x40511A,
            Self::Taiga => 0x0B6659,
            Self::OldGrowthPineTaiga => 0x596651,
            Self::OldGrowthSpruceTaiga => 0x818E79,
            Self::SnowyTaiga => 0x31554A,
            Self::Savanna => 0xBDB25F,
            Self::SavannaPlat => 0xA79D64,
            Self::WindsweptSavanna => 0xE5DA87,
            Self::Jungle => 0x537B09,
            Self::SparseJungle => 0x628B17,
            Self::BambooJungle => 0x768E14,
            Self::Badlands => 0xD94515,
            Self::ErodedBadlands => 0xFF6D3D,
            Self::WoodedBadlands => 0xB09765,
            Self::Meadow => 0x83BB6D,
            Self::CherryGrove => 0xE0A0B5,
            Self::Grove => 0x537D4C,
            Self::SnowySlopes => 0xD2E7E7,
            Self::FrozenPeaks => 0xC4D8E0,
            Self::JaggedPeaks => 0xC0C0C0,
            Self::StonyPeaks => 0x7E7E7E,
            Self::WindsweptHills => 0x606060,
            Self::WindsweptGravellyHills => 0x787878,
            Self::WindsweptForest => 0x507050,
            Self::River => 0x0000FF,
            Self::FrozenRiver => 0xA0A0FF,
            Self::Beach => 0xFADE55,
            Self::SnowyBeach => 0xFAF0C0,
            Self::StonyShore => 0xA2A284,
            Self::MushroomFields => 0xFF00FF,
            Self::LegacyJungleEdge => 0x628B17,
            Self::LegacyDeepWarmOcean => 0x0000AC,
            Self::LegacyMountains => 0x606060,
            Self::LegacyGiantTreeTaiga => 0x596651,
            Self::Unknown => 0xFF00FF,
        }
    }
}

/// Climate point in the 6D multi-noise parameter space (1.18+).
#[derive(Debug, Clone, Copy)]
pub struct ClimatePoint {
    pub temperature: f64,
    pub humidity: f64,
    pub continentalness: f64,
    pub erosion: f64,
    pub weirdness: f64,
    pub depth: f64,
}

/// A biome definition with its climate target point.
#[derive(Debug, Clone)]
pub struct BiomeEntry {
    pub biome: Biome,
    pub climate: ClimatePoint,
}

impl ClimatePoint {
    /// Squared distance in 6D parameter space.
    pub fn distance_sq(&self, other: &ClimatePoint) -> f64 {
        let dt = self.temperature - other.temperature;
        let dh = self.humidity - other.humidity;
        let dc = self.continentalness - other.continentalness;
        let de = self.erosion - other.erosion;
        let dw = self.weirdness - other.weirdness;
        let dd = self.depth - other.depth;
        dt * dt + dh * dh + dc * dc + de * de + dw * dw + dd * dd
    }
}

/// Find the biome whose climate point is closest to the given parameters.
pub fn find_closest_biome(point: &ClimatePoint, entries: &[BiomeEntry]) -> Biome {
    let mut best = Biome::Plains;
    let mut best_dist = f64::MAX;

    for entry in entries {
        let dist = point.distance_sq(&entry.climate);
        if dist < best_dist {
            best_dist = dist;
            best = entry.biome;
        }
    }

    best
}

/// The overworld biome table for 1.18+.
/// These climate points are extracted from Minecraft's vanilla worldgen data.
///
/// Minecraft uses quantized values in range [-1, 1] for each parameter.
/// The biome selection considers surface biomes at depth=0.
pub fn overworld_biome_entries_118() -> Vec<BiomeEntry> {
    // Simplified but representative climate table for 1.18+ overworld.
    // Each biome has a target (temperature, humidity, continentalness, erosion, weirdness, depth).
    // In practice Minecraft uses ranges (min/max) per parameter, but for a map viewer
    // we use representative center points which produce visually correct maps.
    vec![
        // ===== OCEAN BIOMES =====
        // Oceans are selected by continentalness < -0.19
        be(Biome::DeepFrozenOcean,   -1.0, 0.0, -0.9, 0.5, 0.0, 0.0),
        be(Biome::DeepColdOcean,     -0.5, 0.0, -0.9, 0.5, 0.0, 0.0),
        be(Biome::DeepOcean,          0.0, 0.0, -0.9, 0.5, 0.0, 0.0),
        be(Biome::DeepLukewarmOcean,  0.5, 0.0, -0.9, 0.5, 0.0, 0.0),
        be(Biome::FrozenOcean,       -1.0, 0.0, -0.5, 0.5, 0.0, 0.0),
        be(Biome::ColdOcean,         -0.5, 0.0, -0.5, 0.5, 0.0, 0.0),
        be(Biome::Ocean,              0.0, 0.0, -0.5, 0.5, 0.0, 0.0),
        be(Biome::LukewarmOcean,      0.5, 0.0, -0.5, 0.5, 0.0, 0.0),
        be(Biome::WarmOcean,          1.0, 0.0, -0.5, 0.5, 0.0, 0.0),

        // ===== COASTAL / BEACH =====
        be(Biome::SnowyBeach,        -0.7, 0.0, -0.1, 0.8, 0.0, 0.0),
        be(Biome::Beach,              0.2, 0.0, -0.1, 0.8, 0.0, 0.0),
        be(Biome::StonyShore,         0.0, 0.0, -0.1, 0.2, 0.0, 0.0),

        // ===== RIVER =====
        be(Biome::FrozenRiver,       -0.7, 0.0,  0.0, 0.9, 0.0, 0.0),
        be(Biome::River,              0.2, 0.0,  0.0, 0.9, 0.0, 0.0),

        // ===== FLAT INLAND (high erosion) =====
        be(Biome::SnowyPlains,       -0.7, -0.3, 0.3, 0.8, 0.0, 0.0),
        be(Biome::IceSpikes,         -0.7,  0.5, 0.3, 0.7, 0.0, 0.0),
        be(Biome::Plains,             0.0, -0.3, 0.3, 0.7, 0.0, 0.0),
        be(Biome::SunflowerPlains,    0.0, -0.3, 0.3, 0.7, 0.5, 0.0),
        be(Biome::Desert,             1.0, -0.7, 0.3, 0.7, 0.0, 0.0),

        // ===== FORESTS (medium erosion, medium continent) =====
        be(Biome::SnowyTaiga,        -0.5,  0.3, 0.3, 0.5, 0.0, 0.0),
        be(Biome::Taiga,             -0.2,  0.3, 0.3, 0.5, 0.0, 0.0),
        be(Biome::OldGrowthPineTaiga,-0.2,  0.6, 0.3, 0.4, 0.0, 0.0),
        be(Biome::OldGrowthSpruceTaiga, -0.2, 0.8, 0.3, 0.4, 0.0, 0.0),
        be(Biome::Forest,             0.2,  0.0, 0.3, 0.5, 0.0, 0.0),
        be(Biome::FlowerForest,       0.2,  0.0, 0.3, 0.5, 0.5, 0.0),
        be(Biome::BirchForest,        0.2,  0.3, 0.3, 0.5, 0.0, 0.0),
        be(Biome::OldGrowthBirchForest, 0.2, 0.3, 0.3, 0.5, 0.6, 0.0),
        be(Biome::DarkForest,         0.2,  0.5, 0.3, 0.5, 0.0, 0.0),

        // ===== HOT / TROPICAL =====
        be(Biome::Savanna,            0.7, -0.3, 0.3, 0.6, 0.0, 0.0),
        be(Biome::SavannaPlat,        0.7, -0.3, 0.5, 0.3, 0.0, 0.0),
        be(Biome::WindsweptSavanna,    0.7, -0.3, 0.3, 0.6, 0.8, 0.0),
        be(Biome::Jungle,             0.9,  0.7, 0.3, 0.5, 0.0, 0.0),
        be(Biome::SparseJungle,       0.9,  0.5, 0.3, 0.5, 0.0, 0.0),
        be(Biome::BambooJungle,       0.9,  0.8, 0.3, 0.5, 0.5, 0.0),

        // ===== SWAMP =====
        be(Biome::Swamp,              0.2,  0.6, 0.2, 0.8, 0.0, 0.0),
        be(Biome::MangroveSwamp,      0.8,  0.8, 0.2, 0.8, 0.0, 0.0),

        // ===== BADLANDS =====
        be(Biome::Badlands,           1.0, -0.7, 0.4, 0.3, 0.0, 0.0),
        be(Biome::ErodedBadlands,     1.0, -0.7, 0.4, 0.5, 0.0, 0.0),
        be(Biome::WoodedBadlands,     1.0, -0.5, 0.4, 0.3, 0.0, 0.0),

        // ===== MOUNTAIN / HIGHLAND =====
        be(Biome::Meadow,             0.0,  0.0, 0.5, 0.3, 0.0, 0.0),
        be(Biome::CherryGrove,        0.2,  0.3, 0.5, 0.3, 0.5, 0.0),
        be(Biome::Grove,             -0.3,  0.5, 0.6, 0.2, 0.0, 0.0),
        be(Biome::SnowySlopes,       -0.5,  0.0, 0.6, 0.1, 0.0, 0.0),
        be(Biome::FrozenPeaks,       -0.7,  0.0, 0.8, 0.0, 0.0, 0.0),
        be(Biome::JaggedPeaks,       -0.3,  0.0, 0.8, 0.0, 0.0, 0.0),
        be(Biome::StonyPeaks,         0.5,  0.0, 0.8, 0.0, 0.0, 0.0),

        // ===== WINDSWEPT =====
        be(Biome::WindsweptHills,      0.0, -0.2, 0.4, 0.2, 0.0, 0.0),
        be(Biome::WindsweptGravellyHills, 0.0, -0.5, 0.4, 0.2, 0.5, 0.0),
        be(Biome::WindsweptForest,     0.0,  0.2, 0.4, 0.2, 0.0, 0.0),

        // ===== MUSHROOM =====
        be(Biome::MushroomFields,      0.5,  0.5, -0.8, 0.5, 0.0, 0.0),
    ]
}

fn be(biome: Biome, temp: f64, humid: f64, cont: f64, eros: f64, weird: f64, depth: f64) -> BiomeEntry {
    BiomeEntry {
        biome,
        climate: ClimatePoint {
            temperature: temp,
            humidity: humid,
            continentalness: cont,
            erosion: eros,
            weirdness: weird,
            depth,
        },
    }
}

/// Legacy biome IDs used by the pre-1.18 layer system.
/// These map to our Biome enum but use the old numeric IDs.
pub mod legacy {
    use super::Biome;

    pub const OCEAN: i32 = 0;
    pub const PLAINS: i32 = 1;
    pub const DESERT: i32 = 2;
    pub const MOUNTAINS: i32 = 3;
    pub const FOREST: i32 = 4;
    pub const TAIGA: i32 = 5;
    pub const SWAMP: i32 = 6;
    pub const RIVER: i32 = 7;
    pub const FROZEN_OCEAN: i32 = 10;
    pub const FROZEN_RIVER: i32 = 11;
    pub const SNOWY_TUNDRA: i32 = 12;
    pub const SNOWY_MOUNTAINS: i32 = 13;
    pub const MUSHROOM_FIELDS: i32 = 14;
    pub const BEACH: i32 = 16;
    pub const JUNGLE: i32 = 21;
    pub const JUNGLE_EDGE: i32 = 23;
    pub const DEEP_OCEAN: i32 = 24;
    pub const STONE_SHORE: i32 = 25;
    pub const SNOWY_BEACH: i32 = 26;
    pub const BIRCH_FOREST: i32 = 27;
    pub const DARK_FOREST: i32 = 29;
    pub const SNOWY_TAIGA: i32 = 30;
    pub const GIANT_TREE_TAIGA: i32 = 32;
    pub const SAVANNA: i32 = 35;
    pub const SAVANNA_PLATEAU: i32 = 36;
    pub const BADLANDS: i32 = 37;
    pub const WOODED_BADLANDS: i32 = 38;
    pub const WARM_OCEAN: i32 = 44;
    pub const LUKEWARM_OCEAN: i32 = 45;
    pub const COLD_OCEAN: i32 = 46;
    pub const DEEP_WARM_OCEAN: i32 = 47;
    pub const DEEP_LUKEWARM_OCEAN: i32 = 48;
    pub const DEEP_COLD_OCEAN: i32 = 49;
    pub const DEEP_FROZEN_OCEAN: i32 = 50;

    /// Convert legacy numeric biome ID to our Biome enum.
    pub fn from_legacy_id(id: i32) -> Biome {
        match id {
            OCEAN => Biome::Ocean,
            PLAINS => Biome::Plains,
            DESERT => Biome::Desert,
            MOUNTAINS => Biome::WindsweptHills,
            FOREST => Biome::Forest,
            TAIGA => Biome::Taiga,
            SWAMP => Biome::Swamp,
            RIVER => Biome::River,
            FROZEN_OCEAN => Biome::FrozenOcean,
            FROZEN_RIVER => Biome::FrozenRiver,
            SNOWY_TUNDRA | SNOWY_MOUNTAINS => Biome::SnowyPlains,
            MUSHROOM_FIELDS | 15 => Biome::MushroomFields,
            BEACH => Biome::Beach,
            JUNGLE => Biome::Jungle,
            JUNGLE_EDGE => Biome::SparseJungle,
            DEEP_OCEAN => Biome::DeepOcean,
            STONE_SHORE => Biome::StonyShore,
            SNOWY_BEACH => Biome::SnowyBeach,
            BIRCH_FOREST | 28 => Biome::BirchForest,
            DARK_FOREST => Biome::DarkForest,
            SNOWY_TAIGA | 31 => Biome::SnowyTaiga,
            GIANT_TREE_TAIGA | 33 => Biome::OldGrowthPineTaiga,
            34 => Biome::OldGrowthSpruceTaiga,
            SAVANNA => Biome::Savanna,
            SAVANNA_PLATEAU => Biome::SavannaPlat,
            BADLANDS | 39 => Biome::Badlands,
            WOODED_BADLANDS => Biome::WoodedBadlands,
            WARM_OCEAN => Biome::WarmOcean,
            LUKEWARM_OCEAN => Biome::LukewarmOcean,
            COLD_OCEAN => Biome::ColdOcean,
            DEEP_WARM_OCEAN => Biome::WarmOcean,
            DEEP_LUKEWARM_OCEAN => Biome::DeepLukewarmOcean,
            DEEP_COLD_OCEAN => Biome::DeepColdOcean,
            DEEP_FROZEN_OCEAN => Biome::DeepFrozenOcean,
            // Mutated / variant biomes (128+)
            129 => Biome::SunflowerPlains,
            130 => Biome::Desert,
            131 => Biome::WindsweptHills,
            132 => Biome::FlowerForest,
            133 => Biome::Taiga,
            134 => Biome::Swamp,
            140 => Biome::IceSpikes,
            149 => Biome::Jungle,
            151 => Biome::SparseJungle,
            155 => Biome::OldGrowthBirchForest,
            157 => Biome::DarkForest,
            160 => Biome::SnowyTaiga,
            161 => Biome::OldGrowthPineTaiga,
            162 => Biome::OldGrowthSpruceTaiga,
            163 => Biome::WindsweptSavanna,
            166 => Biome::ErodedBadlands,
            _ => Biome::Plains,
        }
    }

    /// Get the RGB color for a legacy biome ID directly.
    pub fn legacy_color(id: i32) -> u32 {
        from_legacy_id(id).color()
    }
}
