/// End City piece generation — exact port of cubiomes finders.c.
/// Used to determine if an End City contains an End Ship.

use crate::java_random::JavaRandom;

#[derive(Debug, Clone, Copy)]
pub struct Pos3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub typ: i32,
    pub rot: i32,
    pub depth: i32,
    pub pos: Pos3,
    pub bb0: Pos3,
    pub bb1: Pos3,
}

// Piece type constants (matching cubiomes order)
const BASE_FLOOR: i32 = 0;
const BASE_ROOF: i32 = 1;
const BRIDGE_END: i32 = 2;
const BRIDGE_GENTLE_STAIRS: i32 = 3;
const BRIDGE_PIECE: i32 = 4;
const BRIDGE_STEEP_STAIRS: i32 = 5;
const FAT_TOWER_BASE: i32 = 6;
const FAT_TOWER_MIDDLE: i32 = 7;
const FAT_TOWER_TOP: i32 = 8;
const SECOND_FLOOR_1: i32 = 9;
const SECOND_FLOOR_2: i32 = 10;
const SECOND_ROOF: i32 = 11;
const END_SHIP: i32 = 12;
const THIRD_FLOOR_1: i32 = 13;
const THIRD_FLOOR_2: i32 = 14;
const THIRD_ROOF: i32 = 15;
const TOWER_BASE: i32 = 16;
const TOWER_FLOOR: i32 = 17;
const TOWER_PIECE: i32 = 18;
const TOWER_TOP: i32 = 19;

// Piece sizes (sx, sy, sz) — matches cubiomes info table
const PIECE_INFO: [(i32, i32, i32); 20] = [
    (9, 3, 9),    // base_floor
    (11, 1, 11),  // base_roof
    (4, 5, 1),    // bridge_end
    (4, 6, 7),    // bridge_gentle_stairs
    (4, 5, 3),    // bridge_piece
    (4, 6, 3),    // bridge_steep_stairs
    (12, 3, 12),  // fat_tower_base
    (12, 7, 12),  // fat_tower_middle
    (16, 5, 16),  // fat_tower_top
    (11, 7, 11),  // second_floor_1
    (11, 7, 11),  // second_floor_2
    (13, 1, 13),  // second_roof
    (12, 23, 28), // ship
    (13, 7, 13),  // third_floor_1
    (13, 7, 13),  // third_floor_2
    (15, 1, 15),  // third_roof
    (6, 6, 6),    // tower_base
    (6, 3, 6),    // tower_floor
    (6, 3, 6),    // tower_piece
    (8, 4, 8),    // tower_top
];

const MAX_PIECES: usize = 512;

struct PieceEnv {
    list: Vec<Piece>,
    ship: bool,
    y: i32,
}

/// chunkGenerateRnd — deterministic seed for chunk generation.
/// Returns a seed value to pass to JavaRandom::new().
fn chunk_generate_rnd(world_seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
    let mut rng = JavaRandom::new(world_seed);
    let a = rng.next_long();
    let b = rng.next_long();
    a.wrapping_mul(chunk_x as i64) ^ b.wrapping_mul(chunk_z as i64) ^ world_seed
}

/// Add an end city piece. Returns the index of the new piece.
fn add_piece(env: &mut PieceEnv, prev_idx: Option<usize>, rot: i32, px: i32, py: i32, pz: i32, typ: i32) -> usize {
    let info = PIECE_INFO[typ as usize];

    let mut pos = if let Some(pi) = prev_idx {
        env.list[pi].pos
    } else {
        Pos3 { x: px, y: py, z: pz }
    };

    let mut bb0 = pos;
    let mut bb1 = pos;
    bb1.y += info.1;

    match rot {
        0 => { bb1.x += info.0; bb1.z += info.2; }
        1 => { bb0.x -= info.2; bb1.z += info.0; }
        2 => { bb0.x -= info.0; bb0.z -= info.2; }
        3 => { bb1.x += info.2; bb0.z -= info.0; }
        _ => unreachable!(),
    }

    if let Some(pi) = prev_idx {
        let prev = env.list[pi];
        let (mut dx, dy, mut dz) = (0i32, py, 0i32);
        match prev.rot {
            0 => { dx += px; dz += pz; }
            1 => { dx -= pz; dz += px; }
            2 => { dx -= px; dz -= pz; }
            3 => { dx += pz; dz -= px; }
            _ => unreachable!(),
        }
        pos.x += dx; pos.y += dy; pos.z += dz;
        bb0.x += dx; bb0.y += dy; bb0.z += dz;
        bb1.x += dx; bb1.y += dy; bb1.z += dz;
    }

    let piece = Piece { typ, rot, depth: 0, pos, bb0, bb1 };
    env.list.push(piece);
    env.list.len() - 1
}

