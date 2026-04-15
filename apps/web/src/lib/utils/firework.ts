/** Minecraft Firework Designer — data & rendering utilities */

// ── Firework explosion shapes ──

export interface FireworkShape {
	id: number;
	name: string;
	label: string;
}

export const SHAPES: FireworkShape[] = [
	{ id: 0, name: 'small_ball', label: 'Petite boule' },
	{ id: 1, name: 'large_ball', label: 'Grande boule' },
	{ id: 2, name: 'star',       label: 'Étoile' },
	{ id: 3, name: 'creeper',    label: 'Creeper' },
	{ id: 4, name: 'burst',      label: 'Burst' },
];

// ── MC dye colors (reuse banner colors) ──

export const FIREWORK_COLORS: { name: string; label: string; hex: string; decimal: number }[] = [
	{ name: 'white',      label: 'Blanc',        hex: '#F9FFFF', decimal: 16383998 },
	{ name: 'orange',     label: 'Orange',        hex: '#F9801D', decimal: 16351261 },
	{ name: 'magenta',    label: 'Magenta',       hex: '#C64FBD', decimal: 12801229 },
	{ name: 'light_blue', label: 'Bleu clair',    hex: '#3AB3DA', decimal: 3847130 },
	{ name: 'yellow',     label: 'Jaune',         hex: '#FED83D', decimal: 16701501 },
	{ name: 'lime',       label: 'Vert clair',    hex: '#80C71F', decimal: 8439583 },
	{ name: 'pink',       label: 'Rose',          hex: '#F38CAA', decimal: 15961002 },
	{ name: 'gray',       label: 'Gris',          hex: '#474F52', decimal: 4673362 },
	{ name: 'light_gray', label: 'Gris clair',    hex: '#9C9D97', decimal: 10329495 },
	{ name: 'cyan',       label: 'Cyan',          hex: '#169C9D', decimal: 1481884 },
	{ name: 'purple',     label: 'Violet',        hex: '#8932B7', decimal: 8991416 },
	{ name: 'blue',       label: 'Bleu',          hex: '#3C44AA', decimal: 3949738 },
	{ name: 'brown',      label: 'Marron',        hex: '#835432', decimal: 8606770 },
	{ name: 'green',      label: 'Vert',          hex: '#5D7C15', decimal: 6192150 },
	{ name: 'red',        label: 'Rouge',         hex: '#B02E26', decimal: 11546150 },
	{ name: 'black',      label: 'Noir',          hex: '#1D1D21', decimal: 1908001 },
];

// ── Firework explosion definition ──

export interface FireworkExplosion {
	shape: number;
	colors: number[];    // indices into FIREWORK_COLORS
	fadeColors: number[]; // indices into FIREWORK_COLORS
	trail: boolean;
	twinkle: boolean;
}

export interface FireworkState {
	flight: number; // 1-3
	explosions: FireworkExplosion[];
}

export function createExplosion(): FireworkExplosion {
	return { shape: 0, colors: [14], fadeColors: [], trail: false, twinkle: false };
}

// ── Command generation (1.20.5+ component format) ──

function colorToDecimal(idx: number): number {
	return FIREWORK_COLORS[idx]?.decimal ?? 16383998;
}

export function toGiveCommand(state: FireworkState, target: string = '@p'): string {
	if (state.explosions.length === 0) {
		return `/give ${target} minecraft:firework_rocket[minecraft:fireworks={flight_duration:${state.flight}}]`;
	}

	const explosions = state.explosions.map(exp => {
		const parts: string[] = [];
		parts.push(`shape:"${SHAPES[exp.shape]?.name ?? 'small_ball'}"`);

		if (exp.colors.length > 0) {
			parts.push(`colors:[I;${exp.colors.map(colorToDecimal).join(',')}]`);
		}
		if (exp.fadeColors.length > 0) {
			parts.push(`fade_colors:[I;${exp.fadeColors.map(colorToDecimal).join(',')}]`);
		}
		if (exp.trail) parts.push('has_trail:1b');
		if (exp.twinkle) parts.push('has_twinkle:1b');

		return `{${parts.join(',')}}`;
	}).join(',');

	return `/give ${target} minecraft:firework_rocket[minecraft:fireworks={flight_duration:${state.flight},explosions:[${explosions}]}]`;
}

// ── Canvas preview rendering ──

