<script lang="ts">
  import { goto } from '$app/navigation';
  import GameChip from '$lib/components/ui/GameChip.svelte';
  import HeroBleed from '$lib/components/ui/HeroBleed.svelte';
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import TogglePills from '$lib/components/ui/TogglePills.svelte';

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

  const spotlightServers = [
    { name: 'Hypixel', address: 'play.hypixel.net', tag: 'Network' },
    { name: 'CubeCraft', address: 'play.cubecraft.net', tag: 'Mini-games' },
    { name: 'Origin Realms', address: 'play.originrealms.com', tag: 'Survival+' }
  ];

  const spotlightPlayers = ['Notch', 'Dream', 'Technoblade', 'Skeppy'];
</script>

<main class="page">
  <HeroBleed
    eyebrow="Minecraft Data Hub"
    title="Un front qui ressemble enfin a un vrai site de jeu."
    description="Direction Chunklock: hero impactant, navigation arcade, et infos claires pour lookup serveur, MOTD visuel et explorer de skins."
    image="/images/hero/hero-main-v01.png"
    primaryLabel="Voir un serveur"
    primaryHref="/server/play.hypixel.net"
    secondaryLabel="Explorer les skins"
    secondaryHref="/skins"
  />

  <section class="lookup-panel section-strip">
    <SectionHeading
      title="Lookup rapide"
      description="Entre une adresse ou un pseudo pour ouvrir la fiche detaillee."
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

  <section class="section-strip split-strip">
    <article class="plain-column">
      <div class="section-title section-title-light">
        <h3>Spotlight Servers</h3>
      </div>
      <div class="plain-list">
        {#each spotlightServers as srv}
          <a class="plain-item" href={`/server/${encodeURIComponent(srv.address)}`}>
            <p class="name">{srv.name}</p>
            <p class="meta">{srv.address} · {srv.tag}</p>
          </a>
        {/each}
      </div>
    </article>

    <article class="plain-column">
      <div class="section-title section-title-light">
        <h3>Skin Explorer</h3>
      </div>
      <div>
        <p class="empty section-copy">
          Recherche type NameMC avec details skin, modele et rendus head/full.
        </p>
        <div class="quick-actions">
          {#each spotlightPlayers as player}
            <GameChip label={player} href={`/player/${encodeURIComponent(player)}`} />
          {/each}
          <GameChip label="Voir la galerie" href="/skins" />
        </div>
      </div>
    </article>
  </section>
</main>
