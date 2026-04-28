/// Biome selection for Minecraft 1.18+ overworld.
///
/// This implements the biome selection logic as a decision tree matching
/// Minecraft's MultiNoiseBiomeSource parameter ranges. The parameters are
/// scaled to the same range as the noise output.
///
/// The 5 parameters used for surface biome selection:
///   t = temperature, h = humidity, c = continentalness, e = erosion, w = weirdness
///   depth = 0 for surface biomes
///
/// Reference: cubiomes' btree18 tables and Minecraft's vanilla worldgen data.
use crate::biomes::Biome;

/// Peaks and Valleys value derived from weirdness.
/// Maps weirdness → PV where PV > 0 = peaks, PV < 0 = valleys.
#[inline]
fn pv(w: f64) -> f64 {
    // MC formula: -(|3*|w| - 2| - 1)
    -(((3.0 * w.abs() - 2.0).abs()) - 1.0)
}

/// Temperature index for biome lookups (0..4).
/// MC defines 5 temperature bands.
#[inline]
fn temp_idx(t: f64) -> usize {
    if t < -0.45 {
        0
    }
    // frozen
    else if t < -0.15 {
        1
    }
    // cold
    else if t < 0.2 {
        2
    }
    // temperate
    else if t < 0.55 {
        3
    }
    // warm
    else {
        4
    } // hot
}

/// Humidity index for biome lookups (0..4).
/// MC defines 5 humidity bands.
#[inline]
fn humid_idx(h: f64) -> usize {
    if h < -0.35 {
        0
    }
    // arid
    else if h < -0.1 {
        1
    }
    // dry
    else if h < 0.1 {
        2
    }
    // neutral
    else if h < 0.3 {
        3
    }
    // wet
    else {
        4
    } // humid
}

/// Main biome selection entry point.
pub fn select_biome(t: f64, h: f64, c: f64, e: f64, _depth: f64, w: f64) -> Biome {
    let pv_val = pv(w);

    // ===== MUSHROOM FIELDS =====
    if c < -1.05 {
        return Biome::MushroomFields;
    }

    // ===== DEEP OCEAN =====
    if c < -0.455 {
        return deep_ocean(t);
    }

    // ===== OCEAN =====
    if c < -0.19 {
        return ocean(t);
    }

    // ===== COAST (narrow band) =====
    if c < -0.11 {
        return coast(t, h, e, pv_val, w);
    }

    // ===== NEAR-INLAND =====
    if c < 0.03 {
        return near_inland(t, h, e, pv_val, w);
    }

    // ===== MID-INLAND =====
    if c < 0.3 {
        return mid_inland(t, h, e, pv_val, w);
    }

    // ===== FAR-INLAND =====
    far_inland(t, h, e, pv_val, w)
}

fn deep_ocean(t: f64) -> Biome {
    match temp_idx(t) {
        0 => Biome::DeepFrozenOcean,
        1 => Biome::DeepColdOcean,
        2 => Biome::DeepOcean,
        3 => Biome::DeepLukewarmOcean,
        _ => Biome::DeepOcean, // no deep warm ocean in 1.18+
    }
}

fn ocean(t: f64) -> Biome {
    match temp_idx(t) {
        0 => Biome::FrozenOcean,
        1 => Biome::ColdOcean,
        2 => Biome::Ocean,
        3 => Biome::LukewarmOcean,
        _ => Biome::WarmOcean,
    }
}

fn coast(t: f64, h: f64, e: f64, pv_val: f64, w: f64) -> Biome {
    // Valleys = rivers
    if pv_val < -0.35 {
        return river(t);
    }

    // Low erosion = stony shore or windswept
    if e < -0.0375 {
        if pv_val > 0.3 {
            return pick_peak_biome(t, h, w);
        }
        return Biome::StonyShore;
    }

    // Mid erosion = various
    if e < 0.45 {
        return pick_middle_biome(t, h, w);
    }

    // High erosion = beach or swamp
    if e < 0.55 {
        if pv_val < -0.1 {
            return beach(t);
        }
        return pick_middle_biome(t, h, w);
    }

    // Very high erosion
    if pv_val < -0.1 {
        return beach(t);
    }
    pick_low_biome(t, h, w)
}

fn near_inland(t: f64, h: f64, e: f64, pv_val: f64, w: f64) -> Biome {
    if pv_val < -0.35 {
        if e < 0.55 {
            return river(t);
        }
        return valley_biome(t, h);
    }

    if e < -0.0375 {
        if pv_val > 0.3 {
            return pick_peak_biome(t, h, w);
        }
        return pick_slope_biome(t, h);
    }
    if e < 0.45 {
        if pv_val > 0.3 {
            return pick_slope_biome(t, h);
        }
        return pick_middle_biome(t, h, w);
    }
    if e < 0.55 {
        return pick_middle_biome(t, h, w);
    }

    pick_low_biome(t, h, w)
}

