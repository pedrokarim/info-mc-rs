<script lang="ts">
  import { goto } from '$app/navigation';
  import { env } from '$env/dynamic/public';
  import SkinTile from '$lib/components/ui/SkinTile.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let query = $state('');

  // Popular skins from NameMC weekly chart (actual Minecraft usernames, no emoji)
  const popularSkins = [
    'hiruai', 'sweetily', 'Mxodz', 'saphyras', 'cakeycat', 'Hamazushi14422',
    'marlowwwwwww', '1441', 'Sumugi', 'Sunlity', 'dhfjdhfjkhgdkhs', 'menuchah',
    'ee9e', 'Xekial_', 'Vietnamesin', 'tempvrance', 'ccquetel', 'quiet',
    'Sucie', 'Alesitu', 'ciubun', 'coldreason', 'Transgenres',
    'DeadbyTuesday0', 'Voilta_', 'MochiBud'
  ];

  function submitSearch(event: SubmitEvent) {
    event.preventDefault();
    const clean = query.trim();
    if (!clean) return;
    goto(`/player/${encodeURIComponent(clean)}`);
  }

  function render3dUrl(username: string): string {
    return `${apiBase}/api/v1/render3d/${encodeURIComponent(username)}?width=240&height=360&theta=30&phi=21`;
  }
</script>

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
      description="Top hebdomadaire — rendus 3D générés par l'API en temps réel."
      light={true}
    />

    <div class="gallery-grid skin-gallery">
      {#each popularSkins as username, i}
        <SkinTile
          {username}
          rank={i + 1}
          image={render3dUrl(username)}
          href={`/player/${encodeURIComponent(username)}`}
        />
      {/each}
    </div>
  </section>
</main>

<style>
  /* Portrait 3D renders — override global 1/1 aspect ratio */
  :global(.skin-gallery .skin-card img) {
    aspect-ratio: 2 / 3;
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
