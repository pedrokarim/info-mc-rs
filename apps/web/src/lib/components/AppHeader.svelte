<script lang="ts">
  import { page } from '$app/stores';
  import { env } from '$env/dynamic/public';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  const links = [
    { href: '/', label: 'Accueil' },
    { href: '/server/play.hypixel.net', label: 'Serveurs' },
    { href: '/skins', label: 'Skins' },
    { href: '/player/Notch', label: 'Joueurs' }
  ];

  function isActive(pathname: string, href: string): boolean {
    if (href === '/') return pathname === '/';
    return pathname === href || pathname.startsWith(`${href}/`);
  }
</script>

<div class="topbar-wrap">
  <header class="topbar">
    <a class="brand" href="/">
      <span class="brand-mark" aria-hidden="true"></span>
      <div>
        <h1>MCInfo-RS</h1>
        <p>Minecraft Intelligence</p>
      </div>
    </a>

    <nav class="topnav" aria-label="Site">
      {#each links as link}
        <a class="nav-link" data-active={isActive($page.url.pathname, link.href)} href={link.href}>
          {link.label}
        </a>
      {/each}
      <a class="nav-link cta" href={`${apiBase}/health`} target="_blank" rel="noreferrer">
        API Live
      </a>
    </nav>
  </header>
</div>