fn mid_inland(t: f64, h: f64, e: f64, pv_val: f64, w: f64) -> Biome {
    if pv_val < -0.35 {
        if e < 0.55 {
            return river(t);
        }
        return valley_biome(t, h);
    }

    if e < -0.0375 {
        if pv_val > 0.3 {
            return pick_peak_biome(t, h, w);
        }
        return pick_slope_biome(t, h);
    }
    if e < 0.45 {
        if pv_val > 0.3 {
            return pick_slope_biome(t, h);
        }
        return pick_middle_biome(t, h, w);
    }
    if e < 0.55 {
        return pick_middle_biome(t, h, w);
    }

    pick_low_biome(t, h, w)
}

fn far_inland(t: f64, h: f64, e: f64, pv_val: f64, w: f64) -> Biome {
    if pv_val < -0.35 {
        if e < 0.55 {
            return river(t);
        }
        return valley_biome(t, h);
    }

    if e < -0.0375 {
        if pv_val > 0.3 {
            return pick_peak_biome(t, h, w);
        }
        return pick_slope_biome(t, h);
    }
    if e < 0.45 {
        if pv_val > 0.3 {
            return pick_plateau_biome(t, h, w);
        }
        return pick_middle_biome(t, h, w);
    }
    if e < 0.55 {
        if pv_val > 0.3 {
            return pick_badlands_or_middle(t, h, w);
        }
        return pick_middle_biome(t, h, w);
    }

    pick_low_biome(t, h, w)
}

fn river(t: f64) -> Biome {
    if temp_idx(t) == 0 {
        Biome::FrozenRiver
    } else {
        Biome::River
    }
}

fn beach(t: f64) -> Biome {
    match temp_idx(t) {
        0 => Biome::SnowyBeach,
        4 => Biome::Desert, // hot beaches are desert in MC
        _ => Biome::Beach,
    }
}

fn valley_biome(t: f64, h: f64) -> Biome {
    let ti = temp_idx(t);
    if ti == 0 {
        return Biome::FrozenRiver;
    }
    if ti >= 4 {
        if humid_idx(h) >= 3 {
            return Biome::MangroveSwamp;
        }
        return Biome::Desert;
    }
    if humid_idx(h) >= 3 {
        return Biome::Swamp;
    }
    Biome::River
}

// ===== Biome selection tables matching Minecraft's vanilla worldgen =====
// These are 5x5 tables indexed by [temp_idx][humid_idx].

/// Middle biomes — the most common biomes on normal terrain.
const MIDDLE_BIOMES: [[Biome; 5]; 5] = [
    // temp 0 (frozen): arid → humid
    [
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyTaiga,
        Biome::Taiga,
    ],
    // temp 1 (cold)
    [
        Biome::Plains,
        Biome::Plains,
        Biome::Forest,
        Biome::Taiga,
        Biome::OldGrowthSpruceTaiga,
    ],
    // temp 2 (temperate)
    [
        Biome::FlowerForest,
        Biome::Plains,
        Biome::Forest,
        Biome::BirchForest,
        Biome::DarkForest,
    ],
    // temp 3 (warm)
    [
        Biome::Savanna,
        Biome::Savanna,
        Biome::Forest,
        Biome::Jungle,
        Biome::Jungle,
    ],
    // temp 4 (hot)
    [
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
    ],
];

/// Middle biomes variant (for weirdness > 0).
const MIDDLE_BIOMES_VARIANT: [[Option<Biome>; 5]; 5] = [
    [
        Some(Biome::IceSpikes),
        None,
        Some(Biome::SnowyTaiga),
        None,
        None,
    ],
    [None, None, None, None, Some(Biome::OldGrowthPineTaiga)],
    [
        Some(Biome::SunflowerPlains),
        None,
        None,
        Some(Biome::OldGrowthBirchForest),
        None,
    ],
    [
        None,
        None,
        Some(Biome::Plains),
        Some(Biome::SparseJungle),
        Some(Biome::BambooJungle),
    ],
    [None, None, None, None, None],
];

