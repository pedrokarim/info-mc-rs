/** Minecraft Command Generator — data & builder utilities */

// ── Command types ──

export type CommandId = 'give' | 'summon' | 'setblock' | 'fill' | 'particle' | 'effect' | 'tp' | 'gamemode' | 'time' | 'weather';

export interface CommandDef {
	id: CommandId;
	label: string;
	syntax: string;
}

export const COMMANDS: CommandDef[] = [
	{ id: 'give',     label: '/give',     syntax: '/give <target> <item> [count]' },
	{ id: 'summon',   label: '/summon',    syntax: '/summon <entity> [x y z] [nbt]' },
	{ id: 'setblock', label: '/setblock',  syntax: '/setblock <x y z> <block> [mode]' },
	{ id: 'fill',     label: '/fill',      syntax: '/fill <x1 y1 z1> <x2 y2 z2> <block> [mode]' },
	{ id: 'particle', label: '/particle',  syntax: '/particle <name> <x y z> <dx dy dz> <speed> <count>' },
	{ id: 'effect',   label: '/effect',    syntax: '/effect give <target> <effect> [duration] [amplifier]' },
	{ id: 'tp',       label: '/tp',        syntax: '/tp <target> <x y z> [yaw pitch]' },
	{ id: 'gamemode', label: '/gamemode',  syntax: '/gamemode <mode> [target]' },
	{ id: 'time',     label: '/time',      syntax: '/time set <value>' },
	{ id: 'weather',  label: '/weather',   syntax: '/weather <type> [duration]' },
];

// ── MC ID lists (curated subset of the most common) ──

export const ITEMS: string[] = [
	// Tools
	'diamond_sword', 'netherite_sword', 'diamond_pickaxe', 'netherite_pickaxe',
	'diamond_axe', 'netherite_axe', 'diamond_shovel', 'netherite_shovel',
	'diamond_hoe', 'netherite_hoe', 'bow', 'crossbow', 'trident', 'mace',
	'fishing_rod', 'flint_and_steel', 'shears', 'shield', 'spyglass', 'brush',
	// Armor
	'diamond_helmet', 'diamond_chestplate', 'diamond_leggings', 'diamond_boots',
	'netherite_helmet', 'netherite_chestplate', 'netherite_leggings', 'netherite_boots',
	'elytra', 'turtle_helmet',
	// Food
	'golden_apple', 'enchanted_golden_apple', 'cooked_beef', 'bread',
	'golden_carrot', 'chorus_fruit', 'suspicious_stew',
	// Blocks
	'diamond_block', 'netherite_block', 'emerald_block', 'gold_block', 'iron_block',
	'tnt', 'command_block', 'barrier', 'bedrock', 'spawner',
	'beacon', 'enchanting_table', 'anvil', 'ender_chest', 'shulker_box',
	// Misc
	'ender_pearl', 'ender_eye', 'experience_bottle', 'firework_rocket',
	'totem_of_undying', 'nether_star', 'dragon_egg', 'elytra',
	'name_tag', 'saddle', 'lead', 'compass', 'clock', 'map',
	'written_book', 'writable_book', 'knowledge_book',
	// Potions
	'potion', 'splash_potion', 'lingering_potion', 'tipped_arrow',
];

export const BLOCKS: string[] = [
	'stone', 'granite', 'diorite', 'andesite', 'deepslate',
	'grass_block', 'dirt', 'cobblestone', 'oak_planks', 'spruce_planks',
	'oak_log', 'spruce_log', 'birch_log', 'jungle_log',
	'sand', 'gravel', 'gold_ore', 'iron_ore', 'diamond_ore',
	'coal_ore', 'redstone_ore', 'lapis_ore', 'emerald_ore',
	'obsidian', 'crying_obsidian', 'glowstone', 'netherrack',
	'soul_sand', 'soul_soil', 'basalt', 'blackstone',
	'end_stone', 'purpur_block', 'prismarine',
	'glass', 'tinted_glass', 'sea_lantern', 'shroomlight',
	'ice', 'packed_ice', 'blue_ice',
	'tnt', 'redstone_block', 'slime_block', 'honey_block',
	'barrier', 'bedrock', 'command_block', 'structure_block',
	'spawner', 'beacon', 'enchanting_table', 'anvil',
	'crafting_table', 'furnace', 'blast_furnace', 'smoker',
	'chest', 'barrel', 'hopper', 'dropper', 'dispenser',
	'piston', 'sticky_piston', 'observer', 'daylight_detector',
	'note_block', 'jukebox', 'bell', 'lodestone',
	'water', 'lava', 'air',
];

export const ENTITIES: string[] = [
	// Hostile
	'zombie', 'skeleton', 'creeper', 'spider', 'enderman',
	'witch', 'phantom', 'drowned', 'husk', 'stray',
	'blaze', 'ghast', 'magma_cube', 'slime', 'silverfish',
	'guardian', 'elder_guardian', 'shulker', 'vex', 'vindicator',
	'evoker', 'pillager', 'ravager', 'wither_skeleton', 'piglin_brute',
	'hoglin', 'zoglin', 'warden', 'breeze',
	// Passive
	'cow', 'pig', 'sheep', 'chicken', 'horse', 'donkey', 'mule',
	'wolf', 'cat', 'ocelot', 'parrot', 'rabbit', 'fox',
	'bee', 'turtle', 'dolphin', 'squid', 'glow_squid',
	'axolotl', 'goat', 'frog', 'allay', 'camel', 'sniffer', 'armadillo',
	'villager', 'iron_golem', 'snow_golem',
	// Bosses
	'ender_dragon', 'wither',
	// Other
	'armor_stand', 'item_frame', 'glow_item_frame', 'painting',
	'minecart', 'boat', 'tnt_minecart', 'chest_minecart',
	'lightning_bolt', 'falling_block', 'experience_orb',
];

