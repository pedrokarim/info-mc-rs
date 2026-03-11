<script lang="ts">
  import { page } from '$app/state';
  import { env } from '$env/dynamic/public';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  const links = [
    { href: '/', label: 'Accueil' },
    { href: '/servers', label: 'Serveurs' },
    { href: '/skins', label: 'Skins' },
    { href: '/player/Notch', label: 'Joueurs' }
  ];

  function isActive(pathname: string, href: string): boolean {
    if (href === '/') return pathname === '/';
    return pathname === href || pathname.startsWith(`${href}/`);
  }

  // Also highlight Serveurs when on a /server/[address] detail page
  function isNavActive(pathname: string, href: string): boolean {
    if (href === '/servers') return pathname === '/servers' || pathname.startsWith('/server/');
    return isActive(pathname, href);
  }
</script>

<div class="topbar-wrap">
  <header class="topbar">
    <a class="brand" href="/">
      <img class="brand-mark" src="/images/logo/logo-mark-v01.png" alt="MCInfo logo" width="36" height="36" />
      <div>
        <h1>MCInfo</h1>
        <p>Minecraft Intelligence</p>
      </div>
    </a>

    <nav class="topnav" aria-label="Site">
      {#each links as link}
        <a class="nav-link" data-active={isNavActive(page.url.pathname, link.href)} href={link.href}>
          {link.label}
        </a>
      {/each}
      <a class="nav-link cta" href={`${apiBase}/health`} target="_blank" rel="noreferrer">
        API Live
      </a>
    </nav>
  </header>
</div>
