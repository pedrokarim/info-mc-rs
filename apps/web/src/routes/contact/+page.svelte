<script lang="ts">
  import SEO from '$lib/components/SEO.svelte';
</script>

<SEO
  title="Contact — MCInfo"
  description="Contactez l'équipe MCInfo pour toute question, suggestion ou signalement de bug."
  canonical="/contact"
/>

<main class="legal-page">
  <h1 class="legal-title">Contact</h1>
  <p class="legal-subtitle">Une question, une suggestion ou un bug ? On est là.</p>

  <section class="legal-section">
    <h2>Par email</h2>
    <p>Pour toute demande, vous pouvez nous écrire à :</p>
    <p class="legal-highlight"><a href="mailto:contact@ascencia.re">contact@ascencia.re</a></p>
    <p>Nous répondons généralement sous <strong>24 à 48 heures</strong>.</p>
  </section>

  <section class="legal-section">
    <h2>Discord</h2>
    <p>Rejoignez la communauté pour discuter, signaler des bugs ou proposer des idées :</p>
    <p class="legal-highlight"><a href="https://discord.gg/rTd95UpUEb" target="_blank" rel="noreferrer">Rejoindre le Discord</a></p>
  </section>

  <section class="legal-section">
    <h2>Sujets fréquents</h2>
    <ul>
      <li><strong>Bug ou problème technique</strong> — décrivez le comportement attendu vs observé, avec l'URL concernée si possible.</li>
      <li><strong>Suggestion ou idée</strong> — toute proposition est la bienvenue, sur Discord ou par email.</li>
      <li><strong>Partenariat</strong> — contactez-nous par email avec votre proposition.</li>
      <li><strong>Données personnelles</strong> — voir notre <a href="/confidentialite">politique de confidentialité</a> pour toute demande liée à vos données.</li>
    </ul>
  </section>

  <section class="legal-section">
    <h2>Éditeur du site</h2>
    <p>MCInfo est un projet développé par <strong>Ahmed Karim</strong> (PedroKarim), sous la marque <a href="https://ascencia.re/" target="_blank" rel="noreferrer">Ascencia</a>.</p>
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

  .legal-highlight {
    font-size: 1rem;
    font-weight: 600;
  }

  .legal-highlight a {
    color: #2a6faa;
    text-decoration: none;
    padding: 0.4rem 0.9rem;
    border: 1px solid rgba(42, 111, 170, 0.3);
    border-radius: 8px;
    background: rgba(42, 111, 170, 0.06);
    transition: background 150ms ease, border-color 150ms ease;
    display: inline-block;
  }

  .legal-highlight a:hover {
    background: rgba(42, 111, 170, 0.12);
    border-color: rgba(42, 111, 170, 0.5);
  }
</style>
