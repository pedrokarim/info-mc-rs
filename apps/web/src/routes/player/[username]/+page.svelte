<script lang="ts">
  import { goto } from '$app/navigation';
  import { browser } from '$app/environment';
  import SEO from '$lib/components/SEO.svelte';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import KeyValueGrid from '$lib/components/ui/KeyValueGrid.svelte';
  import NoticeBanner from '$lib/components/ui/NoticeBanner.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import CapePreview from '$lib/components/ui/CapePreview.svelte';
  import SkinViewer3D from '$lib/components/ui/SkinViewer3D.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  let lookupPlayer = $state('');

  $effect(() => {
    lookupPlayer = data.username;
  });

  function submitLookup(event: SubmitEvent) {
    event.preventDefault();
    const clean = lookupPlayer.trim();
    if (!clean) return;
    goto(`/player/${encodeURIComponent(clean)}`);
  }

  function renderUrl(type: 'full' | 'head' | 'face', size: number): string {
    return `${data.apiBase}/api/v1/render/${encodeURIComponent(data.username)}?type=${type}&size=${size}&overlay=true`;
  }

  const isSlim = $derived(data.player?.skin?.model === 'slim');

  let viewMode = $state<'3d' | '2d'>('3d');
  let animPaused = $state(false);
  let backEquipment = $state<'cape' | 'elytra' | 'none'>('cape');
  let isFavorite = $state(false);
  let favoriteLoading = $state(false);
  let isLiked = $state(false);
  let likeLoading = $state(false);
  let likeCount = $state(data.player?.popularity?.likes ?? 0);
  let viewCount = $state(data.player?.popularity?.views ?? 0);

  // Reset view state when player changes
  $effect(() => {
    data.username;
    viewMode = '3d';
    animPaused = false;
    backEquipment = 'cape';
  });

  // Check favorite + like status on load
  $effect(() => {
    if (!data.player?.uuid) return;
    fetch(`${data.apiBase}/api/v1/favorites/${data.player.uuid}`)
      .then(r => r.json())
      .then(d => { isFavorite = d.favorited === true; })
      .catch(() => { isFavorite = false; });
    fetch(`${data.apiBase}/api/v1/player/${data.player.uuid}/like`)
      .then(r => r.json())
      .then(d => { isLiked = d.liked === true; })
      .catch(() => { isLiked = false; });
    // Sync popularity counters from fresh data
    likeCount = data.player?.popularity?.likes ?? 0;
    viewCount = data.player?.popularity?.views ?? 0;
  });

  async function toggleFavorite() {
    if (!data.player?.uuid || favoriteLoading) return;
    favoriteLoading = true;
    const was = isFavorite;
    isFavorite = !was; // optimistic
    try {
      if (was) {
        await fetch(`${data.apiBase}/api/v1/favorites/${data.player.uuid}`, { method: 'DELETE' });
      } else {
        await fetch(`${data.apiBase}/api/v1/favorites/${data.player.uuid}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username: data.player.username }),
        });
      }
    } catch {
      isFavorite = was; // revert
    } finally {
      favoriteLoading = false;
    }
  }

  async function toggleLike() {
    if (!data.player?.uuid || likeLoading) return;
    likeLoading = true;
    const was = isLiked;
    isLiked = !was;
    likeCount += was ? -1 : 1;
    try {
      await fetch(`${data.apiBase}/api/v1/player/${data.player.uuid}/like`, {
        method: was ? 'DELETE' : 'POST',
      });
    } catch {
      isLiked = was;
      likeCount += was ? 1 : -1;
    } finally {
      likeLoading = false;
    }
  }

  function formatNumber(n: number): string {
    if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + 'M';
    if (n >= 1_000) return (n / 1_000).toFixed(1) + 'k';
    return n.toString();
  }

  const playerDetails = $derived(
    data.player
      ? [
          { key: 'Username', value: data.player.username },
          { key: 'UUID', value: data.player.uuid },
          { key: 'Model', value: data.player.skin?.model ?? 'unknown' },
          { key: 'Cape', value: [data.player.cape && 'Officielle', data.player.optifine_cape && 'OptiFine', data.player.labymod_cape && 'LabyMod'].filter(Boolean).join(', ') || 'Aucune' }
        ]
      : []
  );
</script>

<SEO
  title={`${data.username} — Skin Minecraft 3D, Cape & Détails`}
  description={`Voir le skin Minecraft de ${data.username} en 3D interactif. Télécharger le skin, voir les capes Mojang, OptiFine et l'elytra.`}
  canonical={`/player/${encodeURIComponent(data.username)}`}
  ogImage={`${data.apiBase}/api/v1/render3d/${encodeURIComponent(data.username)}?width=600&height=600&theta=30&phi=21`}
  jsonLd={{
    '@context': 'https://schema.org',
    '@type': 'ProfilePage',
    name: `${data.username} — Skin Minecraft`,
    description: `Skin 3D et détails du joueur Minecraft ${data.username}.`,
    url: `https://mcinfo.ascencia.re/player/${encodeURIComponent(data.username)}`,
    mainEntity: {
      '@type': 'Person',
      name: data.username,
      ...(data.player?.uuid ? { identifier: data.player.uuid } : {})
    }
  }}
  breadcrumbs={[
    { name: 'Accueil', href: '/' },
    { name: 'Skins', href: '/skins' },
    { name: data.username, href: `/player/${encodeURIComponent(data.username)}` },
  ]}
/>

<main class="page">
  <section class="hero hero-player">
    <div class="hero-copy">
      <p class="eyebrow">Player Detail</p>
      <h1>{data.username} — Skin Minecraft 3D</h1>
      <p>
        Viewer 3D interactif + rendu 2D depuis notre API Rust.
        Drag pour faire pivoter le skin.
      </p>
      <div class="hero-tags">
        <StatusPill label={data.player ? 'Skin found' : 'Not found'} kind={data.player ? 'online' : 'offline'} />
        <StatusPill label={isSlim ? 'Slim / Alex' : 'Classic / Steve'} kind="neutral" />
        <GameChip label="Retour galerie" href="/skins" />
      </div>
    </div>
    <div class="hero-media hero-media-player">
      <div class="hero-overlay">
        <p>Username lookup</p>
        <h3>{data.player?.username ?? data.username}</h3>
      </div>
    </div>
  </section>

  <section class="surface lookup-panel">
    <SectionHeading
      title="Skin Detail"
      description="Recherche type NameMC, viewer 3D interactif."
      light={true}
    >
      <GameChip slot="right" label="Retour galerie" href="/skins" />
    </SectionHeading>

    <form class="lookup-form" onsubmit={submitLookup}>
      <SearchInputRow bind:value={lookupPlayer} placeholder="Notch, Dream..." actionLabel="Rechercher" />
    </form>

    {#if data.error}
      <div class="not-found-block">
        <img class="not-found-img" src="/images/ui/player-not-found-v01.png" alt="Joueur introuvable" />
        <div class="not-found-text">
          <h3>Joueur introuvable</h3>
          <p>{data.error}</p>
        </div>
      </div>
    {/if}
  </section>

  {#if data.player}
    <section class="player-layout section-strip">

      <!-- Viewer column -->
      <div class="viewer-col">
        {#if data.player.skin?.url}
          <div class="viewer-toolbar">
            <!-- Play / Pause -->
            <button
              class="toolbar-btn"
              title={animPaused ? 'Reprendre animation' : 'Pause animation'}
              onclick={() => (animPaused = !animPaused)}
            >
              {#if animPaused}
                <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4 2l10 6-10 6z"/></svg>
              {:else}
                <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="3" y="2" width="3.5" height="12"/><rect x="9.5" y="2" width="3.5" height="12"/></svg>
              {/if}
            </button>

            <!-- 3D / 2D toggle -->
            <div class="view-toggle" role="group" aria-label="Mode viewer">
              <button class="view-btn" class:active={viewMode === '3d'} onclick={() => (viewMode = '3d')}>3D</button>
              <button class="view-btn" class:active={viewMode === '2d'} onclick={() => (viewMode = '2d')}>2D</button>
            </div>

            <!-- Download skin -->
            <a class="toolbar-btn" href={data.player.skin.url} download title="Télécharger skin PNG">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1v9m0 0l-3-3m3 3l3-3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M2 12v2h12v-2" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>
            </a>

            <!-- Back equipment toggle (Cape / Elytra) -->
            {#if data.player.cape?.url || data.player.optifine_cape?.url}
              <div class="view-toggle" role="group" aria-label="Back equipment">
                <button class="view-btn" class:active={backEquipment === 'cape'} onclick={() => (backEquipment = 'cape')}>Cape</button>
                <button class="view-btn" class:active={backEquipment === 'elytra'} onclick={() => (backEquipment = 'elytra')}>Elytra</button>
              </div>
            {/if}

            <!-- Like -->
            <button
              class="toolbar-btn toolbar-btn--like"
              class:is-liked={isLiked}
              title={isLiked ? 'Retirer le like' : 'Liker'}
              onclick={toggleLike}
              disabled={likeLoading}
            >
              <svg width="16" height="16" viewBox="0 0 16 16" fill={isLiked ? '#5e90ff' : 'none'} stroke={isLiked ? '#5e90ff' : 'currentColor'} stroke-width="1.2">
                <path d="M4.7 14H2.5a1 1 0 01-1-1V8a1 1 0 011-1h2.2m0 7V7m0 7H11a1.5 1.5 0 001.45-1.12l1.1-4A1.5 1.5 0 0012.1 7H9.5V3.5A1.5 1.5 0 008 2L4.7 7"/>
              </svg>
              <span class="toolbar-count">{formatNumber(likeCount)}</span>
            </button>

            <!-- Favorite -->
            <button
              class="toolbar-btn toolbar-btn--fav"
              class:is-fav={isFavorite}
              title={isFavorite ? 'Retirer des favoris' : 'Ajouter aux favoris'}
              onclick={toggleFavorite}
              disabled={favoriteLoading}
            >
              <svg width="16" height="16" viewBox="0 0 16 16">
                <path d="M8 14s-5.5-3.5-5.5-7.5C2.5 4 4.5 2 6.5 2c1.2 0 2.3.8 3 1.5C10.2 2.8 11.3 2 12.5 2c2 0 3 2 3 4.5S8 14 8 14z"
                  fill={isFavorite ? '#e74c5e' : 'none'}
                  stroke={isFavorite ? '#e74c5e' : 'currentColor'}
                  stroke-width="1.2"
                />
              </svg>
            </button>
          </div>
        {/if}

        {#if viewMode === '3d' && browser && data.player.skin?.url}
          {#key `${data.username}-${data.player.skin.url}`}
            <SkinViewer3D
              skinUrl={data.player.skin.url}
              capeUrl={data.player.cape?.url ?? (data.player.optifine_cape?.url ? `${data.apiBase}${data.player.optifine_cape.url}` : undefined)}
              slim={isSlim}
              paused={animPaused}
              {backEquipment}
              width={240}
              height={360}
            />
          {/key}
        {:else if viewMode === '2d' && data.player.skin?.url}
          <img
            class="skin-preview head-2d"
            src={renderUrl('head', 256)}
            alt={`Tête 2D ${data.player.username}`}
          />
        {:else}
          <img
            class="skin-preview"
            src="/images/skins/skin-placeholder-full-v01.png"
            alt="Skin non disponible"
          />
        {/if}
      </div>

      <!-- Details -->
      <div class="details-col">
        <div class="card">
          <div class="card-head">
            <h3>{data.player.username}</h3>
            <StatusPill label="Online" kind="online" />
          </div>
          <div class="card-body">
            <KeyValueGrid items={playerDetails} />
          </div>
        </div>

        {#if data.player.popularity}
          <div class="popularity-strip">
            <div class="pop-stat">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4"><circle cx="8" cy="8" r="3"/><path d="M1 8s3-5.5 7-5.5S15 8 15 8s-3 5.5-7 5.5S1 8 1 8z"/></svg>
              <span class="pop-value">{formatNumber(viewCount)}</span>
              <span class="pop-label">vues</span>
            </div>
            <div class="pop-stat">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M2 8.5l4 4.5 8-9"/></svg>
              <span class="pop-value">{formatNumber(likeCount)}</span>
              <span class="pop-label">likes</span>
            </div>
            <div class="pop-stat">
              <span class="pop-label">Vu depuis {new Date(data.player.popularity.first_seen_at).toLocaleDateString('fr-FR')}</span>
            </div>
          </div>
        {/if}

        <div class="card" style="margin-top: 1rem;">
          <div class="card-head"><h4>Aperçu 2D</h4></div>
          <div class="card-body renders-row">
            <div class="render-tile">
              <p class="render-label">Visage</p>
              <img
                class="skin-preview head"
                src={renderUrl('face', 128)}
                alt={`Visage ${data.player.username}`}
              />
            </div>
          </div>
        </div>

        <div class="quick-actions" style="margin-top: 0.9rem;">
          {#if data.player.skin?.url}
            <GameChip label="Skin PNG" href={data.player.skin.url} target="_blank" rel="noreferrer" />
          {/if}
          <GameChip label="Voir un serveur" href="/server/play.hypixel.net" />
        </div>

        <!-- Cape section -->
        {#if data.player.cape?.url || data.player.optifine_cape?.url}
          <div class="card" style="margin-top: 1rem;">
            <div class="card-head"><h4>Capes</h4></div>
            <div class="card-body capes-row">
              {#if data.player.cape?.url}
                <div class="cape-tile">
                  <Badge label="Officielle Mojang" variant="success" size="sm" />
                  <CapePreview url={data.player.cape.url} scale={8} />
                  <a class="cape-link" href={data.player.cape.url} target="_blank" rel="noreferrer">PNG ↗</a>
                </div>
              {/if}
              {#if data.player.optifine_cape?.url}
                <div class="cape-tile">
                  <Badge label={`OptiFine${data.player.optifine_cape.active === false ? ' (inactive)' : ''}`} variant="info" size="sm" />
                  <CapePreview url={`${data.apiBase}${data.player.optifine_cape.url}`} scale={8} />
                  <a class="cape-link" href={`${data.apiBase}${data.player.optifine_cape.url}`} target="_blank" rel="noreferrer">PNG ↗</a>
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>

    </section>
  {/if}
</main>

<style>
  .player-layout {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 1.8rem;
    align-items: start;
  }
  .viewer-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.6rem;
  }
  .details-col {
    min-width: 0;
  }
  .renders-row {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  .render-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
  }
  .render-label {
    margin: 0;
    color: var(--ink-2);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
  /* Cape section */
  .capes-row {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
    align-items: flex-start;
  }
  .cape-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }
  .cape-link {
    font-size: 0.75rem;
    color: var(--ink-2);
    text-decoration: none;
  }
  .cape-link:hover { text-decoration: underline; }

  /* Viewer toolbar */
  .viewer-toolbar {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    margin-bottom: 0.6rem;
  }
  .toolbar-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: 1px solid rgba(76, 120, 176, 0.38);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.72);
    backdrop-filter: blur(6px);
    color: var(--ink-1);
    cursor: pointer;
    transition: background 120ms, color 120ms, transform 100ms;
    text-decoration: none;
    padding: 0;
  }
  .toolbar-btn:hover {
    background: rgba(255, 255, 255, 0.92);
  }
  .toolbar-btn:active {
    transform: scale(0.93);
  }
  .toolbar-btn--fav.is-fav {
    border-color: rgba(231, 76, 94, 0.4);
    background: rgba(231, 76, 94, 0.08);
  }

  /* View toggle */
  .view-toggle {
    display: inline-flex;
    border-radius: 8px;
    border: 1px solid rgba(76, 120, 176, 0.38);
    background: rgba(255, 255, 255, 0.72);
    padding: 0.18rem;
    gap: 0.18rem;
  }
  .view-btn {
    font: inherit;
    cursor: pointer;
    border: none;
    background: transparent;
    border-radius: 5px;
    padding: 0.28rem 0.76rem;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    color: var(--ink-1);
    transition: background 120ms, color 120ms;
  }
  .view-btn.active {
    color: #fff;
    background: linear-gradient(180deg, var(--blue-0), var(--blue-1));
    box-shadow: 0 1px 0 rgba(255,255,255,0.2) inset;
  }
  /* Like button */
  .toolbar-btn--like {
    width: auto;
    gap: 0.3rem;
    padding: 0 0.5rem;
  }
  .toolbar-btn--like.is-liked {
    border-color: rgba(94, 144, 255, 0.4);
    background: rgba(94, 144, 255, 0.08);
    color: #5e90ff;
  }
  .toolbar-count {
    font-size: 0.7rem;
    font-weight: 700;
  }

  /* Popularity strip */
  .popularity-strip {
    display: flex;
    align-items: center;
    gap: 1.2rem;
    margin-top: 0.8rem;
    padding: 0.6rem 0.9rem;
    border-radius: 10px;
    background: rgba(94, 144, 255, 0.06);
    border: 1px solid rgba(94, 144, 255, 0.15);
  }
  .pop-stat {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    color: var(--ink-2);
    font-size: 0.8rem;
  }
  .pop-value {
    font-weight: 700;
    color: var(--ink-1);
  }
  .pop-label {
    color: var(--ink-2);
    font-size: 0.75rem;
  }

  .head-2d {
    width: 200px;
    height: 200px;
    border-radius: 12px;
  }

  @media (max-width: 760px) {
    .player-layout {
      grid-template-columns: 1fr;
    }
    .viewer-col {
      align-items: center;
    }
    .popularity-strip {
      flex-wrap: wrap;
      gap: 0.6rem;
    }
  }
</style>