/// Check if two pieces' bounding boxes intersect.
fn boxes_intersect(p: &Piece, q: &Piece) -> bool {
    q.bb1.x >= p.bb0.x && q.bb0.x <= p.bb1.x &&
    q.bb1.z >= p.bb0.z && q.bb0.z <= p.bb1.z &&
    q.bb1.y >= p.bb0.y && q.bb0.y <= p.bb1.y
}

type GenFn = fn(&mut PieceEnv, usize, i32, &mut JavaRandom) -> bool;

/// Recursively generate pieces with depth limit and collision check.
fn gen_pieces_recursively(
    gen_fn: GenFn,
    env: &mut PieceEnv,
    current_idx: usize,
    depth: i32,
    rng: &mut JavaRandom,
) -> bool {
    if depth > 8 || env.list.len() >= MAX_PIECES {
        return false;
    }

    let start_n = env.list.len();
    if !gen_fn(env, current_idx, depth, rng) {
        // Rollback added pieces
        env.list.truncate(start_n);
        return false;
    }

    let new_n = env.list.len();
    let gen_depth = rng.next(32);
    let current_depth = env.list[current_idx].depth;

    // Check collisions for new pieces against all old pieces
    for i in start_n..new_n {
        // Set depth for new piece
        env.list[i].depth = gen_depth;
        let p = env.list[i];
        for j in 0..start_n {
            let q = env.list[j];
            if boxes_intersect(&p, &q) {
                if current_depth != q.depth {
                    env.list.truncate(start_n);
                    return false;
                }
                break;
            }
        }
    }

    true
}

/// genTower — generates a tower with possible bridges or fat tower extension.
fn gen_tower(env: &mut PieceEnv, current_idx: usize, depth: i32, rng: &mut JavaRandom) -> bool {
    let rot = env.list[current_idx].rot;
    let x = 3 + rng.next_int(2);
    let z = 3 + rng.next_int(2);

    let mut base_idx = current_idx;
    base_idx = add_piece(env, Some(base_idx), rot, x, -3, z, TOWER_BASE);
    base_idx = add_piece(env, Some(base_idx), rot, 0, 7, 0, TOWER_PIECE);

    let floor_idx = if rng.next_int(3) == 0 { Some(base_idx) } else { None };
    let mut current_floor_idx = floor_idx;

    let floorcnt = 1 + rng.next_int(3);
    for i in 0..floorcnt {
        base_idx = add_piece(env, Some(base_idx), rot, 0, 4, 0, TOWER_PIECE);
        if i < floorcnt - 1 && rng.next(1) != 0 {
            current_floor_idx = Some(base_idx);
        }
    }

    if let Some(_floor) = current_floor_idx {
        const BINFO: [[i32; 4]; 4] = [
            [0, 1, -1, 0],
            [1, 6, -1, 1],
            [3, 0, -1, 5],
            [2, 5, -1, 6],
        ];
        for i in 0..4 {
            if rng.next(1) == 0 {
                continue;
            }
            let brot = (rot + BINFO[i][0]) & 3;
            let bridge_idx = add_piece(env, Some(base_idx), brot, BINFO[i][1], BINFO[i][2], BINFO[i][3], BRIDGE_END);
            gen_pieces_recursively(gen_bridge, env, bridge_idx, depth + 1, rng);
        }
    } else if depth != 7 {
        return gen_pieces_recursively(gen_fat_tower, env, base_idx, depth + 1, rng);
    }

    add_piece(env, Some(base_idx), rot, -1, 4, -1, TOWER_TOP);
    true
}

/// genBridge — generates a bridge that may end in an end ship.
fn gen_bridge(env: &mut PieceEnv, current_idx: usize, depth: i32, rng: &mut JavaRandom) -> bool {
    let rot = env.list[current_idx].rot;
    let floorcnt = 1 + rng.next_int(4);

    let mut base_idx = current_idx;
    base_idx = add_piece(env, Some(base_idx), rot, 0, 0, -4, BRIDGE_PIECE);
    env.list[base_idx].depth = -1;

    let mut y = 0;
    for _ in 0..floorcnt {
        if rng.next(1) != 0 {
            base_idx = add_piece(env, Some(base_idx), rot, 0, y, -4, BRIDGE_PIECE);
            y = 0;
            continue;
        }
        if rng.next(1) != 0 {
            base_idx = add_piece(env, Some(base_idx), rot, 0, y, -4, BRIDGE_STEEP_STAIRS);
        } else {
            base_idx = add_piece(env, Some(base_idx), rot, 0, y, -8, BRIDGE_GENTLE_STAIRS);
        }
        y = 4;
    }

    if !env.ship && rng.next_int(10 - depth) == 0 {
        let ship_x = -8 + rng.next_int(8);
        let ship_z = -70 + rng.next_int(10);
        base_idx = add_piece(env, Some(base_idx), rot, ship_x, y, ship_z, END_SHIP);
        env.ship = true;
    } else {
        env.y = y + 1;
        if !gen_pieces_recursively(gen_house_tower, env, base_idx, depth + 1, rng) {
            return false;
        }
    }

    let end_idx = add_piece(env, Some(base_idx), (rot + 2) & 3, 4, y, 0, BRIDGE_END);
    env.list[end_idx].depth = -1;
    true
}

