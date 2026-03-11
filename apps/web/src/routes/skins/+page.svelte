<script lang="ts">
  import { goto } from '$app/navigation';
  import { env } from '$env/dynamic/public';
  import SkinTile from '$lib/components/ui/SkinTile.svelte';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let query = $state('');

  const featuredSkins = [
    { username: 'Notch', tag: 'Classic legend' },
    { username: 'Dream', tag: 'Iconic profile' },
    { username: 'Technoblade', tag: 'Community favorite' },
    { username: 'Skeppy', tag: 'PvP style' },
    { username: 'Hypixel', tag: 'Brand skin' },
    { username: 'jeb_', tag: 'Mojang team' },
    { username: 'Dinnerbone', tag: 'Mojang team' },
    { username: 'Herobrine', tag: 'Myth sample' }
  ];

  function submitSearch(event: SubmitEvent) {
    event.preventDefault();
    const clean = query.trim();
    if (!clean) return;
    goto(`/player/${encodeURIComponent(clean)}`);
  }

  function headUrl(username: string): string {
    return `${apiBase}/api/v1/render/${encodeURIComponent(username)}?type=head&size=160&overlay=true`;
  }
</script>

<main class="page">
  <section class="hero hero-skins">
    <div class="hero-copy">
      <p class="eyebrow">Skin Explorer</p>
      <h2>Recherche skin facon NameMC, avec une DA unifiee.</h2>
      <p>
        Galerie rapide + acces detail joueur. Les cartes reprennent la meme DA que
        les pages server: memes boutons, memes surfaces, meme hierarchie.
      </p>
      <div class="hero-tags">
        <span class="status neutral">Head Render</span>
        <span class="status neutral">Face Render</span>
        <span class="status neutral">Profile Metadata</span>
      </div>
    </div>
    <div class="hero-media hero-media-skins">
      <div class="hero-overlay">
        <p>Player skins</p>
        <h3>Browse popular profiles</h3>
      </div>
    </div>
  </section>

  <section class="surface lookup-panel" style="margin-top: 1rem;">
    <div class="section-title">
      <div>
        <h3>Recherche joueur</h3>
        <p>Entre un pseudo pour ouvrir la page détail skin.</p>
      </div>
    </div>

    <form class="lookup-form" onsubmit={submitSearch}>
      <div class="input-row">
        <input class="input" bind:value={query} placeholder="Ex: Notch, Dream" required />
        <button class="btn btn-primary" type="submit">Voir le skin</button>
      </div>
    </form>
  </section>

  <section class="surface lookup-panel" style="margin-top: 1rem;">
    <div class="section-title">
      <div>
        <h3>Skins populaires</h3>
        <p>Cartes rapides pour explorer les profils.</p>
      </div>
    </div>

    <div class="gallery-grid">
      {#each featuredSkins as item}
        <SkinTile
          username={item.username}
          tag={item.tag}
          image={headUrl(item.username)}
          href={`/player/${encodeURIComponent(item.username)}`}
        />
      {/each}
    </div>
  </section>
</main>