/// Low biomes — flat terrain with high erosion.
const LOW_BIOMES: [[Biome; 5]; 5] = [
    [
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyTaiga,
        Biome::Taiga,
    ],
    [
        Biome::Plains,
        Biome::Plains,
        Biome::Forest,
        Biome::Taiga,
        Biome::OldGrowthSpruceTaiga,
    ],
    [
        Biome::FlowerForest,
        Biome::Plains,
        Biome::Forest,
        Biome::BirchForest,
        Biome::DarkForest,
    ],
    [
        Biome::Savanna,
        Biome::Savanna,
        Biome::Forest,
        Biome::Jungle,
        Biome::Jungle,
    ],
    [
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
    ],
];

/// Plateau biomes — high continentalness.
const PLATEAU_BIOMES: [[Biome; 5]; 5] = [
    [
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyTaiga,
        Biome::SnowyTaiga,
    ],
    [
        Biome::Meadow,
        Biome::Meadow,
        Biome::Forest,
        Biome::Taiga,
        Biome::OldGrowthSpruceTaiga,
    ],
    [
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::DarkForest,
    ],
    [
        Biome::SavannaPlat,
        Biome::SavannaPlat,
        Biome::Forest,
        Biome::Forest,
        Biome::Jungle,
    ],
    [
        Biome::Badlands,
        Biome::Badlands,
        Biome::Badlands,
        Biome::WoodedBadlands,
        Biome::WoodedBadlands,
    ],
];

/// Plateau biomes variant (for weirdness > 0).
const PLATEAU_BIOMES_VARIANT: [[Option<Biome>; 5]; 5] = [
    [Some(Biome::IceSpikes), None, None, None, None],
    [
        Some(Biome::CherryGrove),
        None,
        Some(Biome::Meadow),
        Some(Biome::Meadow),
        Some(Biome::OldGrowthPineTaiga),
    ],
    [
        Some(Biome::CherryGrove),
        Some(Biome::CherryGrove),
        Some(Biome::Forest),
        Some(Biome::BirchForest),
        None,
    ],
    [None, None, None, None, None],
    [
        Some(Biome::ErodedBadlands),
        Some(Biome::ErodedBadlands),
        None,
        None,
        None,
    ],
];

/// Slope biomes.
const SLOPE_BIOMES: [[Biome; 5]; 5] = [
    [
        Biome::SnowySlopes,
        Biome::SnowySlopes,
        Biome::SnowySlopes,
        Biome::SnowySlopes,
        Biome::SnowySlopes,
    ],
    [
        Biome::SnowySlopes,
        Biome::SnowySlopes,
        Biome::Grove,
        Biome::Grove,
        Biome::Grove,
    ],
    [
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
    ],
    [
        Biome::Savanna,
        Biome::Savanna,
        Biome::Forest,
        Biome::Forest,
        Biome::Jungle,
    ],
    [
        Biome::Badlands,
        Biome::Badlands,
        Biome::Badlands,
        Biome::WoodedBadlands,
        Biome::WoodedBadlands,
    ],
];

fn pick_middle_biome(t: f64, h: f64, w: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);

    if w > 0.0 {
        if let Some(variant) = MIDDLE_BIOMES_VARIANT[ti][hi] {
            return variant;
        }
    }

    MIDDLE_BIOMES[ti][hi]
}

fn pick_low_biome(t: f64, h: f64, w: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);

    if w > 0.0 {
        if let Some(variant) = MIDDLE_BIOMES_VARIANT[ti][hi] {
            return variant;
        }
    }

    LOW_BIOMES[ti][hi]
}

fn pick_plateau_biome(t: f64, h: f64, w: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);

    if w > 0.0 {
        if let Some(variant) = PLATEAU_BIOMES_VARIANT[ti][hi] {
            return variant;
        }
    }

    PLATEAU_BIOMES[ti][hi]
}

fn pick_slope_biome(t: f64, h: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);
    SLOPE_BIOMES[ti][hi]
}

fn pick_peak_biome(t: f64, h: f64, w: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);

    if ti >= 4 {
        // Hot peaks
        if hi < 2 {
            return Biome::ErodedBadlands;
        }
        return Biome::Badlands;
    }

    if ti >= 3 {
        // Warm peaks
        return Biome::StonyPeaks;
    }

    if ti <= 0 {
        // Frozen peaks
        return Biome::FrozenPeaks;
    }

    // Cold or temperate peaks
    if w > 0.0 {
        return Biome::JaggedPeaks;
    }
    Biome::FrozenPeaks
}

fn pick_badlands_or_middle(t: f64, h: f64, w: f64) -> Biome {
    let ti = temp_idx(t);
    let hi = humid_idx(h);

    if ti >= 4 {
        return PLATEAU_BIOMES[ti][hi];
    }

    pick_middle_biome(t, h, w)
}