export function renderFireworkPreview(
	ctx: CanvasRenderingContext2D,
	state: FireworkState,
	w: number, h: number,
	time: number, // animation time 0..1
) {
	// Dark sky background
	ctx.fillStyle = '#0a0a1e';
	ctx.fillRect(0, 0, w, h);

	// Stars
	const starSeed = 42;
	for (let i = 0; i < 30; i++) {
		const sx = ((starSeed * (i + 1) * 7) % 1000) / 1000 * w;
		const sy = ((starSeed * (i + 1) * 13) % 1000) / 1000 * h * 0.6;
		const brightness = 0.3 + Math.sin(time * 4 + i) * 0.15;
		ctx.fillStyle = `rgba(255,255,255,${brightness})`;
		ctx.fillRect(sx, sy, 1.5, 1.5);
	}

	if (state.explosions.length === 0) {
		// Just the rocket trail
		drawRocketTrail(ctx, w / 2, h, w / 2, h * 0.4, time);
		return;
	}

	// Render each explosion
	const count = state.explosions.length;
	for (let i = 0; i < count; i++) {
		const exp = state.explosions[i];
		const cx = w * (i + 1) / (count + 1);
		const cy = h * 0.35 + Math.sin(i * 1.5) * h * 0.08;

		// Rocket trail (pre-explosion)
		if (time < 0.3) {
			const trailProgress = time / 0.3;
			const ty = h - (h - cy) * trailProgress;
			drawRocketTrail(ctx, cx, h, cx, ty, 1);
		}

		// Explosion
		if (time >= 0.2) {
			const expTime = Math.min(1, (time - 0.2) / 0.6);
			drawExplosion(ctx, exp, cx, cy, Math.min(w, h) * 0.3, expTime);
		}
	}
}

function drawRocketTrail(ctx: CanvasRenderingContext2D, x1: number, y1: number, x2: number, y2: number, progress: number) {
	const grad = ctx.createLinearGradient(x1, y1, x2, y2);
	grad.addColorStop(0, 'rgba(255,200,50,0)');
	grad.addColorStop(Math.min(1, progress), 'rgba(255,200,50,0.8)');
	ctx.strokeStyle = grad;
	ctx.lineWidth = 2;
	ctx.beginPath();
	ctx.moveTo(x1, y1);
	ctx.lineTo(x2, y2);
	ctx.stroke();
}

function drawExplosion(
	ctx: CanvasRenderingContext2D,
	exp: FireworkExplosion,
	cx: number, cy: number,
	maxRadius: number,
	time: number, // 0..1
) {
	const colors = exp.colors.length > 0 ? exp.colors : [0]; // white fallback
	const fadeActive = time > 0.6 && exp.fadeColors.length > 0;
	const activeColors = fadeActive ? exp.fadeColors : colors;
	const alpha = time < 0.8 ? 1 : 1 - (time - 0.8) / 0.2;
	const radius = maxRadius * Math.min(1, time * 1.5);

	const particleCount = exp.shape === 1 ? 60 : exp.shape === 4 ? 40 : 30;

	for (let i = 0; i < particleCount; i++) {
		const colorIdx = activeColors[i % activeColors.length];
		const hex = FIREWORK_COLORS[colorIdx]?.hex ?? '#FFFFFF';

		let angle: number, dist: number;

		switch (exp.shape) {
			case 0: // small ball
				angle = (i / particleCount) * Math.PI * 2;
				dist = radius * 0.6;
				break;
			case 1: // large ball
				angle = (i / particleCount) * Math.PI * 2;
				dist = radius;
				break;
			case 2: { // star (5 points)
				const starAngle = (i / particleCount) * Math.PI * 2;
				const pointDist = i % 2 === 0 ? radius : radius * 0.45;
				angle = starAngle;
				dist = pointDist;
				break;
			}
			case 3: { // creeper face (clustered)
				angle = (i / particleCount) * Math.PI * 2 + Math.sin(i * 3) * 0.3;
				dist = radius * 0.5 + Math.sin(i * 7) * radius * 0.2;
				break;
			}
			case 4: // burst
				angle = (i / particleCount) * Math.PI * 2;
				dist = radius * (0.3 + (i % 3) * 0.25);
				break;
			default:
				angle = (i / particleCount) * Math.PI * 2;
				dist = radius * 0.6;
		}

		const px = cx + Math.cos(angle) * dist;
		const py = cy + Math.sin(angle) * dist + (time > 0.4 ? (time - 0.4) * 30 : 0); // gravity

		// Glow
		ctx.globalAlpha = alpha * 0.3;
		ctx.fillStyle = hex;
		ctx.beginPath();
		ctx.arc(px, py, exp.trail ? 5 : 3, 0, Math.PI * 2);
		ctx.fill();

		// Core
		ctx.globalAlpha = alpha;
		ctx.fillStyle = hex;
		ctx.beginPath();
		ctx.arc(px, py, exp.trail ? 2.5 : 1.5, 0, Math.PI * 2);
		ctx.fill();

		// Trail
		if (exp.trail && time > 0.1) {
			ctx.globalAlpha = alpha * 0.4;
			ctx.strokeStyle = hex;
			ctx.lineWidth = 0.8;
			const trailLen = dist * 0.3;
			ctx.beginPath();
			ctx.moveTo(px, py);
			ctx.lineTo(cx + Math.cos(angle) * (dist - trailLen), cy + Math.sin(angle) * (dist - trailLen));
			ctx.stroke();
		}

		// Twinkle
		if (exp.twinkle && Math.sin(time * 20 + i * 2) > 0.5) {
			ctx.globalAlpha = alpha * 0.8;
			ctx.fillStyle = '#fff';
			ctx.beginPath();
			ctx.arc(px, py, 1, 0, Math.PI * 2);
			ctx.fill();
		}
	}

	ctx.globalAlpha = 1;
}
