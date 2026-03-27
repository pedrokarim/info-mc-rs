/**
 * Mapping block names to their texture file names.
 * Minecraft block IDs don't always match texture filenames.
 * Only blocks that differ from the simple convention need to be listed here.
 */
const TEXTURE_OVERRIDES: Record<string, string> = {
  // Blocks where texture name differs from block name
  'grass_block': 'grass_block_top',
  'snow_block': 'snow',
  'podzol': 'podzol_top',
  'mycelium': 'mycelium_top',
  'farmland': 'farmland',
  'dirt_path': 'dirt_path_top',
  'crafting_table': 'crafting_table_top',
  'furnace': 'furnace_front',
  'blast_furnace': 'blast_furnace_front',
  'smoker': 'smoker_front',
  'chest': 'oak_planks', // chest is entity, use planks as fallback
  'barrel': 'barrel_top',
  'tnt': 'tnt_side',
  'hay_block': 'hay_block_side',
  'bone_block': 'bone_block_side',
  'oak_log': 'oak_log',
  'spruce_log': 'spruce_log',
  'birch_log': 'birch_log',
  'jungle_log': 'jungle_log',
  'acacia_log': 'acacia_log',
  'dark_oak_log': 'dark_oak_log',
  'cherry_log': 'cherry_log',
  'mangrove_log': 'mangrove_log',
  'crimson_stem': 'crimson_stem',
  'warped_stem': 'warped_stem',
  'oak_wood': 'oak_log',
  'spruce_wood': 'spruce_log',
  'birch_wood': 'birch_log',
  'jungle_wood': 'jungle_log',
  'acacia_wood': 'acacia_log',
  'dark_oak_wood': 'dark_oak_log',
  'stripped_oak_log': 'stripped_oak_log',
  'stripped_spruce_log': 'stripped_spruce_log',
  'stripped_birch_log': 'stripped_birch_log',
  'bookshelf': 'bookshelf',
  'melon': 'melon_side',
  'pumpkin': 'pumpkin_side',
  'jack_o_lantern': 'jack_o_lantern',
  'redstone_lamp': 'redstone_lamp',
  'respawn_anchor': 'respawn_anchor_top_off',
  'jukebox': 'jukebox_top',
  'note_block': 'note_block',
  'mushroom_stem': 'mushroom_stem',
  'brown_mushroom_block': 'brown_mushroom_block',
  'red_mushroom_block': 'red_mushroom_block',
  'bee_nest': 'bee_nest_front',
  'beehive': 'beehive_front',
  'composter': 'composter_side',
  'lectern': 'lectern_front',
  'loom': 'loom_front',
  'stonecutter': 'stonecutter_side',
  'grindstone': 'grindstone_side',
  'campfire': 'campfire_log',
  'soul_campfire': 'soul_campfire_log',
  'torch': 'torch', // no 3D rendering, just show the icon
  'lantern': 'lantern',
  'soul_lantern': 'soul_lantern',
  'redstone_torch': 'redstone_torch',

  // Legacy block names (pre-1.13 "The Flattening")
  'planks': 'oak_planks',
  'fence': 'oak_planks',
  'wooden_slab': 'oak_planks',
  'double_wooden_slab': 'oak_planks',
  'wooden_door': 'oak_door_top',
  'trapdoor': 'oak_trapdoor',
  'wooden_button': 'oak_planks',
  'wooden_pressure_plate': 'oak_planks',
  'log': 'oak_log',
  'log2': 'dark_oak_log',
  'leaves': 'oak_leaves',
  'leaves2': 'dark_oak_leaves',
  'sapling': 'oak_sapling',
  'stonebrick': 'stone_bricks',
  'stained_hardened_clay': 'terracotta',
  'stained_glass': 'glass',
  'stained_glass_pane': 'glass',
  'wool': 'white_wool',
  'stone_slab2': 'red_sandstone',
  'double_stone_slab': 'smooth_stone',
  'double_stone_slab2': 'red_sandstone',
  'nether_brick_fence': 'nether_bricks',
  'lit_pumpkin': 'jack_o_lantern',
  'lit_furnace': 'furnace_front',
  'lit_redstone_lamp': 'redstone_lamp',
  'unlit_redstone_torch': 'redstone_torch',
  'powered_repeater': 'repeater',
  'unpowered_repeater': 'repeater',
  'powered_comparator': 'comparator',
  'unpowered_comparator': 'comparator',
  'standing_sign': 'oak_planks',
  'wall_sign': 'oak_planks',
  'standing_banner': 'oak_planks',
  'wall_banner': 'oak_planks',
  'banner': 'oak_planks',
  'illager_captain_wall_banner': 'oak_planks',

  // More legacy names (pre-1.13)
  'web': 'cobweb',
  'monster_egg': 'stone',
  'carpet': 'white_wool',
  'bed': 'red_wool',
  'sign': 'oak_planks',
  'smooth_stone_slab': 'smooth_stone',
  'magma_block': 'magma',
  'cactus': 'cactus_side',
  'lava': 'lava_still',
  'coral_block': 'brain_coral_block',

  // Non-renderable / abstract blocks
  'jigsaw': 'jigsaw_top',
  'splash_potion': 'glass',
  'weakness': 'glass',
  'empty': 'glass',
  'feature': 'glass',
  'chests': 'oak_planks',
  'wall_torch': 'torch',
  'soul_wall_torch': 'soul_torch',
  'redstone_wall_torch': 'redstone_torch',
  'cauldron': 'cauldron_side',
  'water_cauldron': 'cauldron_side',
  'lava_cauldron': 'cauldron_side',
  'powder_snow_cauldron': 'cauldron_side',

  // Carpets → wool texture
  'white_carpet': 'white_wool',
  'orange_carpet': 'orange_wool',
  'magenta_carpet': 'magenta_wool',
  'light_blue_carpet': 'light_blue_wool',
  'yellow_carpet': 'yellow_wool',
  'lime_carpet': 'lime_wool',
  'pink_carpet': 'pink_wool',
  'gray_carpet': 'gray_wool',
  'light_gray_carpet': 'light_gray_wool',
  'cyan_carpet': 'cyan_wool',
  'purple_carpet': 'purple_wool',
  'blue_carpet': 'blue_wool',
  'brown_carpet': 'brown_wool',
  'green_carpet': 'green_wool',
  'red_carpet': 'red_wool',
  'black_carpet': 'black_wool',

  // Slabs → base block texture
  'stone_slab': 'smooth_stone_slab_side',
  'stone_brick_slab': 'stone_bricks',
  'cobblestone_slab': 'cobblestone',
  'oak_slab': 'oak_planks',
  'spruce_slab': 'spruce_planks',
  'birch_slab': 'birch_planks',
  'jungle_slab': 'jungle_planks',
  'acacia_slab': 'acacia_planks',
  'dark_oak_slab': 'dark_oak_planks',
  'cherry_slab': 'cherry_planks',
  'mangrove_slab': 'mangrove_planks',
  'bamboo_slab': 'bamboo_planks',
  'crimson_slab': 'crimson_planks',
  'warped_slab': 'warped_planks',
  'sandstone_slab': 'sandstone',
  'red_sandstone_slab': 'red_sandstone',
  'brick_slab': 'bricks',
  'nether_brick_slab': 'nether_bricks',
  'quartz_slab': 'quartz_block_side',
  'prismarine_slab': 'prismarine',
  'prismarine_brick_slab': 'prismarine_bricks',
  'dark_prismarine_slab': 'dark_prismarine',
  'polished_andesite_slab': 'polished_andesite',
  'polished_diorite_slab': 'polished_diorite',
  'polished_granite_slab': 'polished_granite',
  'deepslate_brick_slab': 'deepslate_bricks',
  'deepslate_tile_slab': 'deepslate_tiles',
  'mossy_stone_brick_slab': 'mossy_stone_bricks',
  'mossy_cobblestone_slab': 'mossy_cobblestone',

  // Stairs → base block texture
  'stone_stairs': 'stone',
  'stone_brick_stairs': 'stone_bricks',
  'cobblestone_stairs': 'cobblestone',
  'oak_stairs': 'oak_planks',
  'spruce_stairs': 'spruce_planks',
  'birch_stairs': 'birch_planks',
  'jungle_stairs': 'jungle_planks',
  'acacia_stairs': 'acacia_planks',
  'dark_oak_stairs': 'dark_oak_planks',
  'cherry_stairs': 'cherry_planks',
  'sandstone_stairs': 'sandstone',
  'brick_stairs': 'bricks',
  'nether_brick_stairs': 'nether_bricks',
  'quartz_stairs': 'quartz_block_side',
  'prismarine_stairs': 'prismarine',

  // Walls → base block texture
  'stone_brick_wall': 'stone_bricks',
  'cobblestone_wall': 'cobblestone',
  'mossy_cobblestone_wall': 'mossy_cobblestone',
  'mossy_stone_brick_wall': 'mossy_stone_bricks',
  'brick_wall': 'bricks',
  'nether_brick_wall': 'nether_bricks',
  'sandstone_wall': 'sandstone',
  'red_sandstone_wall': 'red_sandstone',
  'deepslate_brick_wall': 'deepslate_bricks',

  // Wires/redstone
  'redstone_wire': 'redstone_dust_dot',

  // Beds → wool
  'white_bed': 'white_wool',
  'red_bed': 'red_wool',

  // Doors
  'oak_door': 'oak_door_top',
  'spruce_door': 'spruce_door_top',
  'birch_door': 'birch_door_top',
  'iron_door': 'iron_door_top',

  // Signs
  'oak_sign': 'oak_planks',
  'oak_wall_sign': 'oak_planks',
  'spruce_sign': 'spruce_planks',
  'spruce_wall_sign': 'spruce_planks',

  // Flower pots
  'potted_poppy': 'flower_pot',
  'potted_dandelion': 'flower_pot',
  'potted_cactus': 'flower_pot',

  // Misc
  'infested_stone_bricks': 'stone_bricks',
  'infested_stone': 'stone',
  'infested_cobblestone': 'cobblestone',
  'infested_deepslate': 'deepslate',

  // Coral wall fans → coral fan texture
  'tube_coral_wall_fan': 'tube_coral_fan',
  'brain_coral_wall_fan': 'brain_coral_fan',
  'bubble_coral_wall_fan': 'bubble_coral_fan',
  'fire_coral_wall_fan': 'fire_coral_fan',
  'horn_coral_wall_fan': 'horn_coral_fan',
  'dead_tube_coral_wall_fan': 'dead_tube_coral_fan',
  'dead_brain_coral_wall_fan': 'dead_brain_coral_fan',
  'dead_bubble_coral_wall_fan': 'dead_bubble_coral_fan',
  'dead_fire_coral_wall_fan': 'dead_fire_coral_fan',
  'dead_horn_coral_wall_fan': 'dead_horn_coral_fan',
};