/// genHouseTower — generates a house tower at the end of a bridge.
fn gen_house_tower(env: &mut PieceEnv, current_idx: usize, depth: i32, rng: &mut JavaRandom) -> bool {
    if depth > 8 { return false; }
    let rot = env.list[current_idx].rot;

    let mut base_idx = current_idx;
    base_idx = add_piece(env, Some(base_idx), rot, -3, env.y, -11, BASE_FLOOR);

    let size = rng.next_int(3);
    if size == 0 {
        add_piece(env, Some(base_idx), rot, -1, 4, -1, BASE_ROOF);
        return true;
    }

    base_idx = add_piece(env, Some(base_idx), rot, -1, 0, -1, SECOND_FLOOR_2);
    if size == 1 {
        // SECOND_ROOF — assigns to base (NOT a return)
        base_idx = add_piece(env, Some(base_idx), rot, -1, 8, -1, SECOND_ROOF);
    } else {
        // size == 2: THIRD_FLOOR_2 at Y=4, THIRD_ROOF at Y=8 (cubiomes)
        base_idx = add_piece(env, Some(base_idx), rot, -1, 4, -1, THIRD_FLOOR_2);
        base_idx = add_piece(env, Some(base_idx), rot, -1, 8, -1, THIRD_ROOF);
    }
    gen_pieces_recursively(gen_tower, env, base_idx, depth + 1, rng);
    true
}

/// genFatTower — generates a wider tower variant.
fn gen_fat_tower(env: &mut PieceEnv, current_idx: usize, depth: i32, rng: &mut JavaRandom) -> bool {
    let rot = env.list[current_idx].rot;
    let mut base_idx = current_idx;

    base_idx = add_piece(env, Some(base_idx), rot, -3, 4, -3, FAT_TOWER_BASE);
    base_idx = add_piece(env, Some(base_idx), rot, 0, 4, 0, FAT_TOWER_MIDDLE);

    const BINFO: [[i32; 4]; 4] = [
        [0, 4, -1, 0],
        [1, 12, -1, 4],
        [3, 0, -1, 8],
        [2, 8, -1, 12],
    ];

    for _ in 0..2 {
        if rng.next_int(3) == 0 { break; }
        base_idx = add_piece(env, Some(base_idx), rot, 0, 8, 0, FAT_TOWER_MIDDLE);
        for i in 0..4 {
            if rng.next(1) == 0 { continue; }
            let brot = (rot + BINFO[i][0]) & 3;
            let bridge_idx = add_piece(env, Some(base_idx), brot, BINFO[i][1], BINFO[i][2], BINFO[i][3], BRIDGE_END);
            gen_pieces_recursively(gen_bridge, env, bridge_idx, depth + 1, rng);
        }
    }

    add_piece(env, Some(base_idx), rot, -2, 8, -2, FAT_TOWER_TOP);
    true
}

/// Determine if an End City at the given chunk coordinates contains an End Ship.
pub fn has_end_ship(world_seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
    let chunk_seed = chunk_generate_rnd(world_seed, chunk_x, chunk_z);
    let mut rng = JavaRandom::new(chunk_seed);

    // The first nextInt(4) determines rotation, but we need it for the LCG to advance correctly
    let rot = rng.next_int(4);

    let mut env = PieceEnv {
        list: Vec::with_capacity(MAX_PIECES),
        ship: false,
        y: 0,
    };

    // Initial pieces
    let x = chunk_x * 16 + 8;
    let z = chunk_z * 16 + 8;
    let mut base_idx = add_piece(&mut env, None, rot, x, 0, z, BASE_FLOOR);
    base_idx = add_piece(&mut env, Some(base_idx), rot, -1, 0, -1, SECOND_FLOOR_1);
    base_idx = add_piece(&mut env, Some(base_idx), rot, -1, 4, -1, THIRD_FLOOR_1);
    base_idx = add_piece(&mut env, Some(base_idx), rot, -1, 8, -1, THIRD_ROOF);

    gen_pieces_recursively(gen_tower, &mut env, base_idx, 1, &mut rng);

    env.ship
}
