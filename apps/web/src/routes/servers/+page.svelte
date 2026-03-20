<script lang="ts">
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import MotdBlock from '$lib/components/ui/MotdBlock.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  interface ServerEntry {
    name: string;
    address: string;
    tag: string;
    description: string;
    edition: 'java' | 'bedrock' | 'both';
  }

  const serversByCategory: { label: string; servers: ServerEntry[] }[] = [
    {
      label: 'Réseau / Mini-jeux',
      servers: [
        { name: 'Hypixel', address: 'play.hypixel.net', tag: 'Java', description: 'Le plus grand réseau Java au monde', edition: 'java' },
        { name: 'CubeCraft', address: 'play.cubecraft.net', tag: 'Java + Bedrock', description: 'Mini-jeux multijoueur populaires', edition: 'both' },
        { name: 'Mineplex', address: 'us.mineplex.com', tag: 'Java', description: 'Réseau mini-jeux classique', edition: 'java' },
        { name: 'The Hive', address: 'geo.hivebedrock.network', tag: 'Bedrock', description: 'Mini-jeux Bedrock cross-platform', edition: 'bedrock' },
      ]
    },
    {
      label: 'Survie & Aventure',
      servers: [
        { name: 'Wynncraft', address: 'play.wynncraft.com', tag: 'MMORPG', description: 'RPG complet dans Minecraft', edition: 'java' },
        { name: 'EarthMC', address: 'play.earthmc.net', tag: 'Géopolitique', description: 'Carte Terre 1:1500, nations et guerres', edition: 'java' },
        { name: '2b2t', address: '2b2t.org', tag: 'Anarchie', description: 'Le plus vieux serveur anarchie (2010)', edition: 'java' },
        { name: 'Minehut', address: 'minehut.com', tag: 'Free hosting', description: 'Hôte ton propre serveur gratuitement', edition: 'java' },
      ]
    },
    {
      label: 'Serveurs français',
      servers: [
        { name: 'Epicube', address: 'play.epicube.fr', tag: 'Mini-jeux FR', description: 'Réseau français #1 mini-jeux', edition: 'java' },
        { name: 'FunCraft', address: 'play.funcraft.net', tag: 'Mini-jeux FR', description: 'BedWars, SkyWars, Murder Mystery', edition: 'java' },
        { name: 'Minestrator', address: 'play.minestrator.com', tag: 'Faction / PvP', description: 'Faction, Skyblock, KitPvP français', edition: 'java' },
        { name: 'Minefight', address: 'minefight.fr', tag: 'PvP FR', description: 'KitPvP & UHC à la française', edition: 'java' },
      ]
    }
  ];

  const editionLabel: Record<string, string> = {
    java: 'Java',
    bedrock: 'Bedrock',
    both: 'Java + Bedrock'
  };
</script>

<main class="page">
  <section class="hero hero-server">
    <div class="hero-copy">
      <p class="eyebrow">Serveurs</p>
      <h2>Les serveurs Minecraft populaires</h2>
      <p>
        Sélectionne un serveur pour voir son statut en temps réel, ses joueurs connectés et son MOTD visuel.
      </p>
      <div class="hero-tags">
        <span class="status neutral">Statut temps réel</span>
        <span class="status neutral">MOTD visuel</span>
        <span class="status neutral">Java + Bedrock</span>
      </div>
    </div>
    <div class="hero-media hero-media-server">
      <div class="hero-overlay">
        <p>Server lobby</p>
        <h3>Serveurs populaires</h3>
      </div>
    </div>
  </section>

  {#if data.popularServers.length > 0}
    <section class="surface lookup-panel">
      <SectionHeading title="Tendances" description="Les serveurs les plus recherchés sur MCInfo." light={true} />
      <div class="server-grid">
        {#each data.popularServers as srv}
          {@const liveMotd = data.storedMap[srv.address.toLowerCase()]?.motd_html ?? data.storedMap[srv.hostname.toLowerCase()]?.motd_html}
          <a class="server-card" href={`/server/${encodeURIComponent(srv.address)}`}>
            <div class="server-card-head">
              <p class="server-name">{srv.hostname}</p>
              <span class="server-edition">{srv.edition}</span>
            </div>
            <p class="server-address">{srv.address}</p>
            {#if liveMotd || srv.motd_html || srv.motd_clean}
              <div class="server-motd">
                <MotdBlock html={liveMotd ?? srv.motd_html ?? srv.motd_clean ?? ''} />
              </div>
            {/if}
            <p class="server-stats">{srv.views} vues · {srv.likes} likes</p>
            <span class="server-cta">Voir le statut →</span>
          </a>
        {/each}
      </div>
    </section>
  {/if}

  {#each serversByCategory as category}
    <section class="surface lookup-panel">
      <SectionHeading title={category.label} light={true} />

      <div class="server-grid">
        {#each category.servers as srv}
          {@const stored = data.storedMap[srv.address.toLowerCase()]}
          <a class="server-card" href={`/server/${encodeURIComponent(srv.address)}`}>
            <div class="server-card-head">
              <p class="server-name">{srv.name}</p>
              <span class="server-edition">{editionLabel[srv.edition]}</span>
            </div>
            <p class="server-address">{srv.address}</p>
            {#if stored?.motd_html}
              <div class="server-motd">
                <MotdBlock html={stored.motd_html} />
              </div>
            {:else}
              <p class="server-desc">{srv.description}</p>
            {/if}
            <span class="server-cta">Voir le statut →</span>
          </a>
        {/each}
      </div>
    </section>
  {/each}

  <section class="surface lookup-panel">
    <SectionHeading
      title="Ton serveur"
      description="Entre n'importe quelle adresse pour voir le statut en direct."
      light={true}
    />
    <div class="quick-actions">
      <GameChip label="Analyser un serveur" href="/" />
    </div>
  </section>
</main>

<style>
  .server-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.74rem;
  }

  .server-card {
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

  .server-card:hover {
    transform: translateY(-2px);
    border-color: rgba(84, 139, 206, 0.72);
    box-shadow: 0 10px 20px rgba(21, 61, 96, 0.15);
    background: rgba(255, 255, 255, 0.34);
  }

  .server-card-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .server-name {
    margin: 0;
    color: #0e3555;
    font-family: 'Teko', 'Chakra Petch', sans-serif;
    font-size: 1.45rem;
    font-weight: 600;
    line-height: 0.95;
  }

  .server-edition {
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

  .server-address {
    margin: 0;
    color: var(--ink-2);
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.76rem;
  }

  .server-motd {
    flex: 1;
  }

  .server-desc {
    margin: 0;
    color: var(--ink-1);
    font-size: 0.82rem;
    flex: 1;
  }

  .server-stats {
    margin: 0;
    font-size: 0.72rem;
    color: var(--ink-2);
    font-weight: 600;
  }

  .server-cta {
    margin-top: 0.3rem;
    font-size: 0.74rem;
    font-weight: 700;
    color: #1a5080;
    letter-spacing: 0.03em;
  }

  .server-card:hover .server-cta {
    color: #0e3a62;
  }

  @media (max-width: 980px) {
    .server-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 580px) {
    .server-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
