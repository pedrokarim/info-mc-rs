<script lang="ts">
  import { onMount } from 'svelte';
  import { adminSession, adminFetch } from '$lib/stores/admin';
  import { addToast } from '$lib/stores/toasts';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import Pagination from '$lib/components/ui/Pagination.svelte';

  let players = $state<any[]>([]);
  let total = $state(0);
  let search = $state('');
  let sort = $state('last_seen_at');
  let offset = $state(0);
  let loading = $state(false);
  let error = $state('');

  const limit = 20;
  let currentPage = $state(1);

  const statusVariant: Record<string, 'success' | 'danger' | 'warning' | 'default'> = {
    active: 'success', banned: 'danger', flagged: 'warning'
  };

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams({ limit: String(limit), offset: String(offset), sort });
      if (search.trim()) params.set('search', search.trim());
      const res = await adminFetch(`/api/v1/admin/players?${params}`, sess.token);
      if (!res.ok) throw new Error('Erreur serveur');
      const data = await res.json();
      players = data.data;
      total = data.total;
    } catch {
      error = 'Impossible de charger les joueurs';
    }
    loading = false;
  }

  onMount(() => { load(); });

  function goToPage(page: number) { offset = (page - 1) * limit; currentPage = page; load(); }

  async function moderate(uuid: string, action: Record<string, any>) {
    const sess = $adminSession;
    if (!sess) return;
    if (!confirm(`Confirmer l'action sur ${uuid} ?`)) return;
    try {
      const res = await adminFetch(`/api/v1/admin/players/${uuid}`, sess.token, {
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

  async function deletePlayer(uuid: string) {
    const sess = $adminSession;
    if (!sess) return;
    if (!confirm(`Supprimer ${uuid} de l'index ?`)) return;
    try {
      const res = await adminFetch(`/api/v1/admin/players/${uuid}`, sess.token, { method: 'DELETE' });
      if (!res.ok) throw new Error((await res.json().catch(() => ({}))).message || 'Erreur');
      addToast('Joueur supprimé', 'success');
    } catch (e: any) {
      addToast(e.message || 'Erreur lors de la suppression', 'error');
    }
    load();
  }
</script>

<div class="admin-page">
  <h2>Joueurs ({total})</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

  <div class="toolbar">
    <input class="search-input" type="text" placeholder="Rechercher un joueur..." bind:value={search} onkeydown={(e) => { if (e.key === 'Enter') { offset = 0; currentPage = 1; load(); } }} />
    <select class="sort-select" bind:value={sort} onchange={() => { offset = 0; load(); }}>
      <option value="last_seen_at">Récents</option>
      <option value="views">Vues</option>
      <option value="likes">Likes</option>
      <option value="first_seen_at">Anciens</option>
    </select>
  </div>

  {#if loading}
    <div class="loading"><Spinner size="sm" /> <span>Chargement...</span></div>
  {/if}

  <div class="table-wrap">
    <table>
      <thead>
        <tr>
          <th>Joueur</th>
          <th>UUID</th>
          <th>Status</th>
          <th>Vues</th>
          <th>Likes</th>
          <th>Dernier vu</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each players as p}
          <tr class:banned={p.status === 'banned'} class:flagged={p.status === 'flagged'}>
            <td><a href="/player/{p.username}" target="_blank">{p.username}</a></td>
            <td class="mono">{p.uuid.slice(0, 8)}...</td>
            <td><Badge label={p.status} variant={statusVariant[p.status] ?? 'default'} size="sm" /></td>
            <td>{p.views}</td>
            <td>{p.likes}</td>
            <td class="mono">{p.last_seen_at.slice(0, 10)}</td>
            <td class="actions">
              {#if p.status !== 'banned'}
                <button class="act-btn act-btn--danger" onclick={() => moderate(p.uuid, { status: 'banned' })}>Ban</button>
              {:else}
                <button class="act-btn act-btn--ok" onclick={() => moderate(p.uuid, { status: 'active' })}>Unban</button>
              {/if}
              <button class="act-btn" onclick={() => moderate(p.uuid, { reset_likes: true })}>Reset likes</button>
              <button class="act-btn act-btn--danger" onclick={() => deletePlayer(p.uuid)}>Suppr</button>
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
  tr.flagged td:first-child { border-left: 3px solid #d29922; }


  .actions { display: flex; gap: 0.3rem; flex-wrap: wrap; }
  .act-btn { font-size: 0.7rem; padding: 0.2rem 0.5rem; border: 1px solid #30363d; background: #21262d; color: #8b949e; border-radius: 4px; cursor: pointer; }
  .act-btn:hover { color: #e6edf3; border-color: #8b949e; }
  .act-btn--danger { color: #f85149; border-color: rgba(248,81,73,0.3); }
  .act-btn--danger:hover { background: rgba(248,81,73,0.15); }
  .act-btn--ok { color: #3fb950; border-color: rgba(63,185,80,0.3); }
  .act-btn--ok:hover { background: rgba(63,185,80,0.15); }

</style>
