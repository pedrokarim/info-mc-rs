<script lang="ts">
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import type { ServerEdition } from '$lib/types';
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

<main class="page">
  <section class="hero hero-server">
    <div class="hero-copy">
      <p class="eyebrow">Server Intel</p>
      <h2>{data.address}</h2>
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
