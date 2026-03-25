<script lang="ts">
  import { page } from '$app/state';
  import { env } from '$env/dynamic/public';

  const apiBase = env.PUBLIC_API_BASE || '';

  const links = [
    { href: '/', label: 'Accueil' },
    { href: '/servers', label: 'Serveurs' },
    { href: '/skins', label: 'Skins' },
    { href: '/player/Notch', label: 'Joueurs' },
    { href: '/docs', label: 'API' }
  ];

  let menuOpen = $state(false);

  function isActive(pathname: string, href: string): boolean {
    if (href === '/') return pathname === '/';
    return pathname === href || pathname.startsWith(`${href}/`);
  }

  function isNavActive(pathname: string, href: string): boolean {
    if (href === '/servers') return pathname === '/servers' || pathname.startsWith('/server/');
    return isActive(pathname, href);
  }

  // Close menu on navigation
  $effect(() => {
    page.url.pathname;
    menuOpen = false;
  });
</script>

<div class="topbar-wrap">
  <header class="topbar">
    <a class="brand" href="/">
      <img class="brand-mark" src="/images/logo/logo-mark-v01.png" alt="MCInfo logo" width="36" height="36" />
      <div>
        <span class="brand-name">MCInfo</span>
        <p>Minecraft Intelligence</p>
      </div>
    </a>

    <button class="burger" aria-label="Menu" aria-expanded={menuOpen} onclick={() => (menuOpen = !menuOpen)}>
      <span class="burger-line" class:open={menuOpen}></span>
      <span class="burger-line" class:open={menuOpen}></span>
      <span class="burger-line" class:open={menuOpen}></span>
    </button>

    <nav class="topnav" class:topnav--open={menuOpen} aria-label="Site">
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

<style>
  .burger {
    display: none;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    width: 36px;
    height: 36px;
    flex-shrink: 0;
    background: none;
    border: 1px solid rgba(82, 136, 194, 0.3);
    border-radius: 6px;
    padding: 0;
    cursor: pointer;
  }

  .burger-line {
    display: block;
    width: 18px;
    height: 2px;
    background: var(--ink-1);
    border-radius: 1px;
    transition: transform 200ms ease, opacity 200ms ease;
  }

  .burger-line.open:nth-child(1) {
    transform: translateY(6px) rotate(45deg);
  }
  .burger-line.open:nth-child(2) {
    opacity: 0;
  }
  .burger-line.open:nth-child(3) {
    transform: translateY(-6px) rotate(-45deg);
  }

  @media (max-width: 640px) {
    .burger {
      display: flex;
    }
  }
</style>
