<script lang="ts">
  import { goto } from '$app/navigation';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import KeyValueGrid from '$lib/components/ui/KeyValueGrid.svelte';
  import NoticeBanner from '$lib/components/ui/NoticeBanner.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
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

  const playerDetails = $derived(
    data.player
      ? [
          { key: 'Username', value: data.player.username },
          { key: 'UUID', value: data.player.uuid },
          { key: 'Model', value: data.player.skin?.model ?? 'unknown' },
          { key: 'Cape', value: data.player.cape ? 'Yes' : 'No' },
          { key: 'Skin URL', value: data.player.skin?.url ?? 'N/A' }
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
        Page detaillee type NameMC: rendus skin head/face/full, metadonnees,
        liens utiles et navigation rapide vers un autre pseudo.
      </p>
      <div class="hero-tags">
        <StatusPill label="Skin Profile" kind="neutral" />
        <StatusPill label="Render API" kind="neutral" />
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
      description="Recherche type NameMC, détails centrés sur le skin."
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
    <section class="grid-2" style="margin-top: 1rem;">
      <article class="surface card">
        <div class="card-head">
          <h3>{data.player.username}</h3>
          <StatusPill label="Skin found" kind="online" />
        </div>
        <div class="card-body">
          <div class="player-wrap">
            <KeyValueGrid items={playerDetails} />

            <img class="skin-preview" src={renderUrl('full', 320)} alt={`Skin full ${data.player.username}`} />
          </div>
        </div>
      </article>

      <article class="surface card">
        <div class="card-head">
          <h3>Aperçus</h3>
        </div>
        <div class="card-body">
          <div class="meta-grid">
            <div class="meta-tile">
              <p class="label">Head 2D</p>
              <img
                class="skin-preview head"
                src={renderUrl('head', 128)}
                alt={`Head ${data.player.username}`}
              />
            </div>
            <div class="meta-tile">
              <p class="label">Face</p>
              <img
                class="skin-preview head"
                src={renderUrl('face', 128)}
                alt={`Face ${data.player.username}`}
              />
            </div>
            <div class="meta-tile">
              <p class="label">Actions</p>
              <div class="quick-actions" style="margin-top: 0.5rem;">
                {#if data.player.skin?.url}
                  <GameChip label="Skin PNG" href={data.player.skin.url} target="_blank" rel="noreferrer" />
                {/if}
                <GameChip label="Tester server" href={`/server/${encodeURIComponent('play.hypixel.net')}`} />
              </div>
            </div>
          </div>
        </div>
      </article>
    </section>
  {/if}
</main>
