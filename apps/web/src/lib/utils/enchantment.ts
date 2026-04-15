/** Minecraft Enchantment Calculator — data & anvil cost logic */

// ── Item categories ──

export interface ItemCategory {
	id: string;
	label: string;
	items: string[];
}

export const ITEM_CATEGORIES: ItemCategory[] = [
	{ id: 'sword', label: 'Épée', items: ['wooden_sword', 'stone_sword', 'iron_sword', 'golden_sword', 'diamond_sword', 'netherite_sword'] },
	{ id: 'axe', label: 'Hache', items: ['wooden_axe', 'stone_axe', 'iron_axe', 'golden_axe', 'diamond_axe', 'netherite_axe'] },
	{ id: 'pickaxe', label: 'Pioche', items: ['wooden_pickaxe', 'stone_pickaxe', 'iron_pickaxe', 'golden_pickaxe', 'diamond_pickaxe', 'netherite_pickaxe'] },
	{ id: 'shovel', label: 'Pelle', items: ['wooden_shovel', 'stone_shovel', 'iron_shovel', 'golden_shovel', 'diamond_shovel', 'netherite_shovel'] },
	{ id: 'hoe', label: 'Houe', items: ['wooden_hoe', 'stone_hoe', 'iron_hoe', 'golden_hoe', 'diamond_hoe', 'netherite_hoe'] },
	{ id: 'helmet', label: 'Casque', items: ['leather_helmet', 'chainmail_helmet', 'iron_helmet', 'golden_helmet', 'diamond_helmet', 'netherite_helmet', 'turtle_helmet'] },
	{ id: 'chestplate', label: 'Plastron', items: ['leather_chestplate', 'chainmail_chestplate', 'iron_chestplate', 'golden_chestplate', 'diamond_chestplate', 'netherite_chestplate'] },
	{ id: 'leggings', label: 'Jambières', items: ['leather_leggings', 'chainmail_leggings', 'iron_leggings', 'golden_leggings', 'diamond_leggings', 'netherite_leggings'] },
	{ id: 'boots', label: 'Bottes', items: ['leather_boots', 'chainmail_boots', 'iron_boots', 'golden_boots', 'diamond_boots', 'netherite_boots'] },
	{ id: 'bow', label: 'Arc', items: ['bow'] },
	{ id: 'crossbow', label: 'Arbalète', items: ['crossbow'] },
	{ id: 'trident', label: 'Trident', items: ['trident'] },
	{ id: 'fishing_rod', label: 'Canne à pêche', items: ['fishing_rod'] },
	{ id: 'mace', label: 'Masse', items: ['mace'] },
];

// ── Enchantment definitions ──

export interface EnchantDef {
	id: string;
	label: string;
	maxLevel: number;
	/** Item categories this enchant applies to */
	items: string[];
	/** Multiplier per level for anvil cost (from book) */
	bookMultiplier: number;
	/** Incompatible enchantment IDs */
	incompatible: string[];
}

