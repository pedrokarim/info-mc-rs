<script lang="ts">
  import { goto } from '$app/navigation';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import KeyValueGrid from '$lib/components/ui/KeyValueGrid.svelte';
  import NoticeBanner from '$lib/components/ui/NoticeBanner.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import CapePreview from '$lib/components/ui/CapePreview.svelte';
  import SkinViewer3D from '$lib/components/ui/SkinViewer3D.svelte';
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

  const playerDetails = $derived(
    data.player
      ? [
          { key: 'Username', value: data.player.username },
          { key: 'UUID', value: data.player.uuid },
          { key: 'Model', value: data.player.skin?.model ?? 'unknown' },
          { key: 'Cape', value: data.player.cape ? 'Officielle' : (data.player.optifine_cape ? 'OptiFine' : 'Aucune') }
        ]
      : []
  );
</script>

<main class="page">
  <section class="hero hero-player">
    <div class="hero-copy">
      <p class="eyebrow">Player Detail</p>
      <h2>{data.username}</h2>
      <p>
        Viewer 3D interactif + rendus 2D head/face depuis notre API Rust.
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
      <div style="margin-top: 0.8rem;">
        <NoticeBanner message={data.error} />
      </div>
    {/if}
  </section>

  {#if data.player}
    <section class="player-layout section-strip">

      <!-- 3D viewer -->
      <div class="viewer-col">
        {#if data.player.skin?.url}
          <SkinViewer3D
            skinUrl={data.player.skin.url}
            capeUrl={data.player.cape?.url}
            slim={isSlim}
            width={240}
            height={360}
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

        <div class="card" style="margin-top: 1rem;">
          <div class="card-head"><h4>Aperçus 2D</h4></div>
          <div class="card-body renders-row">
            <div class="render-tile">
              <p class="render-label">Head</p>
              <img
                class="skin-preview head"
                src={renderUrl('head', 128)}
                alt={`Head ${data.player.username}`}
              />
            </div>
            <div class="render-tile">
              <p class="render-label">Face</p>
              <img
                class="skin-preview head"
                src={renderUrl('face', 128)}
                alt={`Face ${data.player.username}`}
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
                  <div class="cape-badge cape-badge--mojang">Officielle Mojang</div>
                  <CapePreview url={data.player.cape.url} scale={8} />
                  <a class="cape-link" href={data.player.cape.url} target="_blank" rel="noreferrer">PNG ↗</a>
                </div>
              {/if}
              {#if data.player.optifine_cape?.url}
                <div class="cape-tile">
                  <div class="cape-badge cape-badge--optifine">OptiFine</div>
                  <CapePreview url={data.player.optifine_cape.url} scale={8} />
                  <a class="cape-link" href={data.player.optifine_cape.url} target="_blank" rel="noreferrer">PNG ↗</a>
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
  .cape-badge {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    padding: 0.2em 0.6em;
    border-radius: 4px;
  }
  .cape-badge--mojang {
    background: #1a6b3c;
    color: #7fffc2;
  }
  .cape-badge--optifine {
    background: #3b2a6b;
    color: #b89fff;
  }
  .cape-link {
    font-size: 0.75rem;
    color: var(--ink-2);
    text-decoration: none;
  }
  .cape-link:hover { text-decoration: underline; }

  @media (max-width: 760px) {
    .player-layout {
      grid-template-columns: 1fr;
    }
    .viewer-col {
      align-items: center;
    }
  }
</style>