/**
 * Suffixes that indicate a variant of a base block.
 * For these, we strip the suffix and use the base block's texture.
 */
const VARIANT_SUFFIXES = [
  '_slab', '_stairs', '_wall', '_fence', '_fence_gate',
  '_button', '_pressure_plate', '_sign', '_wall_sign',
  '_hanging_sign', '_wall_hanging_sign',
];

/**
 * Wood types — used to map fence/gate/etc to planks texture.
 */
const WOOD_TYPES = [
  'oak', 'spruce', 'birch', 'jungle', 'acacia', 'dark_oak',
  'cherry', 'mangrove', 'bamboo', 'crimson', 'warped',
];

export interface BlockFaceTextures {
  right: string;
  left: string;
  top: string;
  bottom: string;
  front: string;
  back: string;
}

const FACE_TEXTURE_OVERRIDES: Record<string, Partial<BlockFaceTextures>> = {
  grass_block: {
    right: 'grass_block_side',
    left: 'grass_block_side',
    front: 'grass_block_side',
    back: 'grass_block_side',
    top: 'grass_block_top',
    bottom: 'dirt',
  },
  podzol: {
    right: 'podzol_side',
    left: 'podzol_side',
    front: 'podzol_side',
    back: 'podzol_side',
    top: 'podzol_top',
    bottom: 'dirt',
  },
  mycelium: {
    right: 'mycelium_side',
    left: 'mycelium_side',
    front: 'mycelium_side',
    back: 'mycelium_side',
    top: 'mycelium_top',
    bottom: 'dirt',
  },
  dirt_path: {
    right: 'dirt_path_side',
    left: 'dirt_path_side',
    front: 'dirt_path_side',
    back: 'dirt_path_side',
    top: 'dirt_path_top',
    bottom: 'dirt',
  },
  sandstone: {
    right: 'sandstone',
    left: 'sandstone',
    front: 'sandstone',
    back: 'sandstone',
    top: 'sandstone_top',
    bottom: 'sandstone_bottom',
  },
  red_sandstone: {
    right: 'red_sandstone',
    left: 'red_sandstone',
    front: 'red_sandstone',
    back: 'red_sandstone',
    top: 'red_sandstone_top',
    bottom: 'red_sandstone_bottom',
  },
  crafting_table: {
    right: 'crafting_table_side',
    left: 'crafting_table_side',
    front: 'crafting_table_front',
    back: 'crafting_table_side',
    top: 'crafting_table_top',
    bottom: 'oak_planks',
  },
  furnace: {
    right: 'furnace_side',
    left: 'furnace_side',
    front: 'furnace_front',
    back: 'furnace_side',
    top: 'furnace_top',
    bottom: 'furnace_top',
  },
  blast_furnace: {
    right: 'blast_furnace_side',
    left: 'blast_furnace_side',
    front: 'blast_furnace_front',
    back: 'blast_furnace_side',
    top: 'blast_furnace_top',
    bottom: 'blast_furnace_top',
  },
  smoker: {
    right: 'smoker_side',
    left: 'smoker_side',
    front: 'smoker_front',
    back: 'smoker_side',
    top: 'smoker_top',
    bottom: 'smoker_top',
  },
  bee_nest: {
    right: 'bee_nest_side',
    left: 'bee_nest_side',
    front: 'bee_nest_front',
    back: 'bee_nest_side',
    top: 'bee_nest_top',
    bottom: 'bee_nest_bottom',
  },
  beehive: {
    right: 'beehive_side',
    left: 'beehive_side',
    front: 'beehive_front',
    back: 'beehive_side',
    top: 'beehive_end',
    bottom: 'beehive_end',
  },
  barrel: {
    right: 'barrel_side',
    left: 'barrel_side',
    front: 'barrel_side',
    back: 'barrel_side',
    top: 'barrel_top',
    bottom: 'barrel_bottom',
  },
  hay_block: {
    right: 'hay_block_side',
    left: 'hay_block_side',
    front: 'hay_block_side',
    back: 'hay_block_side',
    top: 'hay_block_top',
    bottom: 'hay_block_top',
  },
  bone_block: {
    right: 'bone_block_side',
    left: 'bone_block_side',
    front: 'bone_block_side',
    back: 'bone_block_side',
    top: 'bone_block_top',
    bottom: 'bone_block_top',
  },
  melon: {
    right: 'melon_side',
    left: 'melon_side',
    front: 'melon_side',
    back: 'melon_side',
    top: 'melon_top',
    bottom: 'melon_top',
  },
  pumpkin: {
    right: 'pumpkin_side',
    left: 'pumpkin_side',
    front: 'pumpkin_side',
    back: 'pumpkin_side',
    top: 'pumpkin_top',
    bottom: 'pumpkin_top',
  },
  jack_o_lantern: {
    right: 'pumpkin_side',
    left: 'pumpkin_side',
    front: 'jack_o_lantern',
    back: 'pumpkin_side',
    top: 'pumpkin_top',
    bottom: 'pumpkin_top',
  },
  bookshelf: {
    right: 'bookshelf',
    left: 'bookshelf',
    front: 'bookshelf',
    back: 'bookshelf',
    top: 'oak_planks',
    bottom: 'oak_planks',
  },
};