export const ENCHANTMENTS: EnchantDef[] = [
	// Sword
	{ id: 'sharpness', label: 'Tranchant', maxLevel: 5, items: ['sword', 'axe'], bookMultiplier: 1, incompatible: ['smite', 'bane_of_arthropods'] },
	{ id: 'smite', label: 'Châtiment', maxLevel: 5, items: ['sword', 'axe'], bookMultiplier: 1, incompatible: ['sharpness', 'bane_of_arthropods'] },
	{ id: 'bane_of_arthropods', label: 'Fléau des arthropodes', maxLevel: 5, items: ['sword', 'axe'], bookMultiplier: 1, incompatible: ['sharpness', 'smite'] },
	{ id: 'knockback', label: 'Recul', maxLevel: 2, items: ['sword'], bookMultiplier: 1, incompatible: [] },
	{ id: 'fire_aspect', label: 'Aura de feu', maxLevel: 2, items: ['sword'], bookMultiplier: 2, incompatible: [] },
	{ id: 'looting', label: 'Butin', maxLevel: 3, items: ['sword'], bookMultiplier: 2, incompatible: [] },
	{ id: 'sweeping_edge', label: 'Affilage', maxLevel: 3, items: ['sword'], bookMultiplier: 2, incompatible: [] },

	// Tools
	{ id: 'efficiency', label: 'Efficacité', maxLevel: 5, items: ['pickaxe', 'shovel', 'axe', 'hoe'], bookMultiplier: 1, incompatible: [] },
	{ id: 'silk_touch', label: 'Toucher de soie', maxLevel: 1, items: ['pickaxe', 'shovel', 'axe', 'hoe'], bookMultiplier: 4, incompatible: ['fortune'] },
	{ id: 'fortune', label: 'Fortune', maxLevel: 3, items: ['pickaxe', 'shovel', 'axe', 'hoe'], bookMultiplier: 2, incompatible: ['silk_touch'] },

	// Armor (all)
	{ id: 'protection', label: 'Protection', maxLevel: 4, items: ['helmet', 'chestplate', 'leggings', 'boots'], bookMultiplier: 1, incompatible: ['fire_protection', 'blast_protection', 'projectile_protection'] },
	{ id: 'fire_protection', label: 'Protection contre le feu', maxLevel: 4, items: ['helmet', 'chestplate', 'leggings', 'boots'], bookMultiplier: 1, incompatible: ['protection', 'blast_protection', 'projectile_protection'] },
	{ id: 'blast_protection', label: 'Protection contre les explosions', maxLevel: 4, items: ['helmet', 'chestplate', 'leggings', 'boots'], bookMultiplier: 2, incompatible: ['protection', 'fire_protection', 'projectile_protection'] },
	{ id: 'projectile_protection', label: 'Protection contre les projectiles', maxLevel: 4, items: ['helmet', 'chestplate', 'leggings', 'boots'], bookMultiplier: 1, incompatible: ['protection', 'fire_protection', 'blast_protection'] },
	{ id: 'thorns', label: 'Épines', maxLevel: 3, items: ['helmet', 'chestplate', 'leggings', 'boots'], bookMultiplier: 4, incompatible: [] },
	{ id: 'unbreaking', label: 'Solidité', maxLevel: 3, items: ['sword', 'axe', 'pickaxe', 'shovel', 'hoe', 'helmet', 'chestplate', 'leggings', 'boots', 'bow', 'crossbow', 'trident', 'fishing_rod', 'mace'], bookMultiplier: 1, incompatible: [] },
	{ id: 'mending', label: 'Raccommodage', maxLevel: 1, items: ['sword', 'axe', 'pickaxe', 'shovel', 'hoe', 'helmet', 'chestplate', 'leggings', 'boots', 'bow', 'crossbow', 'trident', 'fishing_rod', 'mace'], bookMultiplier: 2, incompatible: [] },

	// Helmet
	{ id: 'respiration', label: 'Respiration', maxLevel: 3, items: ['helmet'], bookMultiplier: 2, incompatible: [] },
	{ id: 'aqua_affinity', label: 'Affinité aquatique', maxLevel: 1, items: ['helmet'], bookMultiplier: 2, incompatible: [] },

	// Boots
	{ id: 'feather_falling', label: 'Chute amortie', maxLevel: 4, items: ['boots'], bookMultiplier: 1, incompatible: [] },
	{ id: 'depth_strider', label: 'Agilité aquatique', maxLevel: 3, items: ['boots'], bookMultiplier: 2, incompatible: ['frost_walker'] },
	{ id: 'frost_walker', label: 'Semelles givrantes', maxLevel: 2, items: ['boots'], bookMultiplier: 2, incompatible: ['depth_strider'] },
	{ id: 'soul_speed', label: 'Vitesse des âmes', maxLevel: 3, items: ['boots'], bookMultiplier: 4, incompatible: [] },
	{ id: 'swift_sneak', label: 'Furtivité rapide', maxLevel: 3, items: ['leggings'], bookMultiplier: 4, incompatible: [] },

	// Bow
	{ id: 'power', label: 'Puissance', maxLevel: 5, items: ['bow'], bookMultiplier: 1, incompatible: [] },
	{ id: 'punch', label: 'Frappe', maxLevel: 2, items: ['bow'], bookMultiplier: 2, incompatible: [] },
	{ id: 'flame', label: 'Flamme', maxLevel: 1, items: ['bow'], bookMultiplier: 2, incompatible: [] },
	{ id: 'infinity', label: 'Infinité', maxLevel: 1, items: ['bow'], bookMultiplier: 4, incompatible: ['mending'] },

	// Crossbow
	{ id: 'quick_charge', label: 'Charge rapide', maxLevel: 3, items: ['crossbow'], bookMultiplier: 1, incompatible: [] },
	{ id: 'multishot', label: 'Tir multiple', maxLevel: 1, items: ['crossbow'], bookMultiplier: 2, incompatible: ['piercing'] },
	{ id: 'piercing', label: 'Perforation', maxLevel: 4, items: ['crossbow'], bookMultiplier: 1, incompatible: ['multishot'] },

	// Trident
	{ id: 'impaling', label: 'Empalement', maxLevel: 5, items: ['trident'], bookMultiplier: 2, incompatible: [] },
	{ id: 'riptide', label: 'Impulsion', maxLevel: 3, items: ['trident'], bookMultiplier: 2, incompatible: ['loyalty', 'channeling'] },
	{ id: 'loyalty', label: 'Loyauté', maxLevel: 3, items: ['trident'], bookMultiplier: 1, incompatible: ['riptide'] },
	{ id: 'channeling', label: 'Canalisation', maxLevel: 1, items: ['trident'], bookMultiplier: 4, incompatible: ['riptide'] },

	// Fishing rod
	{ id: 'luck_of_the_sea', label: 'Chance de la mer', maxLevel: 3, items: ['fishing_rod'], bookMultiplier: 2, incompatible: [] },
	{ id: 'lure', label: 'Appât', maxLevel: 3, items: ['fishing_rod'], bookMultiplier: 2, incompatible: [] },

	// Mace
	{ id: 'density', label: 'Densité', maxLevel: 5, items: ['mace'], bookMultiplier: 1, incompatible: ['breach', 'smite', 'bane_of_arthropods'] },
	{ id: 'breach', label: 'Brèche', maxLevel: 4, items: ['mace'], bookMultiplier: 2, incompatible: ['density', 'smite', 'bane_of_arthropods'] },
	{ id: 'wind_burst', label: 'Rafale', maxLevel: 3, items: ['mace'], bookMultiplier: 4, incompatible: [] },
];

