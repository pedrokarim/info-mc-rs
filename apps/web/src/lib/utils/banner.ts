/** Minecraft Banner Designer — data & rendering utilities */

// ── MC Dye Colors ──

export interface DyeColor {
	id: number;
	name: string;
	label: string;
	hex: string;
}

export const DYE_COLORS: DyeColor[] = [
	{ id: 0,  name: 'white',      label: 'Blanc',        hex: '#F9FFFF' },
	{ id: 1,  name: 'orange',     label: 'Orange',       hex: '#F9801D' },
	{ id: 2,  name: 'magenta',    label: 'Magenta',      hex: '#C64FBD' },
	{ id: 3,  name: 'light_blue', label: 'Bleu clair',   hex: '#3AB3DA' },
	{ id: 4,  name: 'yellow',     label: 'Jaune',        hex: '#FED83D' },
	{ id: 5,  name: 'lime',       label: 'Vert clair',   hex: '#80C71F' },
	{ id: 6,  name: 'pink',       label: 'Rose',         hex: '#F38CAA' },
	{ id: 7,  name: 'gray',       label: 'Gris',         hex: '#474F52' },
	{ id: 8,  name: 'light_gray', label: 'Gris clair',   hex: '#9C9D97' },
	{ id: 9,  name: 'cyan',       label: 'Cyan',         hex: '#169C9D' },
	{ id: 10, name: 'purple',     label: 'Violet',       hex: '#8932B7' },
	{ id: 11, name: 'blue',       label: 'Bleu',         hex: '#3C44AA' },
	{ id: 12, name: 'brown',      label: 'Marron',       hex: '#835432' },
	{ id: 13, name: 'green',      label: 'Vert',         hex: '#5D7C15' },
	{ id: 14, name: 'red',        label: 'Rouge',        hex: '#B02E26' },
	{ id: 15, name: 'black',      label: 'Noir',         hex: '#1D1D21' },
];

// ── Banner Pattern Definitions ──

export interface PatternDef {
	id: string;
	label: string;
	code: string; // legacy NBT short code
	/** Returns true if pixel (x, y) in a (w × h) grid should be colored */
	mask: (x: number, y: number, w: number, h: number) => boolean;
}

// Helper: distance from center as fraction
const cdist = (x: number, y: number, w: number, h: number) => {
	const dx = (x + 0.5 - w / 2) / (w / 2);
	const dy = (y + 0.5 - h / 2) / (h / 2);
	return Math.sqrt(dx * dx + dy * dy);
};

