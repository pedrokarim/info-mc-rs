<script lang="ts">
  import { goto } from '$app/navigation';
  import SEO from '$lib/components/SEO.svelte';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import HeroBleed from '$lib/components/ui/HeroBleed.svelte';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import SkinTile from '$lib/components/ui/SkinTile.svelte';
  import TogglePills from '$lib/components/ui/TogglePills.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const apiBase = data.apiBase;

  let mode = $state<'server' | 'player'>('server');
  let identifier = $state('');
  const lookupModeOptions = [
    { label: 'Server Lookup', value: 'server' },
    { label: 'Player / Skin Lookup', value: 'player' }
  ];

  function submitLookup(event: SubmitEvent) {
    event.preventDefault();
    const clean = identifier.trim();
    if (!clean) return;

    if (mode === 'server') {
      goto(`/server/${encodeURIComponent(clean)}`);
      return;
    }

    goto(`/player/${encodeURIComponent(clean)}`);
  }

  const fallbackSkins = ['Notch', 'Dream', 'Technoblade', 'Skeppy', 'sweetily', 'Mxodz', 'saphyras', 'cakeycat'];
  const popularNames = new Set(data.popularPlayers.map(p => p.username.toLowerCase()));
  const fillSkins = fallbackSkins.filter(u => !popularNames.has(u.toLowerCase()));

  const displaySkins = [
    ...data.popularPlayers.map(p => ({ username: p.username, tag: `${p.views} vues` })),
    ...fillSkins.map(u => ({ username: u, tag: '' })),
  ].slice(0, 8);

  const fallbackServers = [
    { name: 'Hypixel', address: 'play.hypixel.net', edition: 'Java' },
    { name: 'CubeCraft', address: 'play.cubecraft.net', edition: 'Java + Bedrock' },
    { name: 'Wynncraft', address: 'play.wynncraft.com', edition: 'Java' },
    { name: 'Epicube', address: 'play.epicube.fr', edition: 'Java' },
    { name: 'FunCraft', address: 'play.funcraft.net', edition: 'Java' },
    { name: '2b2t', address: '2b2t.org', edition: 'Java' },
  ];

  // Merge popular servers from API with fallbacks
  const popularAddrs = new Set(data.popularServers.map(s => s.address.toLowerCase()));
  const displayServers = [
    ...data.popularServers.slice(0, 6).map(s => ({
      name: s.hostname || s.address,
      address: s.address,
      edition: s.edition || 'Java',
      motd: data.motdMap[s.address.toLowerCase()] ?? s.motd_html ?? '',
      stats: `${s.views} vues · ${s.likes} likes`,
    })),
    ...fallbackServers
      .filter(s => !popularAddrs.has(s.address.toLowerCase()))
      .map(s => ({
        name: s.name,
        address: s.address,
        edition: s.edition,
        motd: data.motdMap[s.address.toLowerCase()] ?? '',
        stats: '',
      })),
  ].slice(0, 6);

  function render3dUrl(username: string): string {
    return `${apiBase}/api/v1/render3d/${encodeURIComponent(username)}?width=256&height=256&theta=30&phi=21`;
  }
</script>

<SEO
  title="Statut serveur Minecraft, Skin Viewer 3D & API"
  description="MCInfo : vérifiez le statut de n'importe quel serveur Minecraft, explorez les skins en 3D, rendus de capes et elytra. API ouverte et gratuite pour les développeurs."
  canonical="/"
  jsonLd={{
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    name: 'MCInfo',
    url: 'https://mcinfo.ascencia.re',
    description: 'Statut serveur Minecraft, skin viewer 3D et API ouverte.',
    potentialAction: {
      '@type': 'SearchAction',
      target: 'https://mcinfo.ascencia.re/player/{search_term_string}',
      'query-input': 'required name=search_term_string'
    }
  }}
/>

<HeroBleed
  eyebrow="Minecraft Intelligence"
  title="Serveurs, joueurs, skins. Tout en un."
  description="Statut en temps réel, MOTD visuel, statistiques de joueurs et explorer de skins Minecraft."
  image="/images/hero/hero-main-v01.png"
  primaryLabel="Voir un serveur"
  primaryHref="/server/play.hypixel.net"
  secondaryLabel="Explorer les skins"
  secondaryHref="/skins"
/>

