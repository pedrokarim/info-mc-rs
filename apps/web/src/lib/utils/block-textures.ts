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
};

/**
 * Get the texture filename for a block.
 * Returns the filename without .png extension.
 */
export function getTextureFileName(blockName: string): string {
  // Remove minecraft: prefix
  const short = blockName.replace('minecraft:', '');

  // Check overrides first
  if (TEXTURE_OVERRIDES[short]) return TEXTURE_OVERRIDES[short];

  // Default: use block name directly
  return short;
}
