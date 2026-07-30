/** Mesure d'audience Sarutobi — initialisation et consentement */

import { env } from '$env/dynamic/public';
import { initSarutobi, useSarutobi, type Props } from '@ascencia/sarutobi-svelte';

// ── Consentement ──
//
// Choix du projet : la collecte démarre dès la première visite, sans bandeau.
// `consent: 'granted'` est passé explicitement — le défaut du SDK est `pending`,
// qui ne collecte rien. C'est une décision assumée, pas un défaut hérité : la
// documentation de Sarutobi précise qu'il ne se prétend pas exempté de
// consentement au sens CNIL.
//
// Un refus déjà enregistré l'emporte sur ce défaut : le SDK lit d'abord l'état
// stocké localement. `denyAnalytics()` reste donc un opt-out fonctionnel et
// durable, à brancher sur la page Confidentialité.

/**
 * Point d'entrée, appelé depuis `+layout.svelte`.
 *
 * Sans jeton configuré, la fonction ne fait rien : un déploiement ou un
 * environnement de développement sans `PUBLIC_SARUTOBI_PROJECT_TOKEN` doit
 * fonctionner normalement, sans erreur ni requête.
 *
 * L'appel est sans effet côté serveur — l'adaptateur se tait pendant le rendu
 * SSR — et idempotent, donc rejouable depuis un effet.
 */
export function startAnalytics(): void {
	const projectToken = env.PUBLIC_SARUTOBI_PROJECT_TOKEN;
	if (!projectToken) return;

	initSarutobi({
		projectToken,
		// Voir la note sur le consentement plus haut : sans cette ligne, le SDK
		// reste en `pending` et ne collecte rien.
		consent: 'granted',
		// Le suivi des navigations passe par l'API `history`, que SvelteKit
		// utilise : rien à brancher sur `afterNavigate`.
		environment: env.PUBLIC_SARUTOBI_ENVIRONMENT || 'production'
	});
}

/** À appeler quand le visiteur accepte la mesure d'audience. */
export function grantAnalytics(): void {
	useSarutobi().setConsent('granted');
}

/** À appeler quand le visiteur la refuse, ou revient sur son accord. */
export function denyAnalytics(): void {
	useSarutobi().setConsent('denied');
}

/** Événement métier, sans effet tant que le consentement n'est pas accordé. */
export function capture(name: string, props?: Props): void {
	useSarutobi().capture(name, props);
}
