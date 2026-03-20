<script lang="ts">
  import '../../app.css';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { browser } from '$app/environment';
  import { adminSession, clearSession, adminFetch } from '$lib/stores/admin';
  import { env } from '$env/dynamic/public';
  import Avatar from '$lib/components/ui/Avatar.svelte';

  let { children } = $props();

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3001';

  const navItems = [
    { label: 'Dashboard', href: '/admin', icon: '📊' },
    { label: 'Joueurs', href: '/admin/players', icon: '👤' },
    { label: 'Serveurs', href: '/admin/servers', icon: '🖥' },
    { label: 'Admins', href: '/admin/users', icon: '🔑' },
    { label: 'Config', href: '/admin/config', icon: '⚙' },
    { label: 'Alertes', href: '/admin/alerts', icon: '🔔' },
    { label: 'Audit', href: '/admin/audit', icon: '📋' },
  ];

  // Redirect to login if not authenticated (except on login page)
  $effect(() => {
    if (!browser) return;
    const sess = $adminSession;
    const path = $page.url.pathname;
    if (!sess && path !== '/admin/login') {
      goto('/admin/login');
    }
  });

  async function logout() {
    const sess = $adminSession;
    if (sess) {
      await adminFetch(apiBase, '/api/v1/admin/auth/logout', sess.token, { method: 'POST' }).catch(() => {});
    }
    clearSession();
    goto('/admin/login');
  }
</script>

{#if !$adminSession && $page.url.pathname !== '/admin/login'}
  <div class="admin-loading">Redirection...</div>
{:else if $page.url.pathname === '/admin/login'}
  {@render children()}
{:else}
  <div class="admin-shell">
    <nav class="admin-sidebar">
      <div class="admin-brand">
        <a href="/admin">MCInfo Admin</a>
      </div>

      <ul class="admin-nav">
        {#each navItems as item}
          <li>
            <a
              href={item.href}
              class="admin-nav-link"
              class:active={$page.url.pathname === item.href}
            >
              <span class="nav-icon">{item.icon}</span>
              {item.label}
            </a>
          </li>
        {/each}
      </ul>

      <div class="admin-sidebar-footer">
        {#if $adminSession?.user}
          <div class="admin-user">
            <Avatar
              src={$adminSession.user.discord_avatar ? `https://cdn.discordapp.com/avatars/${$adminSession.user.discord_id}/${$adminSession.user.discord_avatar}.png?size=64` : ''}
              alt={$adminSession.user.discord_username}
              fallback={$adminSession.user.discord_username}
              size="sm"
            />
            <div class="admin-user-info">
              <span class="admin-username">{$adminSession.user.discord_username}</span>
              <span class="admin-role">{$adminSession.user.role}</span>
            </div>
          </div>
        {/if}
        <button class="admin-logout" onclick={logout}>Déconnexion</button>
        <a class="admin-back" href="/">← Retour au site</a>
      </div>
    </nav>

    <main class="admin-main">
      {@render children()}
    </main>
  </div>
{/if}

<style>
  .admin-shell {
    display: grid;
    grid-template-columns: 240px 1fr;
    min-height: 100vh;
    background: #0d1117;
    color: #e6edf3;
  }

  .admin-sidebar {
    display: flex;
    flex-direction: column;
    background: #161b22;
    border-right: 1px solid #30363d;
    padding: 1.2rem 0;
  }

  .admin-brand a {
    display: block;
    padding: 0 1.2rem 1rem;
    font-family: 'Teko', sans-serif;
    font-size: 1.6rem;
    font-weight: 600;
    color: #58a6ff;
    text-decoration: none;
    border-bottom: 1px solid #30363d;
    margin-bottom: 0.8rem;
  }

  .admin-nav {
    list-style: none;
    margin: 0;
    padding: 0;
    flex: 1;
  }

  .admin-nav-link {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.55rem 1.2rem;
    color: #8b949e;
    text-decoration: none;
    font-size: 0.88rem;
    font-weight: 500;
    transition: background 100ms, color 100ms;
    border-left: 3px solid transparent;
  }

  .admin-nav-link:hover {
    background: rgba(88, 166, 255, 0.06);
    color: #e6edf3;
  }

  .admin-nav-link.active {
    color: #58a6ff;
    background: rgba(88, 166, 255, 0.1);
    border-left-color: #58a6ff;
  }

  .nav-icon { font-size: 1rem; }

  .admin-sidebar-footer {
    padding: 0.8rem 1.2rem;
    border-top: 1px solid #30363d;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .admin-user {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .admin-user-info {
    display: flex;
    flex-direction: column;
  }

  .admin-username {
    font-size: 0.82rem;
    font-weight: 600;
    color: #e6edf3;
  }

  .admin-role {
    font-size: 0.68rem;
    color: #8b949e;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .admin-logout {
    background: none;
    border: 1px solid #30363d;
    color: #8b949e;
    border-radius: 6px;
    padding: 0.35rem 0.7rem;
    font-size: 0.78rem;
    cursor: pointer;
    transition: color 100ms, border-color 100ms;
  }

  .admin-logout:hover {
    color: #f85149;
    border-color: #f85149;
  }

  .admin-back {
    font-size: 0.75rem;
    color: #8b949e;
    text-decoration: none;
    text-align: center;
  }

  .admin-back:hover { color: #58a6ff; }

  .admin-main {
    padding: 1.5rem 2rem;
    overflow-y: auto;
  }

  .admin-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    color: #8b949e;
    background: #0d1117;
  }

  @media (max-width: 768px) {
    .admin-shell {
      grid-template-columns: 1fr;
    }
    .admin-sidebar {
      flex-direction: row;
      overflow-x: auto;
      padding: 0.5rem;
      border-right: none;
      border-bottom: 1px solid #30363d;
    }
    .admin-nav {
      display: flex;
      gap: 0.2rem;
    }
    .admin-nav-link {
      padding: 0.4rem 0.8rem;
      border-left: none;
      border-bottom: 2px solid transparent;
      white-space: nowrap;
    }
    .admin-nav-link.active { border-bottom-color: #58a6ff; border-left-color: transparent; }
    .admin-brand, .admin-sidebar-footer { display: none; }
  }
</style>
