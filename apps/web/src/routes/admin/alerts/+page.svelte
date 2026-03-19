<script lang="ts">
  import { env } from '$env/dynamic/public';
  import { adminSession, adminFetch } from '$lib/stores/admin';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let alerts = $state<any[]>([]);
  let total = $state(0);
  let filter = $state('active');
  let offset = $state(0);
  const limit = 30;

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    const params = new URLSearchParams({ limit: String(limit), offset: String(offset), filter });
    const res = await adminFetch(apiBase, `/api/v1/admin/alerts?${params}`, sess.token);
    if (res.ok) {
      const data = await res.json();
      alerts = data.data;
      total = data.total;
    }
  }

  $effect(() => { load(); });

  async function resolve(id: number) {
    const sess = $adminSession;
    if (!sess) return;
    await adminFetch(apiBase, `/api/v1/admin/alerts/${id}`, sess.token, { method: 'PATCH' });
    load();
  }
</script>

<div class="admin-page">
  <h2>Alertes ({total})</h2>

  <div class="toolbar">
    {#each ['active', 'resolved', 'all'] as f}
      <button class="filter-btn" class:active={filter === f} onclick={() => { filter = f; offset = 0; load(); }}>{f}</button>
    {/each}
  </div>

  {#if alerts.length === 0}
    <p class="empty">Aucune alerte {filter === 'active' ? 'active' : ''}</p>
  {/if}

  {#each alerts as a}
    <div class="alert-card alert--{a.severity}" class:resolved={a.resolved}>
      <div class="alert-head">
        <span class="alert-type">{a.alert_type}</span>
        <span class="alert-severity">{a.severity}</span>
        <span class="alert-date">{a.created_at.slice(0, 16).replace('T', ' ')}</span>
      </div>
      <p class="alert-msg">{a.message}</p>
      {#if a.entity_type}
        <p class="alert-entity">{a.entity_type}: {a.entity_id}</p>
      {/if}
      {#if !a.resolved}
        <button class="resolve-btn" onclick={() => resolve(a.id)}>Résoudre</button>
      {:else}
        <span class="resolved-by">Résolu par {a.resolved_by} le {a.resolved_at?.slice(0, 10)}</span>
      {/if}
    </div>
  {/each}
</div>

<style>
  .admin-page h2 { margin: 0 0 1rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .empty { color: #8b949e; }

  .toolbar { display: flex; gap: 0.3rem; margin-bottom: 1rem; }
  .filter-btn { padding: 0.35rem 0.8rem; background: #21262d; border: 1px solid #30363d; color: #8b949e; border-radius: 6px; cursor: pointer; font-size: 0.78rem; text-transform: capitalize; }
  .filter-btn.active { color: #58a6ff; border-color: #58a6ff; background: rgba(88,166,255,0.1); }

  .alert-card { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 0.8rem 1rem; margin-bottom: 0.6rem; }
  .alert-card.resolved { opacity: 0.5; }
  .alert--warning { border-left: 3px solid #d29922; }
  .alert--critical { border-left: 3px solid #f85149; }
  .alert--info { border-left: 3px solid #58a6ff; }

  .alert-head { display: flex; align-items: center; gap: 0.6rem; margin-bottom: 0.3rem; }
  .alert-type { font-weight: 700; font-size: 0.8rem; color: #e6edf3; }
  .alert-severity { font-size: 0.68rem; text-transform: uppercase; padding: 0.1em 0.4em; border-radius: 3px; font-weight: 600; }
  .alert--warning .alert-severity { background: rgba(210,153,34,0.2); color: #d29922; }
  .alert--critical .alert-severity { background: rgba(248,81,73,0.2); color: #f85149; }
  .alert--info .alert-severity { background: rgba(88,166,255,0.2); color: #58a6ff; }
  .alert-date { font-size: 0.72rem; color: #8b949e; margin-left: auto; font-family: 'JetBrains Mono', monospace; }

  .alert-msg { margin: 0; font-size: 0.85rem; color: #e6edf3; }
  .alert-entity { margin: 0.2rem 0 0; font-size: 0.75rem; color: #8b949e; font-family: 'JetBrains Mono', monospace; }

  .resolve-btn { margin-top: 0.5rem; padding: 0.3rem 0.7rem; background: #238636; border: none; border-radius: 5px; color: #fff; font-size: 0.75rem; font-weight: 600; cursor: pointer; }
  .resolved-by { font-size: 0.72rem; color: #8b949e; }
</style>
