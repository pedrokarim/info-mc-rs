<script lang="ts">
  import { env } from '$env/dynamic/public';
  import { adminSession, adminFetch } from '$lib/stores/admin';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let entries = $state<any[]>([]);
  let total = $state(0);
  let offset = $state(0);
  const limit = 50;

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    const params = new URLSearchParams({ limit: String(limit), offset: String(offset) });
    const res = await adminFetch(apiBase, `/api/v1/admin/audit?${params}`, sess.token);
    if (res.ok) {
      const data = await res.json();
      entries = data.data;
      total = data.total;
    }
  }

  $effect(() => { load(); });

  function nextPage() { offset += limit; load(); }
  function prevPage() { offset = Math.max(0, offset - limit); load(); }
</script>

<div class="admin-page">
  <h2>Audit Log ({total})</h2>

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
            <td><span class="action-badge">{e.action}</span></td>
            <td class="detail">{e.detail ?? '—'}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="pagination">
    <button disabled={offset === 0} onclick={prevPage}>Précédent</button>
    <span>{offset + 1}–{Math.min(offset + limit, total)} / {total}</span>
    <button disabled={offset + limit >= total} onclick={nextPage}>Suivant</button>
  </div>
</div>

<style>
  .admin-page h2 { margin: 0 0 1rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .table-wrap { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.82rem; }
  th { text-align: left; padding: 0.5rem 0.6rem; color: #8b949e; border-bottom: 1px solid #30363d; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em; }
  td { padding: 0.45rem 0.6rem; border-bottom: 1px solid #21262d; color: #e6edf3; }
  .mono { font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; color: #8b949e; }
  .detail { font-size: 0.78rem; color: #8b949e; max-width: 400px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .action-badge { font-size: 0.7rem; font-weight: 600; padding: 0.15em 0.45em; border-radius: 4px; background: rgba(88,166,255,0.1); color: #58a6ff; }
  .pagination { display: flex; align-items: center; justify-content: center; gap: 1rem; margin-top: 1rem; font-size: 0.82rem; color: #8b949e; }
  .pagination button { padding: 0.35rem 0.8rem; background: #21262d; border: 1px solid #30363d; color: #e6edf3; border-radius: 6px; cursor: pointer; font-size: 0.78rem; }
  .pagination button:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
