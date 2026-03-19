<script lang="ts">
  import { env } from '$env/dynamic/public';
  import { adminSession, adminFetch } from '$lib/stores/admin';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let admins = $state<any[]>([]);
  let newId = $state('');
  let newUsername = $state('');
  let newRole = $state('admin');
  let error = $state('');

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    try {
      const res = await adminFetch(apiBase, '/api/v1/admin/users', sess.token);
      if (res.ok) admins = await res.json();
      else if (res.status === 403) error = 'Accès réservé aux super_admin';
    } catch { /* ignore */ }
  }

  $effect(() => { load(); });

  async function addAdmin() {
    const sess = $adminSession;
    if (!sess || !newId.trim() || !newUsername.trim()) return;
    error = '';
    const res = await adminFetch(apiBase, '/api/v1/admin/users', sess.token, {
      method: 'POST',
      body: JSON.stringify({ discord_id: newId.trim(), discord_username: newUsername.trim(), role: newRole }),
    });
    if (!res.ok) {
      const d = await res.json().catch(() => ({}));
      error = d.message || 'Erreur';
      return;
    }
    newId = '';
    newUsername = '';
    load();
  }

  async function deleteAdmin(id: string) {
    const sess = $adminSession;
    if (!sess) return;
    if (!confirm(`Supprimer l'admin ${id} ?`)) return;
    await adminFetch(apiBase, `/api/v1/admin/users/${id}`, sess.token, { method: 'DELETE' });
    load();
  }

  async function changeRole(id: string, role: string) {
    const sess = $adminSession;
    if (!sess) return;
    await adminFetch(apiBase, `/api/v1/admin/users/${id}`, sess.token, {
      method: 'PATCH',
      body: JSON.stringify({ role }),
    });
    load();
  }
</script>

<div class="admin-page">
  <h2>Gestion Admins</h2>

  {#if error}
    <div class="admin-error">{error}</div>
  {/if}

  <div class="add-form">
    <input class="input" type="text" placeholder="Discord ID" bind:value={newId} />
    <input class="input" type="text" placeholder="Username" bind:value={newUsername} />
    <select class="input" bind:value={newRole}>
      <option value="admin">admin</option>
      <option value="super_admin">super_admin</option>
    </select>
    <button class="add-btn" onclick={addAdmin}>Ajouter</button>
  </div>

  <div class="table-wrap">
    <table>
      <thead>
        <tr>
          <th>Avatar</th>
          <th>Username</th>
          <th>Discord ID</th>
          <th>Role</th>
          <th>Dernier login</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each admins as a}
          <tr>
            <td>
              {#if a.discord_avatar}
                <img class="avatar" src="https://cdn.discordapp.com/avatars/{a.discord_id}/{a.discord_avatar}.png?size=32" alt="" />
              {:else}
                <div class="avatar-placeholder"></div>
              {/if}
            </td>
            <td>{a.discord_username}</td>
            <td class="mono">{a.discord_id}</td>
            <td><span class="badge badge--{a.role}">{a.role}</span></td>
            <td class="mono">{a.last_login_at?.slice(0, 10) ?? 'Jamais'}</td>
            <td class="actions">
              {#if a.role === 'admin'}
                <button class="act-btn" onclick={() => changeRole(a.discord_id, 'super_admin')}>Promouvoir</button>
              {:else}
                <button class="act-btn" onclick={() => changeRole(a.discord_id, 'admin')}>Rétrograder</button>
              {/if}
              <button class="act-btn act-btn--danger" onclick={() => deleteAdmin(a.discord_id)}>Supprimer</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .admin-page h2 { margin: 0 0 1rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .admin-error { background: rgba(248,81,73,0.1); border: 1px solid rgba(248,81,73,0.4); color: #f85149; border-radius: 6px; padding: 0.5rem 0.8rem; font-size: 0.82rem; margin-bottom: 1rem; }

  .add-form { display: flex; gap: 0.5rem; margin-bottom: 1.2rem; flex-wrap: wrap; }
  .input { padding: 0.5rem 0.7rem; background: #0d1117; border: 1px solid #30363d; border-radius: 6px; color: #e6edf3; font-size: 0.85rem; }
  .input:focus { outline: none; border-color: #58a6ff; }
  .add-btn { padding: 0.5rem 1rem; background: #238636; border: none; border-radius: 6px; color: #fff; font-weight: 600; cursor: pointer; font-size: 0.85rem; }
  .add-btn:hover { background: #2ea043; }

  .table-wrap { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.82rem; }
  th { text-align: left; padding: 0.5rem 0.6rem; color: #8b949e; border-bottom: 1px solid #30363d; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em; }
  td { padding: 0.45rem 0.6rem; border-bottom: 1px solid #21262d; color: #e6edf3; vertical-align: middle; }
  .mono { font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; color: #8b949e; }

  .avatar { width: 28px; height: 28px; border-radius: 50%; }
  .avatar-placeholder { width: 28px; height: 28px; border-radius: 50%; background: #30363d; }

  .badge { font-size: 0.68rem; font-weight: 600; padding: 0.15em 0.5em; border-radius: 4px; text-transform: uppercase; }
  .badge--admin { background: rgba(88,166,255,0.2); color: #58a6ff; }
  .badge--super_admin { background: rgba(210,153,34,0.2); color: #d29922; }

  .actions { display: flex; gap: 0.3rem; }
  .act-btn { font-size: 0.7rem; padding: 0.2rem 0.5rem; border: 1px solid #30363d; background: #21262d; color: #8b949e; border-radius: 4px; cursor: pointer; }
  .act-btn:hover { color: #e6edf3; border-color: #8b949e; }
  .act-btn--danger { color: #f85149; border-color: rgba(248,81,73,0.3); }
</style>
