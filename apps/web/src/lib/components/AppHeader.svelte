<script lang="ts">
  import { page } from '$app/state';
  import { env } from '$env/dynamic/public';

  const apiBase = env.PUBLIC_API_BASE || '';

  const links = [
    { href: '/', label: 'Accueil' },
    { href: '/servers', label: 'Serveurs' },
    { href: '/skins', label: 'Skins' },
    { href: '/player/Notch', label: 'Joueurs' },
    { href: '/tools', label: 'Outils' },
    { href: '/docs', label: 'API' }
  ];

  let menuOpen = $state(false);

  // API health state
  let apiStatus: 'checking' | 'online' | 'offline' = $state('checking');
  let apiPing = $state(0);
  let apiVersion = $state('');
  let tooltipVisible = $state(false);
  let detailsOpen = $state(false);
  let mouseX = $state(0);
  let mouseY = $state(0);
  let isMobile = $state(false);

  function onMouseMove(e: MouseEvent) {
    mouseX = e.clientX;
    mouseY = e.clientY;
  }

  function checkMobile() {
    isMobile = window.matchMedia('(max-width: 820px)').matches;
  }

  $effect(() => {
    checkMobile();
    const mq = window.matchMedia('(max-width: 820px)');
    const handler = () => checkMobile();
    mq.addEventListener('change', handler);
    return () => mq.removeEventListener('change', handler);
  });

  async function checkHealth() {
    const t0 = performance.now();
    try {
      const res = await fetch(`${apiBase}/health`, { signal: AbortSignal.timeout(5000) });
      apiPing = Math.round(performance.now() - t0);
      if (res.ok) {
        const data = await res.json();
        apiVersion = data.version || '';
        apiStatus = 'online';
      } else {
        apiStatus = 'offline';
      }
    } catch {
      apiPing = Math.round(performance.now() - t0);
      apiStatus = 'offline';
    }
  }

  $effect(() => {
    checkHealth();
    const iv = setInterval(checkHealth, 30_000);
    return () => clearInterval(iv);
  });

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
    detailsOpen = false;
  });
</script>

{#if menuOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="menu-backdrop" onclick={() => (menuOpen = false)}></div>
{/if}

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
      <div class="api-status-wrap">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <button
          class="nav-link cta"
          class:cta-online={apiStatus === 'online'}
          class:cta-offline={apiStatus === 'offline'}
          type="button"
          onclick={() => (detailsOpen = !detailsOpen)}
          onmouseenter={() => { if (!isMobile) tooltipVisible = true; }}
          onmouseleave={() => { tooltipVisible = false; }}
          onmousemove={onMouseMove}
        >
          API {apiStatus === 'online' ? 'Live' : apiStatus === 'offline' ? 'Down' : '...'}
        </button>

        {#if tooltipVisible && !isMobile && !detailsOpen}
          <div class="api-tooltip" role="tooltip" style="left:{mouseX - 12}px;top:{mouseY + 16}px;transform:translateX(-100%);">
            <span class="tt-row"><span class="tt-label">status</span><span class="tt-val" class:tt-green={apiStatus === 'online'} class:tt-red={apiStatus === 'offline'}>{apiStatus}</span></span>
            <span class="tt-row"><span class="tt-label">ping</span><span class="tt-val">{apiPing}ms</span></span>
            {#if apiVersion}<span class="tt-row"><span class="tt-label">ver</span><span class="tt-val">{apiVersion}</span></span>{/if}
            <span class="tt-row"><span class="tt-label">host</span><span class="tt-val tt-mono">{apiBase || 'local'}</span></span>
          </div>
        {/if}

        {#if detailsOpen}
          <div class="api-details">
            <span class="tt-row"><span class="tt-label">Status</span><span class="tt-val" class:tt-green={apiStatus === 'online'} class:tt-red={apiStatus === 'offline'}>{apiStatus}</span></span>
            <span class="tt-row"><span class="tt-label">Ping</span><span class="tt-val">{apiPing}ms</span></span>
            {#if apiVersion}<span class="tt-row"><span class="tt-label">Version</span><span class="tt-val">{apiVersion}</span></span>{/if}
            <span class="tt-row"><span class="tt-label">Host</span><span class="tt-val tt-mono">{apiBase || 'local'}</span></span>
          </div>
        {/if}
      </div>
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

  @media (max-width: 820px) {
    .burger {
      display: flex;
    }
  }
</style>