export const PATTERNS: PatternDef[] = [
	// Stripes
	{ id: 'stripe_bottom', label: 'Base', code: 'bs', mask: (_x, y, _w, h) => y >= h * 2 / 3 },
	{ id: 'stripe_top', label: 'Chef', code: 'ts', mask: (_x, y, _w, h) => y < h / 3 },
	{ id: 'stripe_left', label: 'Pal dextre', code: 'ls', mask: (x, _y, w) => x < w / 3 },
	{ id: 'stripe_right', label: 'Pal senestre', code: 'rs', mask: (x, _y, w) => x >= w * 2 / 3 },
	{ id: 'stripe_center', label: 'Pal', code: 'cs', mask: (x, _y, w) => x >= w / 3 && x < w * 2 / 3 },
	{ id: 'stripe_middle', label: 'Fasce', code: 'ms', mask: (_x, y, _w, h) => y >= h / 3 && y < h * 2 / 3 },
	{ id: 'stripe_downright', label: 'Bande', code: 'drs',
		mask: (x, y, w, h) => { const t = y / h; const cx = t * w; return Math.abs(x - cx) < w / 5; } },
	{ id: 'stripe_downleft', label: 'Barre', code: 'dls',
		mask: (x, y, w, h) => { const t = y / h; const cx = (1 - t) * w; return Math.abs(x - cx) < w / 5; } },

	// Cross / saltire
	{ id: 'cross', label: 'Croix', code: 'cr',
		mask: (x, y, w, h) => (x >= w / 3 && x < w * 2 / 3) || (y >= h / 3 && y < h * 2 / 3) },
	{ id: 'saltire', label: 'Sautoir', code: 'sc',
		mask: (x, y, w, h) => {
			const nx = x / w, ny = y / h;
			return Math.abs(nx - ny) < 0.2 || Math.abs(nx - (1 - ny)) < 0.2;
		} },

	// Divisions
	{ id: 'per_fess', label: 'Coupé', code: 'hh', mask: (_x, y, _w, h) => y < h / 2 },
	{ id: 'per_fess_inverted', label: 'Coupé inversé', code: 'hhi', mask: (_x, y, _w, h) => y >= h / 2 },
	{ id: 'per_pale', label: 'Parti', code: 'vh', mask: (x, _y, w) => x < w / 2 },
	{ id: 'per_pale_inverted', label: 'Parti inversé', code: 'vhi', mask: (x, _y, w) => x >= w / 2 },
	{ id: 'per_bend', label: 'Tranché', code: 'db',
		mask: (x, y, w, h) => (x / w) < (y / h) },
	{ id: 'per_bend_inverted', label: 'Tranché inversé', code: 'dbi',
		mask: (x, y, w, h) => (x / w) >= (y / h) },
	{ id: 'per_bend_sinister', label: 'Taillé', code: 'dbs',
		mask: (x, y, w, h) => (x / w) + (y / h) < 1 },
	{ id: 'per_bend_sinister_inverted', label: 'Taillé inversé', code: 'dbsi',
		mask: (x, y, w, h) => (x / w) + (y / h) >= 1 },

	// Shapes
	{ id: 'roundel', label: 'Tourteau', code: 'mc', mask: (x, y, w, h) => cdist(x, y, w, h) < 0.45 },
	{ id: 'lozenge', label: 'Losange', code: 'mr',
		mask: (x, y, w, h) => {
			const nx = Math.abs((x + 0.5) / w - 0.5) * 2;
			const ny = Math.abs((y + 0.5) / h - 0.5) * 2;
			return nx + ny < 0.7;
		} },

	// Chevrons
	{ id: 'chevron', label: 'Chevron', code: 'tt',
		mask: (x, y, w, h) => {
			const nx = x / w, ny = y / h;
			return ny < 0.5 && ny > Math.abs(nx - 0.5);
		} },
	{ id: 'inverted_chevron', label: 'Chevron inversé', code: 'bt',
		mask: (x, y, w, h) => {
			const nx = x / w, ny = 1 - y / h;
			return ny < 0.5 && ny > Math.abs(nx - 0.5);
		} },

	// Triangles
	{ id: 'triangle_bottom', label: 'Triangle bas', code: 'tbl',
		mask: (x, y, w, h) => {
			const ny = y / h;
			const hw = (1 - ny) * 0.5;
			const nx = x / w;
			return ny > 0.5 && Math.abs(nx - 0.5) < hw;
		} },
	{ id: 'triangle_top', label: 'Triangle haut', code: 'tts',
		mask: (x, y, w, h) => {
			const ny = y / h;
			const hw = ny * 0.5;
			const nx = x / w;
			return ny < 0.5 && Math.abs(nx - 0.5) < hw;
		} },

	// Corners
	{ id: 'square_top_left', label: 'Canton haut-gauche', code: 'tl', mask: (x, y, w, h) => x < w / 3 && y < h / 3 },
	{ id: 'square_top_right', label: 'Canton haut-droit', code: 'tr', mask: (x, y, w, h) => x >= w * 2 / 3 && y < h / 3 },
	{ id: 'square_bottom_left', label: 'Canton bas-gauche', code: 'bl', mask: (x, y, w, h) => x < w / 3 && y >= h * 2 / 3 },
	{ id: 'square_bottom_right', label: 'Canton bas-droit', code: 'br', mask: (x, y, w, h) => x >= w * 2 / 3 && y >= h * 2 / 3 },

	// Gradient
	{ id: 'gradient', label: 'Dégradé haut', code: 'gra',
		mask: (_x, y, _w, h) => Math.random() < 1 - (y / h) },
	{ id: 'gradient_up', label: 'Dégradé bas', code: 'gru',
		mask: (_x, y, _w, h) => Math.random() < (y / h) },

	// Border
	{ id: 'border', label: 'Bordure', code: 'bo',
		mask: (x, y, w, h) => x < 2 || x >= w - 2 || y < 2 || y >= h - 2 },

	// Special (item-required)
	{ id: 'creeper', label: 'Creeper', code: 'cre',
		mask: (x, y, w, h) => {
			const nx = Math.floor(x / w * 8), ny = Math.floor(y / h * 10);
			// Simplified creeper face
			return (ny >= 2 && ny <= 3 && (nx === 2 || nx === 5)) || // eyes
				   (ny >= 4 && ny <= 5 && nx >= 3 && nx <= 4) || // nose
				   (ny >= 6 && ny <= 7 && nx >= 2 && nx <= 5 && !(ny === 7 && (nx === 3 || nx === 4))); // mouth
		} },
	{ id: 'skull', label: 'Crâne', code: 'sku',
		mask: (x, y, w, h) => {
			const nx = Math.floor(x / w * 8), ny = Math.floor(y / h * 10);
			return (ny >= 1 && ny <= 2 && nx >= 2 && nx <= 5) || // top
				   (ny >= 3 && ny <= 4 && nx >= 1 && nx <= 6) || // wide
				   (ny === 3 && (nx === 2 || nx === 5)) || // eyes
				   (ny >= 5 && ny <= 6 && nx >= 2 && nx <= 5); // jaw
		} },
	{ id: 'flower', label: 'Fleur', code: 'flo',
		mask: (x, y, w, h) => {
			const d = cdist(x, y, w, h);
			if (d < 0.12) return true; // center
			if (d > 0.15 && d < 0.35) {
				const angle = Math.atan2(y + 0.5 - h / 2, x + 0.5 - w / 2);
				return Math.cos(angle * 4) > 0.3; // petals
			}
			return false;
		} },
	{ id: 'mojang', label: 'Mojang', code: 'moj',
		mask: (x, y, w, h) => {
			const nx = x / w, ny = y / h;
			// Simplified M shape
			return ny > 0.3 && ny < 0.7 && (
				(nx > 0.15 && nx < 0.25) || (nx > 0.75 && nx < 0.85) || // sides
				(ny < 0.5 && nx > 0.35 && nx < 0.65) // top bar
			);
		} },
	{ id: 'globe', label: 'Globe', code: 'glb',
		mask: (x, y, w, h) => {
			const d = cdist(x, y, w, h);
			if (d > 0.45) return false;
			const nx = (x + 0.5 - w / 2) / (w / 2);
			return d < 0.45 && (Math.abs(nx) < 0.08 || Math.abs((y + 0.5 - h / 2) / (h / 2)) < 0.08 || (d > 0.35 && d < 0.45));
		} },
	{ id: 'piglin', label: 'Piglin', code: 'pig',
		mask: (x, y, w, h) => {
			const nx = Math.floor(x / w * 8), ny = Math.floor(y / h * 10);
			return (ny >= 1 && ny <= 2 && nx >= 1 && nx <= 6) || // head
				   (ny >= 3 && ny <= 4 && nx >= 2 && nx <= 5) || // snout
				   (ny === 2 && (nx === 2 || nx === 5)); // eyes
		} },
];

