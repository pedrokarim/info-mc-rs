<script lang="ts">
  import { onMount } from 'svelte';
  import { adminSession, adminFetch } from '$lib/stores/admin';
  import { addToast } from '$lib/stores/toasts';
  import Badge from '$lib/components/ui/Badge.svelte';

  let alerts = $state<any[]>([]);
  let total = $state(0);
  let filter = $state('active');
  let offset = $state(0);
  let error = $state('');
  const limit = 30;

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    error = '';
    try {
      const params = new URLSearchParams({ limit: String(limit), offset: String(offset), filter });
      const res = await adminFetch(`/api/v1/admin/alerts?${params}`, sess.token);
      if (!res.ok) throw new Error('Erreur serveur');
      const data = await res.json();
      alerts = data.data;
      total = data.total;
    } catch {
      error = 'Impossible de charger les alertes';
    }
  }

  onMount(() => { load(); });

  async function resolve(id: number) {
    const sess = $adminSession;
    if (!sess) return;
    try {
      const res = await adminFetch(`/api/v1/admin/alerts/${id}`, sess.token, { method: 'PATCH' });
      if (!res.ok) throw new Error((await res.json().catch(() => ({}))).message || 'Erreur');
      addToast('Alerte résolue', 'success');
    } catch (e: any) {
      addToast(e.message || 'Erreur lors de la résolution', 'error');
    }
    load();
  }
</script>

<div class="admin-page">
  <h2>Alertes ({total})</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

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
        <Badge label={a.alert_type} variant="default" size="sm" />
        <Badge label={a.severity} variant={a.severity === 'critical' ? 'danger' : a.severity === 'warning' ? 'warning' : 'info'} size="sm" />
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
  .admin-error { background: rgba(248,81,73,0.1); border: 1px solid rgba(248,81,73,0.4); color: #f85149; border-radius: 6px; padding: 0.5rem 0.8rem; font-size: 0.82rem; margin-bottom: 1rem; }
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
  .alert-date { font-size: 0.72rem; color: #8b949e; margin-left: auto; font-family: 'JetBrains Mono', monospace; }

  .alert-msg { margin: 0; font-size: 0.85rem; color: #e6edf3; }
  .alert-entity { margin: 0.2rem 0 0; font-size: 0.75rem; color: #8b949e; font-family: 'JetBrains Mono', monospace; }

  .resolve-btn { margin-top: 0.5rem; padding: 0.3rem 0.7rem; background: #238636; border: none; border-radius: 5px; color: #fff; font-size: 0.75rem; font-weight: 600; cursor: pointer; }
  .resolved-by { font-size: 0.72rem; color: #8b949e; }
</style>