<main class="page">
  <section class="lookup-panel section-strip">
    <SectionHeading
      title="Lookup rapide"
      description="Entre une adresse de serveur ou un pseudo joueur pour ouvrir la fiche détaillée."
    />

    <form class="lookup-form" onsubmit={submitLookup}>
      <TogglePills options={lookupModeOptions} bind:value={mode} name="lookup-mode-home" ariaLabel="Mode lookup" />
      <SearchInputRow
        bind:value={identifier}
        placeholder={mode === 'server'
          ? 'play.hypixel.net ou 172.65.128.35:25565'
          : 'Notch, Dream ou UUID'}
        actionLabel="Analyser"
      />
    </form>
  </section>

  <!-- Skins populaires -->
  <section class="section-strip">
    <SectionHeading title="Skins populaires" description="Les skins les plus recherchés — rendus 3D en temps réel.">
      <GameChip slot="right" label="Voir tout →" href="/skins" />
    </SectionHeading>

    <div class="home-skin-grid">
      {#each displaySkins as skin, i}
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

  <!-- Serveurs populaires -->
  <section class="section-strip">
    <SectionHeading title="Serveurs populaires" description="Statut en direct avec MOTD visuel et couleurs Minecraft.">
      <GameChip slot="right" label="Voir tout →" href="/servers" />
    </SectionHeading>

    <div class="home-server-grid">
      {#each displayServers as srv}
        <a class="home-srv-card" href={`/server/${encodeURIComponent(srv.address)}`}>
          <div class="home-srv-head">
            <p class="home-srv-name">{srv.name}</p>
            <span class="home-srv-edition">{srv.edition}</span>
          </div>
          <p class="home-srv-address">{srv.address}</p>
          {#if srv.motd}
            <div class="home-srv-motd">
              <MotdBlock html={srv.motd} />
            </div>
          {/if}
          {#if srv.stats}
            <p class="home-srv-stats">{srv.stats}</p>
          {/if}
          <span class="home-srv-cta">Voir le statut →</span>
        </a>
      {/each}
    </div>
  </section>
</main>

<style>
  /* ── Skins grid ── */
  .home-skin-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 0.74rem;
  }

  :global(.home-skin-grid .skin-card img) {
    aspect-ratio: 1 / 1;
    background: linear-gradient(180deg, #e8f0f7 0%, #d4e4f0 100%);
  }

  @media (max-width: 980px) {
    .home-skin-grid {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }
  }

  @media (max-width: 600px) {
    .home-skin-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  /* ── Servers grid ── */
  .home-server-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.74rem;
  }

  .home-srv-card {
    text-decoration: none;
    display: flex;
    flex-direction: column;
    gap: 0.26rem;
    border-radius: 10px;
    border: 1px solid rgba(84, 126, 181, 0.22);
    background: rgba(255, 255, 255, 0.2);
    padding: 0.82rem 0.88rem;
    transition: transform 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
  }

  .home-srv-card:hover {
    transform: translateY(-2px);
    border-color: rgba(84, 139, 206, 0.72);
    box-shadow: 0 10px 20px rgba(21, 61, 96, 0.15);
    background: rgba(255, 255, 255, 0.34);
  }

  .home-srv-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .home-srv-name {
    margin: 0;
    color: #0e3555;
    font-family: 'Teko', 'Chakra Petch', sans-serif;
    font-size: 1.45rem;
    font-weight: 600;
    line-height: 0.95;
  }

  .home-srv-edition {
    flex-shrink: 0;
    font-size: 0.65rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: #1e4b71;
    background: rgba(204, 224, 241, 0.95);
    border: 1px solid rgba(70, 113, 166, 0.3);
    border-radius: 999px;
    padding: 0.1em 0.5em;
  }

  .home-srv-address {
    margin: 0;
    color: var(--ink-2);
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.76rem;
  }

  .home-srv-motd {
    flex: 1;
    margin-top: 0.2rem;
  }

  .home-srv-stats {
    margin: 0;
    font-size: 0.72rem;
    color: var(--ink-2);
    font-weight: 600;
  }

  .home-srv-cta {
    margin-top: 0.3rem;
    font-size: 0.74rem;
    font-weight: 700;
    color: #1a5080;
    letter-spacing: 0.03em;
  }

  .home-srv-card:hover .home-srv-cta {
    color: #0e3a62;
  }

  @media (max-width: 980px) {
    .home-server-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 600px) {
    .home-server-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
