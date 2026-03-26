/**
 * Mapping of Minecraft block names to approximate hex colors.
 * Used for 3D structure visualization (no textures, just colored cubes).
 */

export const BLOCK_COLORS: Record<string, string> = {
  // Stone variants
  'minecraft:stone': '#808080',
  'minecraft:granite': '#9a6c50',
  'minecraft:polished_granite': '#9a6e53',
  'minecraft:diorite': '#bcbcbc',
  'minecraft:polished_diorite': '#c0c0c0',
  'minecraft:andesite': '#888888',
  'minecraft:polished_andesite': '#8c8c8c',
  'minecraft:cobblestone': '#7a7a7a',
  'minecraft:mossy_cobblestone': '#6a7a5a',
  'minecraft:stone_bricks': '#7a7a7a',
  'minecraft:mossy_stone_bricks': '#6a7a5a',
  'minecraft:cracked_stone_bricks': '#747474',
  'minecraft:deepslate': '#505050',
  'minecraft:cobbled_deepslate': '#4a4a4a',
  'minecraft:tuff': '#6b6b5e',
  'minecraft:calcite': '#ddddd0',
  'minecraft:dripstone_block': '#8b7a60',
  'minecraft:smooth_stone': '#9a9a9a',
  'minecraft:bedrock': '#3a3a3a',

  // Dirt / Grass
  'minecraft:grass_block': '#7cba4e',
  'minecraft:dirt': '#8b6c42',
  'minecraft:coarse_dirt': '#7a6038',
  'minecraft:rooted_dirt': '#8b6840',
  'minecraft:podzol': '#6b5a30',
  'minecraft:mycelium': '#8b7a8b',
  'minecraft:mud': '#3c3528',
  'minecraft:farmland': '#6e4e2a',
  'minecraft:dirt_path': '#9a8550',
  'minecraft:clay': '#a0a0b4',
  'minecraft:gravel': '#8a8480',

  // Sand
  'minecraft:sand': '#dbd3a0',
  'minecraft:red_sand': '#b86020',
  'minecraft:sandstone': '#d8c878',
  'minecraft:red_sandstone': '#b85e20',
  'minecraft:soul_sand': '#4e3e28',
  'minecraft:soul_soil': '#44362a',

  // Wood
  'minecraft:oak_log': '#6b5230',
  'minecraft:oak_planks': '#bc9456',
  'minecraft:oak_wood': '#6b5830',
  'minecraft:spruce_log': '#3e2810',
  'minecraft:spruce_planks': '#6b5030',
  'minecraft:birch_log': '#d5cca0',
  'minecraft:birch_planks': '#c8b878',
  'minecraft:jungle_log': '#5a4820',
  'minecraft:jungle_planks': '#a07848',
  'minecraft:acacia_log': '#686058',
  'minecraft:acacia_planks': '#b06030',
  'minecraft:dark_oak_log': '#3e2e14',
  'minecraft:dark_oak_planks': '#42320e',
  'minecraft:cherry_log': '#34212c',
  'minecraft:cherry_planks': '#e4b4a8',
  'minecraft:mangrove_log': '#6a4838',
  'minecraft:mangrove_planks': '#7a3838',
  'minecraft:bamboo_planks': '#c8b040',
  'minecraft:crimson_planks': '#6e2e3e',
  'minecraft:warped_planks': '#2e6e5e',
  'minecraft:crimson_stem': '#6e2838',
  'minecraft:warped_stem': '#2e6858',

  // Leaves
  'minecraft:oak_leaves': '#4a8a2a',
  'minecraft:spruce_leaves': '#3a6a2a',
  'minecraft:birch_leaves': '#5a8a3a',
  'minecraft:jungle_leaves': '#3a8a1a',
  'minecraft:acacia_leaves': '#4a8a2a',
  'minecraft:dark_oak_leaves': '#3a6a2a',
  'minecraft:cherry_leaves': '#e8a0b8',
  'minecraft:azalea_leaves': '#5a8a3a',
  'minecraft:mangrove_leaves': '#4a8a2a',

  // Ores
  'minecraft:coal_ore': '#606060',
  'minecraft:iron_ore': '#8a7a6a',
  'minecraft:gold_ore': '#8a8a3a',
  'minecraft:diamond_ore': '#5abab4',
  'minecraft:emerald_ore': '#4aba4a',
  'minecraft:lapis_ore': '#3050a0',
  'minecraft:redstone_ore': '#8a2020',
  'minecraft:copper_ore': '#7a5a3a',

  // Metal blocks
  'minecraft:iron_block': '#d8d8d8',
  'minecraft:gold_block': '#f0d020',
  'minecraft:diamond_block': '#60e8e0',
  'minecraft:emerald_block': '#40d840',
  'minecraft:lapis_block': '#2050a0',
  'minecraft:redstone_block': '#a01010',
  'minecraft:copper_block': '#b86840',
  'minecraft:netherite_block': '#404040',
  'minecraft:amethyst_block': '#8860b0',

  // Liquids
  'minecraft:water': '#3f76e4',
  'minecraft:lava': '#cf5b19',

  // Glass
  'minecraft:glass': '#c0d8e8',
  'minecraft:tinted_glass': '#302830',

  // Terracotta
  'minecraft:terracotta': '#985a40',
  'minecraft:white_terracotta': '#d1b0a0',
  'minecraft:orange_terracotta': '#a05828',
  'minecraft:red_terracotta': '#8e3a2a',
  'minecraft:brown_terracotta': '#4e3828',
  'minecraft:yellow_terracotta': '#ba8a28',
  'minecraft:cyan_terracotta': '#565a5a',

  // Concrete
  'minecraft:white_concrete': '#cfd5d6',
  'minecraft:black_concrete': '#08090b',
  'minecraft:red_concrete': '#8e2020',
  'minecraft:blue_concrete': '#2c2e8e',
  'minecraft:green_concrete': '#495b24',
  'minecraft:yellow_concrete': '#f0b810',
  'minecraft:orange_concrete': '#e06010',
  'minecraft:purple_concrete': '#6e2e8e',
  'minecraft:cyan_concrete': '#157789',
  'minecraft:light_blue_concrete': '#3586b0',
  'minecraft:lime_concrete': '#5ea818',
  'minecraft:pink_concrete': '#d6658a',
  'minecraft:magenta_concrete': '#a93098',
  'minecraft:gray_concrete': '#363b3f',
  'minecraft:light_gray_concrete': '#7d7d73',
  'minecraft:brown_concrete': '#603818',

  // Wool
  'minecraft:white_wool': '#e8e8e8',
  'minecraft:black_wool': '#1a1a1e',
  'minecraft:red_wool': '#a02020',
  'minecraft:blue_wool': '#2e3090',
  'minecraft:green_wool': '#355a18',
  'minecraft:yellow_wool': '#b8a020',

  // Nether
  'minecraft:netherrack': '#6a2828',
  'minecraft:nether_bricks': '#2e1418',
  'minecraft:basalt': '#505050',
  'minecraft:blackstone': '#2a2830',
  'minecraft:glowstone': '#b09850',
  'minecraft:magma_block': '#8a3818',
  'minecraft:obsidian': '#0e0a18',
  'minecraft:crying_obsidian': '#1e0a28',

  // End
  'minecraft:end_stone': '#d8d898',
  'minecraft:end_stone_bricks': '#d0d088',
  'minecraft:purpur_block': '#a878a8',

  // Misc
  'minecraft:bricks': '#966050',
  'minecraft:bookshelf': '#6a5030',
  'minecraft:hay_block': '#b8a030',
  'minecraft:melon': '#6aa028',
  'minecraft:pumpkin': '#c87818',
  'minecraft:jack_o_lantern': '#d89018',
  'minecraft:snow_block': '#f0f0f0',
  'minecraft:ice': '#8cb0f0',
  'minecraft:packed_ice': '#7898d8',
  'minecraft:blue_ice': '#5878c0',
  'minecraft:prismarine': '#5a9a8a',
  'minecraft:dark_prismarine': '#2a5a48',
  'minecraft:sea_lantern': '#a8c8d8',
  'minecraft:sponge': '#c0c040',
  'minecraft:tnt': '#c03020',
  'minecraft:crafting_table': '#6a5030',
  'minecraft:furnace': '#808080',
  'minecraft:chest': '#8a6830',
  'minecraft:barrel': '#7a5828',
  'minecraft:air': '',
  'minecraft:cave_air': '',
  'minecraft:void_air': '',
  'minecraft:structure_void': '',
};

/**
 * Get color for a block name. Falls back to a deterministic hash color.
 */
export function getBlockColor(blockName: string): string {
  if (!blockName) return '#ff00ff';
  const color = BLOCK_COLORS[blockName];
  if (color !== undefined) return color;

  // Deterministic hash → color for unknown blocks
  let hash = 0;
  for (let i = 0; i < blockName.length; i++) {
    hash = ((hash << 5) - hash + blockName.charCodeAt(i)) | 0;
  }
  const r = ((hash >> 16) & 0xff);
  const g = ((hash >> 8) & 0xff);
  const b = (hash & 0xff);
  return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
}

/**
 * Check if a block is air/invisible (should not be rendered).
 */
export function isAirBlock(blockName: string): boolean {
  return !blockName || BLOCK_COLORS[blockName] === '' ||
    blockName.endsWith(':air') || blockName.endsWith(':cave_air') ||
    blockName.endsWith(':void_air') || blockName.endsWith(':structure_void');
}
