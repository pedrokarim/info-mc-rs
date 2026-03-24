<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import type { ServerEdition } from '$lib/types';
  import SEO from '$lib/components/SEO.svelte';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import KeyValueGrid from '$lib/components/ui/KeyValueGrid.svelte';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';
  import NoticeBanner from '$lib/components/ui/NoticeBanner.svelte';
  import ProgressMeter from '$lib/components/ui/ProgressMeter.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';

  let { data }: { data: PageData } = $props();

  let lookupAddress = $state('');
  let edition = $state<ServerEdition>('auto');

  $effect(() => {
    lookupAddress = data.address;
    edition = data.edition;
  });

  const editionLinks = [
    { label: 'Auto', value: 'auto' },
    { label: 'Java', value: 'java' },
    { label: 'Bedrock', value: 'bedrock' }
  ] as const;

  function submitLookup(event: SubmitEvent) {
    event.preventDefault();
    const clean = lookupAddress.trim();
    if (!clean) return;

    goto(`/server/${encodeURIComponent(clean)}?type=${edition}`);
  }

  function statusKind() {
    if (!data.server) return 'offline';
    return data.server.online ? 'online' : 'offline';
  }

  function populationPercent(): number {
    const players = data.server?.players;
    if (!players || players.max <= 0) return 0;
    return Math.max(0, Math.min(100, (players.online / players.max) * 100));
  }

  let isLiked = $state(false);
  let likeLoading = $state(false);
  let likeCount = $state(data.server?.popularity?.likes ?? 0);
  let viewCount = $state(data.server?.popularity?.views ?? 0);

  $effect(() => {
    if (!data.server || !data.server.online) return;
    const addr = data.server.address.hostname;
    fetch(`${data.apiBase}/api/v1/server/${encodeURIComponent(addr)}/like`)
      .then(r => r.json())
      .then(d => { isLiked = d.liked === true; })
      .catch(() => { isLiked = false; });
    likeCount = data.server?.popularity?.likes ?? 0;
    viewCount = data.server?.popularity?.views ?? 0;
  });

  async function toggleLike() {
    if (!data.server?.online || likeLoading) return;
    const addr = data.server.address.hostname;
    likeLoading = true;
    const was = isLiked;
    isLiked = !was;
    likeCount += was ? -1 : 1;
    try {
      await fetch(`${data.apiBase}/api/v1/server/${encodeURIComponent(addr)}/like`, {
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

  const playerCount = $derived(data.server?.players ? `${data.server.players.online}/${data.server.players.max} joueurs` : '');
  const versionName = $derived(data.server?.version?.name ?? '');

  const serverDetails = $derived(data.server
    ? [
        { key: 'Hostname', value: data.server.address.hostname },
        { key: 'IP', value: data.server.address.ip },
        { key: 'Port', value: data.server.address.port },
        { key: 'Edition', value: data.server.edition },
        { key: 'Version', value: data.server.version?.name ?? 'N/A' },
        { key: 'Protocol', value: data.server.version?.protocol ?? 'N/A' },
        { key: 'Latency', value: `${data.server.latency_ms ?? 'N/A'} ms` },
        { key: 'SRV', value: data.server.address.srv_record ? 'Yes' : 'No' }
      ]
    : []);
</script>

<SEO
  title={`${data.address} — Statut Serveur Minecraft`}
  description={`Statut en temps réel du serveur Minecraft ${data.address}. ${data.server?.online ? `En ligne — ${playerCount}${versionName ? ` — ${versionName}` : ''}` : 'Hors ligne'}. MOTD visuel, latence et détails.`}
  canonical={`/server/${encodeURIComponent(data.address)}`}
  jsonLd={{
    '@context': 'https://schema.org',
    '@type': 'WebApplication',
    name: `${data.address} — Statut Serveur`,
    url: `https://mcinfo.ascencia.re/server/${encodeURIComponent(data.address)}`,
    applicationCategory: 'GameApplication',
    operatingSystem: 'Minecraft Java / Bedrock'
  }}
  breadcrumbs={[
    { name: 'Accueil', href: '/' },
    { name: 'Serveurs', href: '/servers' },
    { name: data.address, href: `/server/${encodeURIComponent(data.address)}` },
  ]}
/>

<main class="page">
  <section class="hero hero-server">
    <div class="hero-copy">
      <p class="eyebrow">Server Intel</p>
      <h1>{data.address} — Statut Serveur Minecraft</h1>
      <p>
        Snapshot live de ton serveur Minecraft: statut, version, joueurs, latence
        et MOTD visuel dans la meme DA que le reste du site.
      </p>
      <div class="hero-tags">
        <StatusPill label={data.server?.online ? 'Online' : 'Offline'} kind={statusKind()} />
        <StatusPill label={`Mode ${data.edition}`} kind="neutral" />
        {#if data.server?.players}
          <StatusPill label={`${data.server.players.online}/${data.server.players.max} joueurs`} kind="neutral" />
        {/if}
      </div>
    </div>
    <div class="hero-media hero-media-server">
      <div class="hero-overlay">
        <p>Server snapshot</p>
        <h3>{data.server?.version?.name ?? 'No version data'}</h3>
      </div>
    </div>
  </section>

  <section class="surface lookup-panel">
    <SectionHeading title="Server Snapshot" description={`Adresse: ${data.address}`}>
      <StatusPill
        slot="right"
        label={data.server?.online ? 'Online' : 'Offline'}
        kind={statusKind()}
      />
    </SectionHeading>

    <form class="lookup-form" onsubmit={submitLookup}>
      <SearchInputRow bind:value={lookupAddress} placeholder="play.hypixel.net" actionLabel="Rechercher" />

      <div class="quick-actions">
        {#each editionLinks as item}
          <GameChip
            label={item.label}
            active={edition === item.value}
            onClick={() => {
              edition = item.value;
            }}
          />
        {/each}
      </div>
    </form>

    {#if data.error}
      <div style="margin-top: 0.8rem;">
        <NoticeBanner message={data.error} />
      </div>
    {/if}
  </section>

  {#if data.server}
    <section class="grid-2" style="margin-top: 1rem;">
      <article class="surface card">
        <div class="card-head">
          <h4>Détails</h4>
        </div>
        <div class="card-body">
          <KeyValueGrid items={serverDetails} />

          {#if data.server.players}
            <p style="margin: 0.8rem 0 0;">
              {data.server.players.online} / {data.server.players.max} joueurs
            </p>
            <ProgressMeter value={populationPercent()} max={100} />
          {/if}

          {#if data.server.popularity}
            <div class="popularity-strip" style="margin-top: 0.8rem; display: flex; align-items: center; gap: 1rem; padding: 0.5rem 0.7rem; border-radius: 8px; background: rgba(94,144,255,0.06); border: 1px solid rgba(94,144,255,0.15); font-size: 0.8rem;">
              <span style="display:flex;align-items:center;gap:0.3rem;color:var(--ink-2)">
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4"><circle cx="8" cy="8" r="3"/><path d="M1 8s3-5.5 7-5.5S15 8 15 8s-3 5.5-7 5.5S1 8 1 8z"/></svg>
                <strong style="color:var(--ink-1)">{formatNumber(viewCount)}</strong> vues
              </span>
              <span style="display:flex;align-items:center;gap:0.3rem;color:var(--ink-2)">
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M2 8.5l4 4.5 8-9"/></svg>
                <strong style="color:var(--ink-1)">{formatNumber(likeCount)}</strong> likes
              </span>
              <button
                onclick={toggleLike}
                disabled={likeLoading}
                style="margin-left:auto; cursor:pointer; border:1px solid {isLiked ? 'rgba(94,144,255,0.4)' : 'rgba(76,120,176,0.3)'}; background:{isLiked ? 'rgba(94,144,255,0.1)' : 'rgba(255,255,255,0.7)'}; color:{isLiked ? '#5e90ff' : 'var(--ink-1)'}; border-radius:6px; padding:0.25rem 0.6rem; font-size:0.75rem; font-weight:700; display:flex; align-items:center; gap:0.3rem;"
              >
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 8.5l4 4.5 8-9"/></svg>
                {isLiked ? 'Liké' : 'Like'}
              </button>
            </div>
          {/if}
        </div>
      </article>

      <article class="surface card">
        <div class="card-head">
          <h4>MOTD Visual</h4>
        </div>
        <div class="card-body">
          <MotdBlock html={data.server.motd?.html ?? ''} fallback="No MOTD" />

          {#if data.server.players?.sample && data.server.players.sample.length > 0}
            <p class="empty" style="margin-bottom: 0.4rem;">Sample joueurs</p>
            <div class="quick-actions">
              {#each data.server.players.sample.slice(0, 8) as player}
                <GameChip label={player.name} href={`/player/${encodeURIComponent(player.name)}`} />
              {/each}
            </div>
          {/if}
        </div>
      </article>
    </section>
  {/if}
</main>