// ── Enchantment selection ──

export interface SelectedEnchant {
	id: string;
	level: number;
}

/** Get compatible enchantments for an item category */
export function getCompatibleEnchants(category: string): EnchantDef[] {
	return ENCHANTMENTS.filter(e => e.items.includes(category));
}

/** Check if an enchant is compatible with existing selections */
export function isCompatible(enchantId: string, selected: SelectedEnchant[]): boolean {
	const def = ENCHANTMENTS.find(e => e.id === enchantId);
	if (!def) return false;
	for (const s of selected) {
		const sDef = ENCHANTMENTS.find(e => e.id === s.id);
		if (!sDef) continue;
		if (def.incompatible.includes(s.id) || sDef.incompatible.includes(enchantId)) return false;
	}
	return true;
}

// ── Anvil cost calculation ──

/**
 * Calculate the total XP cost to combine all enchantments from books onto an item.
 * Uses the MC anvil formula: cost = sum of (level × bookMultiplier) + prior work penalties.
 *
 * Returns the total XP levels cost and whether it exceeds "Too Expensive!" (39 levels per step).
 */
export interface AnvilStep {
	enchant: SelectedEnchant;
	label: string;
	stepCost: number;
	totalPenalty: number;
	tooExpensive: boolean;
}

export function calculateAnvilCost(enchants: SelectedEnchant[]): { steps: AnvilStep[]; totalCost: number; tooExpensive: boolean } {
	if (enchants.length === 0) return { steps: [], totalCost: 0, tooExpensive: false };

	const steps: AnvilStep[] = [];
	let priorWorkPenalty = 0; // item penalty: 2^n - 1 where n = number of operations
	let totalCost = 0;
	let tooExpensive = false;

	for (const ench of enchants) {
		const def = ENCHANTMENTS.find(e => e.id === ench.id);
		if (!def) continue;

		// Cost = level × multiplier + item prior work + book prior work (0 for fresh book)
		const enchCost = ench.level * def.bookMultiplier;
		const stepCost = enchCost + priorWorkPenalty;
		const stepTooExpensive = stepCost > 39;

		steps.push({
			enchant: ench,
			label: `${def.label} ${toRoman(ench.level)}`,
			stepCost,
			totalPenalty: priorWorkPenalty,
			tooExpensive: stepTooExpensive,
		});

		if (stepTooExpensive) tooExpensive = true;
		totalCost += stepCost;

		// Update prior work penalty: doubles each operation
		priorWorkPenalty = (priorWorkPenalty + 1) * 2 - 1;
	}

	return { steps, totalCost, tooExpensive };
}

/**
 * Find optimal order to minimize max step cost (avoid "Too Expensive!").
 * Strategy: apply cheapest enchants first (lowest level × multiplier) to minimize penalty accumulation.
 */
export function optimizeOrder(enchants: SelectedEnchant[]): SelectedEnchant[] {
	return [...enchants].sort((a, b) => {
		const da = ENCHANTMENTS.find(e => e.id === a.id);
		const db = ENCHANTMENTS.find(e => e.id === b.id);
		const costA = (a.level * (da?.bookMultiplier ?? 1));
		const costB = (b.level * (db?.bookMultiplier ?? 1));
		return costA - costB;
	});
}

function toRoman(n: number): string {
	const map: [number, string][] = [[5, 'V'], [4, 'IV'], [3, 'III'], [2, 'II'], [1, 'I']];
	let result = '';
	for (const [val, sym] of map) {
		while (n >= val) { result += sym; n -= val; }
	}
	return result;
}

// ── /give command generation ──

export function toGiveCommand(category: string, enchants: SelectedEnchant[], target: string = '@p'): string {
	const cat = ITEM_CATEGORIES.find(c => c.id === category);
	const item = cat?.items[cat.items.length - 1] ?? 'diamond_sword'; // best tier

	if (enchants.length === 0) return `/give ${target} minecraft:${item}`;

	const enchList = enchants.map(e => `{id:"minecraft:${e.id}",lvl:${e.level}}`).join(',');
	return `/give ${target} minecraft:${item}[minecraft:enchantments={levels:{${enchants.map(e => `"minecraft:${e.id}":${e.level}`).join(',')}}}]`;
}
