<script lang="ts">
  import { onMount } from 'svelte';
  import { adminSession, adminFetch } from '$lib/stores/admin';

  interface Metrics {
    total_players: number;
    total_servers: number;
    total_views_players: number;
    total_views_servers: number;
    total_likes: number;
    total_favorites: number;
    players_last_24h: number;
    servers_last_24h: number;
    admin_count: number;
  }

  let metrics = $state<Metrics | null>(null);
  let error = $state('');

  onMount(() => {
    const sess = $adminSession;
    if (!sess) return;
    adminFetch('/api/v1/admin/dashboard', sess.token)
      .then(r => {
        if (!r.ok) throw new Error('Erreur serveur');
        return r.json();
      })
      .then(d => { metrics = d; })
      .catch(() => { error = 'Impossible de charger les métriques'; });
  });

  function fmt(n: number): string {
    return n.toLocaleString('fr-FR');
  }
</script>

<div class="admin-page">
  <h2>Dashboard</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

  {#if metrics}
    <div class="stats-grid">
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.total_players)}</span>
        <span class="stat-label">Joueurs indexés</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.total_servers)}</span>
        <span class="stat-label">Serveurs indexés</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.total_views_players + metrics.total_views_servers)}</span>
        <span class="stat-label">Vues totales</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.total_likes)}</span>
        <span class="stat-label">Likes</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.total_favorites)}</span>
        <span class="stat-label">Favoris</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{fmt(metrics.admin_count)}</span>
        <span class="stat-label">Admins</span>
      </div>
    </div>

    <h3>Dernières 24h</h3>
    <div class="stats-grid stats-grid--sm">
      <div class="stat-card stat-card--accent">
        <span class="stat-value">{fmt(metrics.players_last_24h)}</span>
        <span class="stat-label">Joueurs recherchés</span>
      </div>
      <div class="stat-card stat-card--accent">
        <span class="stat-value">{fmt(metrics.servers_last_24h)}</span>
        <span class="stat-label">Serveurs recherchés</span>
      </div>
    </div>
  {:else if !error}
    <p class="loading">Chargement...</p>
  {/if}
</div>

<style>
  .admin-page h2 { margin: 0 0 1.2rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .admin-page h3 { margin: 1.5rem 0 0.8rem; font-size: 1rem; color: #8b949e; text-transform: uppercase; letter-spacing: 0.06em; }
  .admin-error { background: rgba(248,81,73,0.1); border: 1px solid rgba(248,81,73,0.4); color: #f85149; border-radius: 6px; padding: 0.5rem 0.8rem; font-size: 0.82rem; margin-bottom: 1rem; }
  .loading { color: #8b949e; }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.8rem;
  }
  .stats-grid--sm { grid-template-columns: repeat(2, 1fr); }

  .stat-card {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 10px;
    padding: 1.2rem;
    text-align: center;
  }
  .stat-card--accent { border-color: rgba(88,166,255,0.3); }

  .stat-value {
    display: block;
    font-family: 'Teko', sans-serif;
    font-size: 2.2rem;
    font-weight: 600;
    color: #e6edf3;
    line-height: 1;
  }
  .stat-card--accent .stat-value { color: #58a6ff; }

  .stat-label {
    display: block;
    font-size: 0.75rem;
    color: #8b949e;
    margin-top: 0.3rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  @media (max-width: 600px) {
    .stats-grid { grid-template-columns: repeat(2, 1fr); }
  }
</style>
