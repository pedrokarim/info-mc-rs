<script lang="ts">
  import SEO from '$lib/components/SEO.svelte';
</script>

<SEO
  title="Mentions Légales — MCInfo"
  description="Mentions légales du site MCInfo : éditeur, hébergement et propriété intellectuelle."
  canonical="/mentions-legales"
/>

<main class="legal-page">
  <h1 class="legal-title">Mentions Légales</h1>
  <p class="legal-subtitle">Informations légales relatives au site mcinfo.ascencia.re</p>

  <section class="legal-section">
    <h2>Éditeur du site</h2>
    <ul class="legal-info-list">
      <li><span class="legal-label">Responsable</span> Ahmed Karim (PedroKarim)</li>
      <li><span class="legal-label">Marque</span> <a href="https://ascencia.re/" target="_blank" rel="noreferrer">Ascencia</a></li>
      <li><span class="legal-label">Email</span> <a href="mailto:contact@ascencia.re">contact@ascencia.re</a></li>
      <li><span class="legal-label">Site</span> <a href="https://mcinfo.ascencia.re">mcinfo.ascencia.re</a></li>
    </ul>
  </section>

  <section class="legal-section">
    <h2>Propriété intellectuelle</h2>
    <p>L'ensemble du contenu de ce site (code source, textes, visuels, logos) est la propriété d'Ascencia, sauf mention contraire.</p>
    <p>Minecraft est une marque déposée de Mojang Studios / Microsoft Corporation. <strong>MCInfo n'est ni affilié, ni approuvé par Mojang ou Microsoft.</strong></p>
    <p>Les données de serveurs, skins et joueurs sont récupérées via les API publiques de Mojang et les protocoles ouverts de Minecraft.</p>
  </section>

  <section class="legal-section">
    <h2>Hébergement</h2>
    <p>Le site est hébergé sur un VPS chez <strong>Contabo GmbH</strong>, Aschauer Str. 32a, 81549 Munich, Allemagne. Pour toute demande relative à l'hébergement, contactez-nous via <a href="mailto:contact@ascencia.re">contact@ascencia.re</a>.</p>
  </section>

  <section class="legal-section">
    <h2>Données personnelles</h2>
    <p>Pour toute information concernant la collecte et le traitement de vos données, consultez notre <a href="/confidentialite">politique de confidentialité</a>.</p>
  </section>

  <section class="legal-section">
    <h2>Crédits</h2>
    <p>MCInfo est un projet open-source développé par PedroKarim sous la marque Ascencia.</p>
    <p>Les icônes et visuels des outils sont des créations originales du projet.</p>
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

  .legal-section a {
    color: #2a6faa;
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .legal-section a:hover {
    color: #0e3a62;
  }

  .legal-info-list {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0;
    display: flex;
    flex-direction: column;
    gap: 0.45rem;
  }

  .legal-info-list li {
    display: flex;
    align-items: baseline;
    gap: 0.6rem;
  }

  .legal-label {
    flex-shrink: 0;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--ink-2, #5a7894);
    min-width: 90px;
  }
</style>
