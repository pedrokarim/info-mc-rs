<script lang="ts">
  import { goto } from '$app/navigation';
  import SEO from '$lib/components/SEO.svelte';
  import SkinTile from '$lib/components/ui/SkinTile.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const apiBase = data.apiBase;

  let query = $state('');

  // Hardcoded skins to always fill the gallery
  const fallbackSkins = [
    'hiruai', 'sweetily', 'Mxodz', 'saphyras', 'cakeycat', 'Hamazushi14422',
    'marlowwwwwww', '1441', 'Sumugi', 'Sunlity', 'dhfjdhfjkhgdkhs', 'menuchah',
    'ee9e', 'Xekial_', 'Vietnamesin', 'tempvrance', 'ccquetel', 'quiet',
    'Sucie', 'Alesitu', 'ciubun', 'coldreason', 'Transgenres',
    'DeadbyTuesday0', 'Voilta_', 'MochiBud'
  ];

  // Merge: popular first, then fill with hardcoded (no duplicates)
  const popularNames = new Set(data.popularPlayers.map(p => p.username.toLowerCase()));
  const fillSkins = fallbackSkins.filter(u => !popularNames.has(u.toLowerCase()));

  interface DisplaySkin {
    username: string;
    tag?: string;
    fromApi: boolean;
  }

  const allSkins: DisplaySkin[] = [
    ...data.popularPlayers.map(p => ({
      username: p.username,
      tag: `${p.views} vues · ${p.likes} likes`,
      fromApi: true,
    })),
    ...fillSkins.map(u => ({
      username: u,
      fromApi: false,
    })),
  ];

  function submitSearch(event: SubmitEvent) {
    event.preventDefault();
    const clean = query.trim();
    if (!clean) return;
    goto(`/player/${encodeURIComponent(clean)}`);
  }

  function render3dUrl(username: string): string {
    return `${apiBase}/api/v1/render3d/${encodeURIComponent(username)}?width=256&height=256&theta=30&phi=21`;
  }
</script>

<SEO
  title="Skins Minecraft populaires — Galerie 3D"
  description="Explorez les skins Minecraft les plus populaires en rendu 3D. Viewer interactif, détection de capes, téléchargement PNG. Mis à jour en temps réel."
  canonical="/skins"
  jsonLd={{
    '@context': 'https://schema.org',
    '@type': 'CollectionPage',
    name: 'Skins Minecraft populaires',
    description: 'Galerie des skins Minecraft les plus recherchés avec rendu 3D interactif.',
    url: 'https://mcinfo.ascencia.re/skins'
  }}
/>

<main class="page">
  <section class="hero hero-skins">
    <div class="hero-copy">
      <p class="eyebrow">Skin Explorer</p>
      <h2>Skins Minecraft populaires</h2>
      <p>Galerie 3D générée par notre API Rust. Clique sur un skin pour voir le détail, le viewer interactif et les capes.</p>
      <div class="hero-tags">
        <span class="status neutral">Rendu 3D</span>
        <span class="status neutral">Cape detection</span>
        <span class="status neutral">Viewer interactif</span>
      </div>
    </div>
    <div class="hero-media hero-media-skins">
      <div class="hero-overlay">
        <p>Skins populaires</p>
        <h3>Top hebdomadaire NameMC</h3>
      </div>
    </div>
  </section>

  <section class="surface lookup-panel">
    <SectionHeading
      title="Recherche joueur"
      description="Entre un pseudo pour ouvrir la page détail skin avec viewer 3D."
      light={true}
    />
    <form class="lookup-form" onsubmit={submitSearch}>
      <SearchInputRow bind:value={query} placeholder="Notch, Dream, hiruai..." actionLabel="Voir le skin" />
    </form>
  </section>

  <section class="surface lookup-panel">
    <SectionHeading
      title="Skins populaires"
      description="Les joueurs les plus recherchés sur MCInfo — rendus 3D en temps réel."
      light={true}
    />

    <div class="gallery-grid skin-gallery">
      {#each allSkins as skin, i}
        <SkinTile
          username={skin.username}
          rank={i + 1}
          image={render3dUrl(skin.username)}
          href={`/player/${encodeURIComponent(skin.username)}`}
          tag={skin.tag}
        />
      {/each}
    </div>
  </section>
</main>

<style>
  /* Square 3D renders matching NameMC style */
  :global(.skin-gallery .skin-card img) {
    aspect-ratio: 1 / 1;
    background: linear-gradient(180deg, #e8f0f7 0%, #d4e4f0 100%);
  }

  .skin-gallery {
    grid-template-columns: repeat(6, minmax(0, 1fr));
  }

  @media (max-width: 980px) {
    .skin-gallery {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }
  }

  @media (max-width: 600px) {
    .skin-gallery {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }
</style>
