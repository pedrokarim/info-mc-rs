<script lang="ts">
  import { onMount } from 'svelte';
  import { adminSession, adminFetch } from '$lib/stores/admin';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Pagination from '$lib/components/ui/Pagination.svelte';

  let entries = $state<any[]>([]);
  let total = $state(0);
  let offset = $state(0);
  let currentPage = $state(1);
  let error = $state('');
  const limit = 50;

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    error = '';
    try {
      const params = new URLSearchParams({ limit: String(limit), offset: String(offset) });
      const res = await adminFetch(`/api/v1/admin/audit?${params}`, sess.token);
      if (!res.ok) throw new Error('Erreur serveur');
      const data = await res.json();
      entries = data.data;
      total = data.total;
    } catch {
      error = 'Impossible de charger les logs';
    }
  }

  onMount(() => { load(); });

  function goToPage(page: number) { offset = (page - 1) * limit; currentPage = page; load(); }
</script>

<div class="admin-page">
  <h2>Audit Log ({total})</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

  <div class="table-wrap">
    <table>
      <thead>
        <tr>
          <th>Date</th>
          <th>Admin</th>
          <th>Action</th>
          <th>Détail</th>
        </tr>
      </thead>
      <tbody>
        {#each entries as e}
          <tr>
            <td class="mono">{e.created_at.slice(0, 16).replace('T', ' ')}</td>
            <td class="mono">{e.discord_id.slice(0, 8)}...</td>
            <td><Badge label={e.action} variant="info" size="sm" /></td>
            <td class="detail">{e.detail ?? '—'}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <Pagination current={currentPage} {total} perPage={limit} onchange={goToPage} />
</div>

<style>
  .admin-page h2 { margin: 0 0 1rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .admin-error { background: rgba(248,81,73,0.1); border: 1px solid rgba(248,81,73,0.4); color: #f85149; border-radius: 6px; padding: 0.5rem 0.8rem; font-size: 0.82rem; margin-bottom: 1rem; }
  .table-wrap { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.82rem; }
  th { text-align: left; padding: 0.5rem 0.6rem; color: #8b949e; border-bottom: 1px solid #30363d; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em; }
  td { padding: 0.45rem 0.6rem; border-bottom: 1px solid #21262d; color: #e6edf3; }
  .mono { font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; color: #8b949e; }
  .detail { font-size: 0.78rem; color: #8b949e; max-width: 400px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