// ── Banner Layer ──

export interface BannerLayer {
	pattern: string; // pattern id
	color: number;   // dye color id (0-15)
}

export interface BannerState {
	baseColor: number; // dye color id for base
	layers: BannerLayer[];
}

// ── Rendering ──

const BANNER_W = 20;
const BANNER_H = 40;

export function renderBanner(
	ctx: CanvasRenderingContext2D,
	state: BannerState,
	x: number, y: number,
	displayW: number, displayH: number,
) {
	const pixW = displayW / BANNER_W;
	const pixH = displayH / BANNER_H;

	// Base color
	const baseHex = DYE_COLORS[state.baseColor]?.hex ?? '#FFFFFF';
	ctx.fillStyle = baseHex;
	ctx.fillRect(x, y, displayW, displayH);

	// Layers
	for (const layer of state.layers) {
		const pattern = PATTERNS.find(p => p.id === layer.pattern);
		if (!pattern) continue;
		const colorHex = DYE_COLORS[layer.color]?.hex ?? '#000000';
		ctx.fillStyle = colorHex;

		for (let py = 0; py < BANNER_H; py++) {
			for (let px = 0; px < BANNER_W; px++) {
				if (pattern.mask(px, py, BANNER_W, BANNER_H)) {
					ctx.fillRect(
						x + px * pixW,
						y + py * pixH,
						Math.ceil(pixW),
						Math.ceil(pixH),
					);
				}
			}
		}
	}
}

// ── Command generation ──

export function toGiveCommand(state: BannerState, target: string = '@p'): string {
	const baseName = DYE_COLORS[state.baseColor]?.name ?? 'white';
	if (state.layers.length === 0) {
		return `/give ${target} minecraft:${baseName}_banner`;
	}

	const patterns = state.layers.map(l => {
		const pat = PATTERNS.find(p => p.id === l.pattern);
		const colorName = DYE_COLORS[l.color]?.name ?? 'white';
		return `{pattern:"minecraft:${l.pattern}",color:"${colorName}"}`;
	}).join(',');

	return `/give ${target} minecraft:${baseName}_banner[minecraft:banner_patterns=[${patterns}]]`;
}

export { BANNER_W, BANNER_H };