const AXIS_END_TEXTURE_BLOCKS = [
  '_log',
  '_stem',
  '_wood',
  '_hyphae',
  'bone_block',
  'hay_block',
  'purpur_pillar',
  'quartz_pillar',
];

function withUniformFaces(side: string): BlockFaceTextures {
  return {
    right: side,
    left: side,
    top: side,
    bottom: side,
    front: side,
    back: side,
  };
}

function isAxisTextureBlock(short: string): boolean {
  return AXIS_END_TEXTURE_BLOCKS.some((suffix) => short.endsWith(suffix) || short === suffix);
}

/**
 * Get the texture filename for a block.
 * Returns the filename without .png extension.
 */
export function getTextureFileName(blockName: string): string {
  const short = blockName.replace('minecraft:', '');

  // Check explicit overrides first
  if (TEXTURE_OVERRIDES[short]) return TEXTURE_OVERRIDES[short];

  // Auto-resolve variant suffixes (slab, stairs, wall, fence, etc.)
  for (const suffix of VARIANT_SUFFIXES) {
    if (short.endsWith(suffix)) {
      const base = short.slice(0, -suffix.length);

      // Wood variants → planks (for any suffix: slab, stairs, fence, sign, etc.)
      for (const wood of WOOD_TYPES) {
        if (base === wood) return `${wood}_planks`;
      }

      // Check if the base block has an explicit override
      if (TEXTURE_OVERRIDES[base]) return TEXTURE_OVERRIDES[base];

      // Return the base block name
      return base;
    }
  }

  // Block entities / non-renderable blocks → use a generic fallback
  const ENTITY_BLOCKS = [
    'chest', 'trapped_chest', 'ender_chest', 'shulker_box',
    'banner', 'wall_banner', 'bed', 'head', 'wall_head', 'skull', 'wall_skull',
    'sign', 'wall_sign', 'hanging_sign', 'wall_hanging_sign',
  ];
  for (const eb of ENTITY_BLOCKS) {
    if (short === eb || short.endsWith(`_${eb}`)) return 'oak_planks';
  }

  return short;
}

export function getBlockFaceTextures(
  blockName: string,
  props?: Record<string, string>
): BlockFaceTextures {
  const short = blockName.replace('minecraft:', '');
  const base = getTextureFileName(blockName);

  const explicit = FACE_TEXTURE_OVERRIDES[short];
  if (explicit) {
    const side = explicit.front ?? explicit.back ?? explicit.right ?? explicit.left ?? base;
    return {
      right: explicit.right ?? side,
      left: explicit.left ?? side,
      top: explicit.top ?? side,
      bottom: explicit.bottom ?? side,
      front: explicit.front ?? side,
      back: explicit.back ?? side,
    };
  }

  if (isAxisTextureBlock(short)) {
    const end = `${base}_top`;
    const axis = props?.axis ?? 'y';

    if (axis === 'x') {
      return {
        right: end,
        left: end,
        top: base,
        bottom: base,
        front: base,
        back: base,
      };
    }

    if (axis === 'z') {
      return {
        right: base,
        left: base,
        top: base,
        bottom: base,
        front: end,
        back: end,
      };
    }

    return {
      right: base,
      left: base,
      top: end,
      bottom: end,
      front: base,
      back: base,
    };
  }

  return withUniformFaces(base);
}
