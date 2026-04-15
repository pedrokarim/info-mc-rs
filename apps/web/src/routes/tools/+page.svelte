<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte';

  const tools = [
    {
      href: '/tools/motd-editor',
      title: 'Éditeur MOTD',
      description: 'Créez des MOTD colorés pour votre serveur Minecraft. Couleurs, dégradés, formats et export multi-format.',
      image: '/images/ui/tool-motd-editor-v01.png',
    },
    {
      href: '/tools/skin-editor',
      title: 'Éditeur de Skin',
      description: 'Dessinez votre skin Minecraft pixel par pixel avec prévisualisation 3D en temps réel.',
      image: '/images/ui/tool-skin-editor-v01.png',
    },
    {
      href: '/tools/cape-editor',
      title: 'Éditeur de Cape',
      description: 'Créez vos capes Minecraft. Mode OptiFine, presets officiels, vue elytra et peinture 3D.',
      image: '/images/ui/tool-cape-editor-v01.png',
    },
    {
      href: '/capes',
      title: 'Galerie des Capes',
      description: 'Découvrez toutes les capes officielles Minecraft avec leur histoire, rareté et prévisualisation 3D.',
      image: '/images/ui/tool-cape-gallery-v01.png',
    },
    {
      href: '/tools/nbt-viewer',
      title: 'Visualiseur NBT',
      description: 'Explorez vos fichiers NBT Minecraft : arbre de tags et rendu 3D des structures en couleurs.',
      image: '/images/ui/tool-nbt-viewer-v01.png',
    },
    {
      href: '/tools/seed-map',
      title: 'Carte de Seed',
      description: 'Visualisez les biomes et slime chunks de votre monde Minecraft à partir de sa seed.',
      image: '/images/ui/tool-seed-map-v01.png',
    },
    {
      href: '/tools/coordinate-calculator',
      title: 'Calculateur de Coordonnées',
      description: 'Convertissez Overworld ↔ Nether, trouvez votre chunk et région, calculez les distances.',
      image: '/images/ui/tool-coordinate-calculator-v01.png',
    },
  ];
</script>

<svelte:head>
  <title>Outils Minecraft — MCInfo</title>
  <meta name="description" content="Outils gratuits pour Minecraft : éditeur de MOTD, éditeur de skin avec prévisualisation 3D, et plus encore." />
</svelte:head>

<main class="tools-page">
  <div class="tools-header">
    <h1 class="tools-title">Outils</h1>
    <p class="tools-subtitle">Outils gratuits pour Minecraft</p>
  </div>

  <div class="tools-grid">
    {#each tools as tool}
      <a class="tool-card" href={tool.href}>
        <img
          class="tool-icon-img"
          src={tool.image}
          alt={tool.title}
          width="100"
          height="100"
          loading="lazy"
        />
        <div class="tool-info">
          <h2 class="tool-name">{tool.title}</h2>
          <p class="tool-desc">{tool.description}</p>
        </div>
        <svg class="tool-arrow" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M5 12h14M12 5l7 7-7 7" />
        </svg>
      </a>
    {/each}
  </div>
</main>

<style>
  .tools-page {
    width: var(--layout-width, min(1160px, calc(100% - 2rem)));
    margin: 0 auto;
    padding-top: 2rem;
    padding-bottom: 3rem;
  }

  .tools-header {
    margin-bottom: 2rem;
  }

  .tools-title {
    font-family: 'Teko', sans-serif;
    font-size: 2.4rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1.1;
  }

  .tools-subtitle {
    font-size: 0.95rem;
    color: var(--ink-2, #5a7894);
    margin: 0.3rem 0 0;
  }

  .tools-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 16px;
  }

  .tool-card {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 20px 24px;
    background: var(--surface-1, #edf5fa);
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-lg, 16px);
    text-decoration: none;
    color: inherit;
    transition:
      border-color var(--ease, 160ms ease),
      transform var(--ease, 160ms ease),
      box-shadow var(--ease, 160ms ease);
  }

  .tool-card:hover {
    border-color: var(--blue-0, #5e90ff);
    transform: translateY(-2px);
    box-shadow: var(--shadow-sm, 0 8px 18px rgba(16, 45, 72, 0.16));
  }

  .tool-icon-img {
    width: 100px;
    height: 100px;
    border-radius: var(--radius-md, 12px);
    flex-shrink: 0;
    object-fit: contain;
    image-rendering: auto;
  }

  .tool-info {
    flex: 1;
    min-width: 0;
  }

  .tool-name {
    font-family: 'Teko', sans-serif;
    font-size: 1.35rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    margin: 0;
    line-height: 1.2;
  }

  .tool-desc {
    font-size: 0.8rem;
    color: var(--ink-2, #5a7894);
    margin: 0.25rem 0 0;
    line-height: 1.4;
  }

  .tool-arrow {
    color: var(--ink-2, #5a7894);
    flex-shrink: 0;
    transition: color var(--ease, 160ms ease), transform var(--ease, 160ms ease);
  }

  .tool-card:hover .tool-arrow {
    color: var(--blue-0, #5e90ff);
    transform: translateX(3px);
  }

  @media (max-width: 640px) {
    .tools-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
