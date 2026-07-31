/** Mesure d'audience Sarutobi — initialisation, consentement et événements */

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
// stocké localement. `denyAnalytics()` est donc un opt-out durable, et il est
// désormais branché sur la page Confidentialité — ce que le commentaire
// annonçait depuis le premier jour sans que personne ne le fasse.

/**
 * Clé publique du site, telle que l'écran d'installation de Sarutobi la nomme.
 *
 * Elle s'appelait ici `PUBLIC_SARUTOBI_PROJECT_TOKEN`, du nom que le SDK
 * accepte encore par compatibilité mais n'affiche plus nulle part. Deux noms
 * pour une valeur se paient à chaque lecture, alors le nom du produit gagne.
 *
 * L'ancienne variable reste lue, et seulement lue : un déploiement en cours ne
 * doit pas perdre sa mesure d'audience parce qu'on a renommé une variable. Elle
 * disparaîtra une fois les environnements passés à `PUBLIC_SARUTOBI_SITE_ID`.
 */
function readSiteId(): string | undefined {
	return env.PUBLIC_SARUTOBI_SITE_ID || env.PUBLIC_SARUTOBI_PROJECT_TOKEN || undefined;
}

/**
 * Chemins jamais collectés, en plus du filtrage serveur.
 *
 * L'administration n'est pas de l'audience : elle est fréquentée par une
 * poignée de comptes, ses URL portent des identifiants de joueurs et de
 * serveurs, et ses visites fausseraient les chiffres des pages publiques sans
 * rien apprendre à personne. Le `/**` final couvre `/admin` lui-même autant que
 * ses enfants.
 */
const CHEMINS_EXCLUS = ['/admin/**'];

/**
 * Point d'entrée, appelé depuis `+layout.svelte`.
 *
 * Sans `siteId` configuré, la fonction ne fait rien : un déploiement ou un
 * environnement de développement non instrumenté doit fonctionner normalement,
 * sans erreur ni requête.
 *
 * L'appel est sans effet côté serveur — l'adaptateur se tait pendant le rendu
 * SSR — et idempotent, donc rejouable depuis un effet.
 */
export function startAnalytics(): void {
	const siteId = readSiteId();
	if (!siteId) return;

	initSarutobi({
		siteId,
		// Voir la note sur le consentement plus haut : sans cette ligne, le SDK
		// reste en `pending` et ne collecte rien.
		consent: 'granted',
		excludePaths: CHEMINS_EXCLUS,
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

/**
 * État courant du refus, pour pouvoir l'afficher.
 *
 * Une bascule qui ne sait pas dire dans quelle position elle se trouve laisse
 * le visiteur cliquer sans savoir ce qu'il vient de faire.
 */
export function analyticsDenied(): boolean {
	return useSarutobi().isOptedOut();
}

/**
 * Rattache la session à un compte d'administration.
 *
 * Utile alors même que `/admin/**` n'est pas collecté : l'identité persiste
 * au-delà de la connexion, donc les visites publiques d'un administrateur
 * cessent de gonfler les chiffres comme celles d'un visiteur de plus.
 *
 * **Sans propriétés, et c'est une contrainte, pas un oubli.** `identify()`
 * fait deux choses : il fixe l'identifiant localement, et il émet un événement
 * `$identify` qui porte les propriétés. La connexion a lieu sur
 * `/admin/login`, que `CHEMINS_EXCLUS` filtre — l'événement est donc jeté, et
 * toute propriété passée ici n'arriverait jamais.
 *
 * Le rattachement, lui, fonctionne : chaque envoi porte l'identifiant dans son
 * enveloppe, pas seulement l'événement `$identify`. Le premier passage sur une
 * page publique suffit à relier la personne et à fusionner son historique
 * anonyme.
 *
 * Aucune donnée personnelle non plus : l'identifiant du compte, rien d'autre.
 * Ni email ni pseudo — ils n'ajouteraient rien sur une population de quelques
 * comptes et sortiraient du minimum annoncé par la page Confidentialité.
 */
export function identifyAdmin(userId: string): void {
	useSarutobi().identify(userId);
}

/**
 * Détache la session du compte, à la déconnexion.
 *
 * Sans cet appel, le poste garde l'identité de l'administrateur : sur une
 * machine partagée, la navigation du suivant lui serait attribuée.
 */
export function resetIdentity(): void {
	useSarutobi().reset();
}

/** Événement métier, sans effet tant que le consentement n'est pas accordé. */
export function capture(name: string, props?: Props): void {
	useSarutobi().capture(name, props);
}

/**
 * Usage d'un outil de `/tools`.
 *
 * Les onze outils sont la raison d'être du site, et une pageview ne dit pas
 * s'ils servent : on ouvre le générateur de commandes, on regarde, on repart —
 * indistinguable de quelqu'un qui s'en sert. `tool_used` sépare les deux, et
 * c'est la seule question qu'on se pose vraiment sur cette section.
 *
 * `tool` porte l'identifiant de l'outil plutôt qu'un nom d'événement par outil :
 * onze noms distincts obligeraient à onze courbes pour lire une comparaison.
 */
export function captureToolUsed(tool: string, props?: Props): void {
	capture('tool_used', { tool, ...props });
}

/**
 * Déjà vus depuis le chargement de la page.
 *
 * Vidé à chaque navigation dure, jamais entre deux navigations SvelteKit — et
 * c'est voulu : compter deux fois un outil parce qu'on est passé par l'accueil
 * entre-temps redonnerait exactement le bruit qu'on cherche à éviter.
 */
const dejaVus = new Set<string>();

/**
 * Même chose, mais une seule fois par outil et par visite.
 *
 * Les calculateurs — coordonnées, enchantements — recalculent à chaque frappe.
 * Y brancher `captureToolUsed` produirait une trentaine d'événements pour une
 * seule utilisation, ce qui ne mesure plus l'usage mais la longueur du nombre
 * saisi. On ne garde que le premier résultat obtenu, qui est la seule chose
 * qu'on voulait savoir : cet outil a servi.
 *
 * `cle` distingue ce qui mérite d'être compté séparément — l'onglet d'un
 * calculateur, par exemple — sans multiplier les noms d'événements.
 */
export function captureToolUsedOnce(tool: string, cle?: string, props?: Props): void {
	const marque = cle ? `${tool}:${cle}` : tool;
	if (dejaVus.has(marque)) return;
	dejaVus.add(marque);
	captureToolUsed(tool, props);
}
