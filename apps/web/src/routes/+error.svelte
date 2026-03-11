<script lang="ts">
  import { page } from '$app/state';

  interface ErrorConfig {
    eyebrow: string;
    title: string;
    description: string;
    image: string;
    primaryLabel: string;
    primaryHref: string;
    secondaryLabel?: string;
    secondaryHref?: string;
  }

  function buildConfig(): ErrorConfig {
    const status = page.status;
    const message = page.error?.message;

    if (status === 404) {
      return {
        eyebrow: 'Erreur 404',
        title: 'Chunk introuvable',
        description:
          "Ce chunk n'a pas été généré. La page que tu cherches n'existe pas ou a été déplacée.",
        image: '/images/errors/error-404-v01.png',
        primaryLabel: "Retour à l'accueil",
        primaryHref: '/',
        secondaryLabel: 'Explorer les skins',
        secondaryHref: '/skins'
      };
    }

    if (status >= 500) {
      return {
        eyebrow: `Erreur ${status}`,
        title: 'Le serveur a crashé',
        description:
          "Une explosion de TNT côté serveur. Notre équipe travaille à l'extinction de l'incendie.",
        image: '/images/errors/error-5xx-v01.png',
        primaryLabel: "Retour à l'accueil",
        primaryHref: '/',
        secondaryLabel: 'Réessayer'
      };
    }

    return {
      eyebrow: `Erreur ${status}`,
      title: 'Accès refusé',
      description: message ?? "Tu n'as pas les permissions pour accéder à cette zone.",
      image: '/images/errors/error-generic-v01.png',
      primaryLabel: "Retour à l'accueil",
      primaryHref: '/'
    };
  }

  const cfg = $derived(buildConfig());

  function retry() {
    window.location.reload();
  }
</script>

<main class="error-root">
  <section class="error-stage">
    <div
      class="error-bg"
      style="background-image: url('{cfg.image}');"
      aria-hidden="true"
    ></div>

    <div class="error-content">
      <!-- Code fantôme en arrière-plan -->
      <span class="error-ghost" aria-hidden="true">{page.status}</span>

      <p class="error-eyebrow">{cfg.eyebrow}</p>
      <h1 class="error-title">{cfg.title}</h1>
      <p class="error-desc">{cfg.description}</p>

      <div class="error-actions">
        <a class="btn btn-primary" href={cfg.primaryHref}>{cfg.primaryLabel}</a>
        {#if cfg.secondaryLabel}
          {#if cfg.secondaryHref}
            <a class="btn btn-secondary" href={cfg.secondaryHref}>{cfg.secondaryLabel}</a>
          {:else}
            <button class="btn btn-secondary" onclick={retry}>{cfg.secondaryLabel}</button>
          {/if}
        {/if}
      </div>
    </div>
  </section>
</main>

<style>
  .error-root {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  /* ── Stage ─────────────────────────────────────────── */
  .error-stage {
    position: relative;
    flex: 1;
    min-height: min(520px, 72vh);
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  /* Image de fond */
  .error-bg {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center;
    background-color: #080e16; /* fallback si l'image n'est pas encore générée */
    transform: scale(1.04); /* léger zoom pour éviter les bords blancs */
  }

  /* Overlay gradient sombre */
  .error-stage::after {
    content: '';
    position: absolute;
    inset: 0;
    background:
      radial-gradient(ellipse 70% 80% at 50% 60%, rgba(5, 6, 8, 0.55), transparent),
      linear-gradient(180deg, rgba(5, 6, 8, 0.6) 0%, rgba(11, 19, 28, 0.78) 55%, rgba(5, 6, 8, 0.92) 100%);
  }

  /* ── Contenu ────────────────────────────────────────── */
  .error-content {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 3.5rem 2rem 4rem;
    max-width: 600px;
    gap: 0;
  }

  /* Code fantôme */
  .error-ghost {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -54%);
    font-family: 'Teko', sans-serif;
    font-size: clamp(8rem, 22vw, 16rem);
    font-weight: 700;
    line-height: 1;
    letter-spacing: -0.02em;
    color: rgba(255, 255, 255, 0.06);
    pointer-events: none;
    user-select: none;
    white-space: nowrap;
  }

  /* Eyebrow */
  .error-eyebrow {
    margin: 0 0 0.7rem;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: var(--gold-0);
    position: relative;
  }

  .error-eyebrow::before,
  .error-eyebrow::after {
    content: '—';
    margin: 0 0.5em;
    opacity: 0.6;
  }

  /* Titre principal */
  .error-title {
    margin: 0 0 1rem;
    font-family: 'Teko', sans-serif;
    font-size: clamp(2.6rem, 7vw, 4.4rem);
    font-weight: 700;
    line-height: 1;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    color: #fff;
    position: relative;
  }

  /* Description */
  .error-desc {
    margin: 0 0 2rem;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.92rem;
    line-height: 1.65;
    color: rgba(255, 255, 255, 0.65);
    max-width: 420px;
    position: relative;
  }

  /* Boutons */
  .error-actions {
    display: flex;
    gap: 0.7rem;
    justify-content: center;
    flex-wrap: wrap;
    position: relative;
  }

  /* Hérite des styles .btn, .btn-primary, .btn-secondary de app.css */

  /* ── Responsive ─────────────────────────────────────── */
  @media (max-width: 480px) {
    .error-actions {
      flex-direction: column;
      width: 100%;
      max-width: 280px;
    }

    .error-actions .btn {
      width: 100%;
      text-align: center;
    }
  }
</style>