export const PARTICLES: string[] = [
	'flame', 'soul_fire_flame', 'smoke', 'large_smoke',
	'cloud', 'explosion', 'explosion_emitter',
	'heart', 'angry_villager', 'happy_villager',
	'portal', 'enchant', 'crit', 'enchanted_hit',
	'totem_of_undying', 'end_rod', 'firework',
	'dripping_water', 'dripping_lava', 'dripping_honey',
	'dust', 'dust_color_transition',
	'note', 'witch', 'campfire_cosy_smoke',
	'cherry_leaves', 'sculk_charge', 'sculk_soul',
	'sonic_boom', 'flash', 'snowflake',
	'gust', 'trial_spawner_detection',
];

export const EFFECTS: string[] = [
	'speed', 'slowness', 'haste', 'mining_fatigue',
	'strength', 'instant_health', 'instant_damage',
	'jump_boost', 'nausea', 'regeneration',
	'resistance', 'fire_resistance', 'water_breathing',
	'invisibility', 'blindness', 'night_vision',
	'hunger', 'weakness', 'poison', 'wither',
	'health_boost', 'absorption', 'saturation',
	'glowing', 'levitation', 'luck', 'unluck',
	'slow_falling', 'conduit_power', 'dolphins_grace',
	'bad_omen', 'hero_of_the_village', 'darkness',
	'trial_omen', 'wind_charged', 'weaving', 'oozing', 'infested',
];

export const GAMEMODES = ['survival', 'creative', 'adventure', 'spectator'];
export const WEATHER_TYPES = ['clear', 'rain', 'thunder'];
export const TIME_PRESETS = [
	{ label: 'Jour', value: 'day' },
	{ label: 'Midi', value: 'noon' },
	{ label: 'Nuit', value: 'night' },
	{ label: 'Minuit', value: 'midnight' },
	{ label: 'Lever', value: '0' },
	{ label: 'Coucher', value: '12000' },
];
export const TARGETS = ['@p', '@a', '@s', '@r', '@e'];
export const SETBLOCK_MODES = ['replace', 'destroy', 'keep'];
export const FILL_MODES = ['replace', 'destroy', 'keep', 'hollow', 'outline'];

// ── Builder state for each command ──

export interface GiveState { target: string; item: string; count: string; }
export interface SummonState { entity: string; x: string; y: string; z: string; }
export interface SetblockState { x: string; y: string; z: string; block: string; mode: string; }
export interface FillState { x1: string; y1: string; z1: string; x2: string; y2: string; z2: string; block: string; mode: string; }
export interface ParticleState { name: string; x: string; y: string; z: string; dx: string; dy: string; dz: string; speed: string; count: string; }
export interface EffectState { target: string; effect: string; duration: string; amplifier: string; }
export interface TpState { target: string; x: string; y: string; z: string; }
export interface GamemodeState { mode: string; target: string; }
export interface TimeState { value: string; }
export interface WeatherState { type: string; duration: string; }

// ── Command builders ──

export function buildGive(s: GiveState): string {
	const parts = [`/give ${s.target} minecraft:${s.item}`];
	const count = parseInt(s.count);
	if (!isNaN(count) && count > 1) parts.push(count.toString());
	return parts.join(' ');
}

export function buildSummon(s: SummonState): string {
	const hasPos = s.x || s.y || s.z;
	if (hasPos) return `/summon minecraft:${s.entity} ${s.x || '~'} ${s.y || '~'} ${s.z || '~'}`;
	return `/summon minecraft:${s.entity}`;
}

export function buildSetblock(s: SetblockState): string {
	return `/setblock ${s.x || '~'} ${s.y || '~'} ${s.z || '~'} minecraft:${s.block} ${s.mode}`;
}

export function buildFill(s: FillState): string {
	return `/fill ${s.x1 || '~'} ${s.y1 || '~'} ${s.z1 || '~'} ${s.x2 || '~'} ${s.y2 || '~'} ${s.z2 || '~'} minecraft:${s.block} ${s.mode}`;
}

export function buildParticle(s: ParticleState): string {
	return `/particle minecraft:${s.name} ${s.x || '~'} ${s.y || '~'} ${s.z || '~'} ${s.dx || '0'} ${s.dy || '0'} ${s.dz || '0'} ${s.speed || '0'} ${s.count || '1'}`;
}

export function buildEffect(s: EffectState): string {
	const parts = [`/effect give ${s.target} minecraft:${s.effect}`];
	if (s.duration) parts.push(s.duration);
	if (s.amplifier) parts.push(s.amplifier);
	return parts.join(' ');
}

export function buildTp(s: TpState): string {
	return `/tp ${s.target} ${s.x || '~'} ${s.y || '~'} ${s.z || '~'}`;
}

export function buildGamemode(s: GamemodeState): string {
	return `/gamemode ${s.mode} ${s.target}`;
}

export function buildTime(s: TimeState): string {
	return `/time set ${s.value}`;
}

export function buildWeather(s: WeatherState): string {
	const parts = [`/weather ${s.type}`];
	if (s.duration) parts.push(s.duration);
	return parts.join(' ');
}

// ── Fuzzy search helper ──

export function fuzzyFilter(items: string[], query: string, limit = 20): string[] {
	if (!query) return items.slice(0, limit);
	const q = query.toLowerCase();
	return items.filter(i => i.includes(q)).slice(0, limit);
}
