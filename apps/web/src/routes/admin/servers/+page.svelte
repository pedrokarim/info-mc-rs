<script lang="ts">
  import { onMount } from 'svelte';
  import { adminSession, adminFetch } from '$lib/stores/admin';
  import { addToast } from '$lib/stores/toasts';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import Pagination from '$lib/components/ui/Pagination.svelte';

  let servers = $state<any[]>([]);
  let total = $state(0);
  let search = $state('');
  let sort = $state('last_seen_at');
  let offset = $state(0);
  let loading = $state(false);
  let currentPage = $state(1);
  let error = $state('');

  const limit = 20;

  const statusVariant: Record<string, 'success' | 'danger' | 'warning' | 'info' | 'default'> = {
    active: 'success', banned: 'danger', flagged: 'warning', verified: 'info', pinned: 'warning'
  };

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams({ limit: String(limit), offset: String(offset), sort });
      if (search.trim()) params.set('search', search.trim());
      const res = await adminFetch(`/api/v1/admin/servers?${params}`, sess.token);
      if (!res.ok) throw new Error('Erreur serveur');
      const data = await res.json();
      servers = data.data;
      total = data.total;
    } catch {
      error = 'Impossible de charger les serveurs';
    }
    loading = false;
  }

  onMount(() => { load(); });

  function goToPage(page: number) { offset = (page - 1) * limit; currentPage = page; load(); }

  async function moderate(address: string, action: Record<string, any>) {
    const sess = $adminSession;
    if (!sess) return;
    if (!confirm(`Confirmer l'action sur ${address} ?`)) return;
    try {
      const res = await adminFetch(`/api/v1/admin/servers/${encodeURIComponent(address)}`, sess.token, {
        method: 'PATCH',
        body: JSON.stringify(action),
      });
      if (!res.ok) throw new Error((await res.json().catch(() => ({}))).message || 'Erreur');
      addToast('Action effectuée', 'success');
    } catch (e: any) {
      addToast(e.message || 'Erreur lors de la modération', 'error');
    }
    load();
  }

  async function deleteServer(address: string) {
    const sess = $adminSession;
    if (!sess) return;
    if (!confirm(`Supprimer ${address} de l'index ?`)) return;
    try {
      const res = await adminFetch(`/api/v1/admin/servers/${encodeURIComponent(address)}`, sess.token, { method: 'DELETE' });
      if (!res.ok) throw new Error((await res.json().catch(() => ({}))).message || 'Erreur');
      addToast('Serveur supprimé', 'success');
    } catch (e: any) {
      addToast(e.message || 'Erreur lors de la suppression', 'error');
    }
    load();
  }
</script>

<div class="admin-page">
  <h2>Serveurs ({total})</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

  <div class="toolbar">
    <input class="search-input" type="text" placeholder="Rechercher un serveur..." bind:value={search} onkeydown={(e) => { if (e.key === 'Enter') { offset = 0; currentPage = 1; load(); } }} />
    <select class="sort-select" bind:value={sort} onchange={() => { offset = 0; currentPage = 1; load(); }}>
      <option value="last_seen_at">Récents</option>
      <option value="views">Vues</option>
      <option value="likes">Likes</option>
      <option value="first_seen_at">Anciens</option>
    </select>
  </div>

  {#if loading}<div class="loading"><Spinner size="sm" /> <span>Chargement...</span></div>{/if}

  <div class="table-wrap">
    <table>
      <thead>
        <tr>
          <th>Adresse</th>
          <th>Edition</th>
          <th>Version</th>
          <th>Status</th>
          <th>Vues</th>
          <th>Likes</th>
          <th>Dernier vu</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each servers as s}
          <tr class:banned={s.status === 'banned'}>
            <td><a href="/server/{encodeURIComponent(s.address)}" target="_blank">{s.address}</a></td>
            <td>{s.edition}</td>
            <td class="mono">{s.version_name ?? '—'}</td>
            <td><Badge label={s.status} variant={statusVariant[s.status] ?? 'default'} size="sm" /></td>
            <td>{s.views}</td>
            <td>{s.likes}</td>
            <td class="mono">{s.last_seen_at.slice(0, 10)}</td>
            <td class="actions">
              {#if s.status !== 'banned'}
                <button class="act-btn act-btn--danger" onclick={() => moderate(s.address, { status: 'banned' })}>Ban</button>
              {:else}
                <button class="act-btn act-btn--ok" onclick={() => moderate(s.address, { status: 'active' })}>Unban</button>
              {/if}
              {#if s.status !== 'verified'}
                <button class="act-btn act-btn--ok" onclick={() => moderate(s.address, { status: 'verified' })}>Verify</button>
              {/if}
              <button class="act-btn act-btn--danger" onclick={() => deleteServer(s.address)}>Suppr</button>
            </td>
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
  .loading { color: #8b949e; display: flex; align-items: center; gap: 0.5rem; }
  .toolbar { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
  .search-input { flex: 1; padding: 0.5rem 0.7rem; background: #0d1117; border: 1px solid #30363d; border-radius: 6px; color: #e6edf3; font-size: 0.85rem; }
  .search-input:focus { outline: none; border-color: #58a6ff; }
  .sort-select { padding: 0.5rem; background: #0d1117; border: 1px solid #30363d; border-radius: 6px; color: #e6edf3; font-size: 0.85rem; }
  .table-wrap { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.82rem; }
  th { text-align: left; padding: 0.5rem 0.6rem; color: #8b949e; border-bottom: 1px solid #30363d; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em; }
  td { padding: 0.45rem 0.6rem; border-bottom: 1px solid #21262d; color: #e6edf3; }
  td a { color: #58a6ff; text-decoration: none; }
  td a:hover { text-decoration: underline; }
  .mono { font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; color: #8b949e; }
  tr.banned { opacity: 0.5; }
  .actions { display: flex; gap: 0.3rem; flex-wrap: wrap; }
  .act-btn { font-size: 0.7rem; padding: 0.2rem 0.5rem; border: 1px solid #30363d; background: #21262d; color: #8b949e; border-radius: 4px; cursor: pointer; }
  .act-btn:hover { color: #e6edf3; border-color: #8b949e; }
  .act-btn--danger { color: #f85149; border-color: rgba(248,81,73,0.3); }
  .act-btn--danger:hover { background: rgba(248,81,73,0.15); }
  .act-btn--ok { color: #3fb950; border-color: rgba(63,185,80,0.3); }
  .act-btn--ok:hover { background: rgba(63,185,80,0.15); }
</style>
