<script lang="ts">
  export let eyebrow = 'Minecraft Data Hub';
  export let title = 'Un front qui ressemble enfin a un vrai site de jeu.';
  export let description =
    'Hero pleine largeur avec une seule boite de description, lisible sur desktop et mobile.';
  export let image = '/images/hero/hero-main-v01.png';
  export let primaryLabel = 'Voir un serveur';
  export let primaryHref = '/server/play.hypixel.net';
  export let secondaryLabel = 'Explorer les skins';
  export let secondaryHref = '/skins';

  /**
   * Variante Ken Burns :
   *   zoom-in        — zoom lent vers l'avant depuis le centre
   *   zoom-out       — zoom lent vers l'arrière
   *   pan-right      — glissement gauche → droite
   *   pan-left       — glissement droite → gauche
   *   drift-up       — dérive bas → haut
   *   drift-diagonal — pan + zoom combinés (le plus cinématique)
   */
  export let variant:
    | 'zoom-in'
    | 'zoom-out'
    | 'pan-right'
    | 'pan-left'
    | 'drift-up'
    | 'drift-diagonal' = 'zoom-in';
</script>

<section class="hero-bleed">
  <div class="hero-stage">
    <div
      class="kb-layer kb-{variant}"
      style="background-image: url('{image}');"
      aria-hidden="true"
    ></div>
    <div class="hero-description">
      <p class="eyebrow">{eyebrow}</p>
      <h2>{title}</h2>
      <p>{description}</p>
      <div class="quick-actions">
        <a class="btn btn-primary" href={primaryHref}>{primaryLabel}</a>
        <a class="btn btn-secondary" href={secondaryHref}>{secondaryLabel}</a>
      </div>
    </div>
  </div>
</section>
