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
    return `${apiBase}/api/v1/render3d/${encodeURIComponent(username)}?width=256&height=256&theta=30&phi=21&back=none`;
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

  <!-- Tools CTA -->
  <section class="tools-cta">
    <div class="tools-cta-icons" aria-hidden="true">
      <img src="/images/ui/tool-skin-editor-v01.png" alt="" width="72" height="72" loading="lazy" />
      <img src="/images/ui/tool-motd-editor-v01.png" alt="" width="72" height="72" loading="lazy" />
      <img src="/images/ui/tool-banner-designer-v01.png" alt="" width="72" height="72" loading="lazy" />
      <img src="/images/ui/tool-seed-map-v01.png" alt="" width="72" height="72" loading="lazy" />
      <img src="/images/ui/tool-enchantment-calculator-v01.png" alt="" width="72" height="72" loading="lazy" />
      <img src="/images/ui/tool-command-generator-v01.png" alt="" width="72" height="72" loading="lazy" />
    </div>
    <h3 class="tools-cta-title">12 outils gratuits pour Minecraft</h3>
    <p class="tools-cta-desc">Éditeurs de skin, MOTD, bannières, calculateurs d'enchantements, seed map et plus encore.</p>
    <a class="tools-cta-btn" href="/tools">
      Explorer les outils
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5l7 7-7 7" /></svg>
    </a>
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

  /* ── Tools CTA ── */
  .tools-cta {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 2.5rem 1.5rem;
    margin-top: 1rem;
    border-radius: 16px;
    border: 1px solid rgba(84, 126, 181, 0.18);
    background:
      radial-gradient(ellipse 80% 60% at 50% 120%, rgba(88, 166, 255, 0.10) 0%, transparent 70%),
      rgba(237, 245, 250, 0.6);
  }

  .tools-cta-icons {
    display: flex;
    gap: 10px;
    margin-bottom: 1.2rem;
  }

  .tools-cta-icons img {
    width: 56px;
    height: 56px;
    border-radius: 12px;
    border: 2px solid rgba(84, 126, 181, 0.2);
    background: rgba(255, 255, 255, 0.5);
    object-fit: contain;
    transition: transform 200ms ease;
  }

  .tools-cta:hover .tools-cta-icons img:nth-child(odd) {
    transform: translateY(-3px);
  }

  .tools-cta:hover .tools-cta-icons img:nth-child(even) {
    transform: translateY(3px);
  }

  .tools-cta-title {
    margin: 0;
    font-family: 'Teko', sans-serif;
    font-size: 1.7rem;
    font-weight: 600;
    color: var(--ink-0, #0f253a);
    line-height: 1.15;
  }

  .tools-cta-desc {
    margin: 0.35rem 0 0;
    font-size: 0.88rem;
    color: var(--ink-2, #5a7894);
    max-width: 440px;
    line-height: 1.45;
  }

  .tools-cta-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-top: 1.2rem;
    padding: 0.6rem 1.4rem;
    font-family: inherit;
    font-size: 0.9rem;
    font-weight: 700;
    color: #fff;
    background: linear-gradient(180deg, #4a90d9 0%, #3570b8 100%);
    border: 1px solid rgba(30, 75, 130, 0.4);
    border-radius: 10px;
    text-decoration: none;
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.25),
      0 2px 6px rgba(16, 45, 72, 0.2);
    transition: transform 160ms ease, box-shadow 160ms ease, background 160ms ease;
  }

  .tools-cta-btn:hover {
    background: linear-gradient(180deg, #5a9de3 0%, #407cc4 100%);
    transform: translateY(-2px);
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.25),
      0 6px 16px rgba(16, 45, 72, 0.22);
  }

  .tools-cta-btn:active {
    transform: translateY(0);
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.1),
      0 1px 3px rgba(16, 45, 72, 0.15);
  }

  @media (max-width: 600px) {
    .tools-cta-icons img {
      width: 44px;
      height: 44px;
    }

    .tools-cta-icons {
      gap: 6px;
    }
  }
</style>
