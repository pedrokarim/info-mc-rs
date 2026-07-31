<script lang="ts">
  import { onMount } from 'svelte';
  import SEO from '$lib/components/SEO.svelte';
  import { analyticsDenied, denyAnalytics, grantAnalytics } from '$lib/analytics';

  // L'état vit dans le stockage local du visiteur : il n'existe pas au rendu
  // serveur. On part donc de « accepté », qui est le défaut du site, et on
  // corrige au montage plutôt que d'afficher une bascule vide le temps de
  // l'hydratation.
  let refuse = $state(false);
  let pret = $state(false);

  onMount(() => {
    refuse = analyticsDenied();
    pret = true;
  });

  function basculer() {
    if (refuse) grantAnalytics();
    else denyAnalytics();
    refuse = !refuse;
  }
</script>

<SEO
  title="Politique de Confidentialité — MCInfo"
  description="Politique de confidentialité de MCInfo : données collectées, cookies et droits des utilisateurs."
  canonical="/confidentialite"
/>

<main class="legal-page">
  <h1 class="legal-title">Politique de Confidentialité</h1>
  <p class="legal-subtitle">Dernière mise à jour : avril 2026</p>

  <section class="legal-section">
    <h2>Responsable du traitement</h2>
    <p>Le responsable du traitement des données est Ahmed Karim (PedroKarim), opérant sous la marque <a href="https://ascencia.re/" target="_blank" rel="noreferrer">Ascencia</a>.</p>
    <p>Contact : <a href="mailto:contact@ascencia.re">contact@ascencia.re</a></p>
  </section>

  <section class="legal-section">
    <h2>Données collectées</h2>
    <p>MCInfo collecte le minimum de données nécessaires au fonctionnement du service :</p>
    <ul>
      <li><strong>Données de navigation</strong> — adresse IP, user-agent, pages visitées. Ces données sont utilisées à des fins de statistiques anonymes et de sécurité.</li>
      <li><strong>Recherches</strong> — les pseudos joueurs et adresses de serveurs recherchés sont temporairement mis en cache pour améliorer les performances. Ces données sont publiques par nature (API Mojang, protocole Minecraft).</li>
      <li><strong>Cookies</strong> — MCInfo utilise uniquement des cookies techniques essentiels au fonctionnement du site. Aucun cookie publicitaire ou de tracking tiers n'est utilisé.</li>
      <li><strong>Mesure d'audience</strong> — les pages consultées et les outils utilisés sont comptabilisés par <a href="https://sarutobi.ascencia.re/" target="_blank" rel="noreferrer">Sarutobi</a>, une instance auto-hébergée par Ascencia. Aucune donnée n'est transmise à un tiers, et un identifiant anonyme est conservé dans votre navigateur pour ne pas compter deux fois la même visite. Vous pouvez la refuser ci-dessous.</li>
    </ul>
  </section>

  <section class="legal-section">
    <h2>Refuser la mesure d'audience</h2>
    <p>
      La mesure démarre dès la première visite, sans bandeau : elle ne sert qu'à savoir quelles pages
      et quels outils sont consultés. Vous pouvez vous y opposer à tout moment, et ce choix est
      conservé dans ce navigateur.
    </p>

    <div class="optout">
      <p class="optout-state" aria-live="polite">
        {#if !pret}
          Chargement de votre choix…
        {:else if refuse}
          La mesure d'audience est <strong>désactivée</strong> sur ce navigateur.
        {:else}
          La mesure d'audience est <strong>active</strong> sur ce navigateur.
        {/if}
      </p>
      <button type="button" class="optout-button" onclick={basculer} disabled={!pret}>
        {refuse ? 'Réactiver la mesure' : 'Refuser la mesure'}
      </button>
    </div>

    <p class="optout-note">
      Ce réglage est enregistré localement : il ne suit pas votre compte et devra être refait sur un
      autre navigateur ou après effacement des données du site.
    </p>
  </section>

  <section class="legal-section">
    <h2>Utilisation des données</h2>
    <p>Les données collectées sont utilisées exclusivement pour :</p>
    <ul>
      <li>Assurer le fonctionnement et la performance du service</li>
      <li>Produire des statistiques de consultation anonymes</li>
      <li>Afficher les contenus populaires (skins, serveurs les plus consultés)</li>
      <li>Protéger le service contre les abus</li>
    </ul>
    <p><strong>Aucune donnée n'est vendue, partagée ou transmise à des tiers à des fins commerciales.</strong></p>
  </section>

  <section class="legal-section">
    <h2>Durée de conservation</h2>
    <p>Les données de cache (serveurs, skins) sont conservées de quelques minutes à quelques heures selon leur nature.</p>
    <p>Les statistiques de consultation anonymisées peuvent être conservées sans limite de durée.</p>
  </section>

  <section class="legal-section">
    <h2>Vos droits</h2>
    <p>Conformément au RGPD, vous disposez des droits suivants :</p>
    <ul>
      <li><strong>Accès</strong> — obtenir une copie des données vous concernant</li>
      <li><strong>Rectification</strong> — corriger des données inexactes</li>
      <li><strong>Suppression</strong> — demander l'effacement de vos données</li>
      <li><strong>Opposition</strong> — vous opposer au traitement de vos données</li>
    </ul>
    <p>Pour exercer ces droits, contactez-nous à <a href="mailto:contact@ascencia.re">contact@ascencia.re</a>.</p>
  </section>

  <section class="legal-section">
    <h2>Services tiers</h2>
    <p>MCInfo interagit avec les services suivants pour fournir ses fonctionnalités :</p>
    <ul>
      <li><strong>API Mojang</strong> — résolution de pseudos et récupération de skins (données publiques)</li>
      <li><strong>Protocole Minecraft</strong> — ping des serveurs pour afficher leur statut en temps réel</li>
    </ul>
  </section>
</main>

<style>
  .legal-page {
    width: var(--layout-width, min(800px, calc(100% - 2rem)));
    margin: 2rem auto 3rem;
  }

  .legal-title {
    font-family: 'Teko', sans-serif;
    font-size: 2.4rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1.1;
  }

  .legal-subtitle {
    font-size: 0.95rem;
    color: var(--ink-2, #5a7894);
    margin: 0.3rem 0 0;
  }

  .legal-section {
    margin-top: 1.8rem;
    padding-top: 1.4rem;
    border-top: 1px solid rgba(72, 112, 156, 0.2);
  }

  .legal-section h2 {
    font-family: 'Teko', sans-serif;
    font-size: 1.4rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0 0 0.6rem;
    line-height: 1.15;
  }

  .legal-section p,
  .legal-section li {
    font-size: 0.88rem;
    color: var(--ink-1, #2a4a66);
    line-height: 1.6;
    margin: 0.3rem 0;
  }

  .legal-section ul {
    padding-left: 1.2rem;
    margin: 0.5rem 0;
  }

  .legal-section li {
    margin: 0.4rem 0;
  }

  .legal-section a {
    color: #2a6faa;
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .legal-section a:hover {
    color: #0e3a62;
  }

  .optout {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.8rem;
    margin: 0.9rem 0 0;
    padding: 0.9rem 1rem;
    border: 1px solid rgba(72, 112, 156, 0.25);
    border-radius: 10px;
    background: rgba(72, 112, 156, 0.05);
  }

  .optout-state {
    margin: 0;
    font-size: 0.88rem;
    color: var(--ink-1, #2a4a66);
  }

  .optout-button {
    flex-shrink: 0;
    padding: 0.5rem 0.95rem;
    border: 1px solid #2a6faa;
    border-radius: 8px;
    background: #2a6faa;
    color: #fff;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition:
      background 0.15s ease,
      opacity 0.15s ease;
  }

  .optout-button:hover:not(:disabled) {
    background: #0e3a62;
  }

  .optout-button:disabled {
    opacity: 0.55;
    cursor: default;
  }

  .optout-button:focus-visible {
    outline: 2px solid #0e3a62;
    outline-offset: 2px;
  }

  .optout-note {
    font-size: 0.8rem !important;
    color: var(--ink-2, #5a7894) !important;
    margin-top: 0.6rem !important;
  }
</style>
