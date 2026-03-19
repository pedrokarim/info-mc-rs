<script lang="ts">
  import { env } from '$env/dynamic/public';
  import { adminSession, adminFetch } from '$lib/stores/admin';

  const apiBase = env.PUBLIC_API_BASE || 'http://127.0.0.1:3002';

  let configs = $state<{ key: string; value: string; updated_at: string }[]>([]);
  let editValues = $state<Record<string, string>>({});
  let saving = $state(false);
  let message = $state('');

  async function load() {
    const sess = $adminSession;
    if (!sess) return;
    const res = await adminFetch(apiBase, '/api/v1/admin/config', sess.token);
    if (res.ok) {
      configs = await res.json();
      editValues = Object.fromEntries(configs.map(c => [c.key, c.value]));
    }
  }

  $effect(() => { load(); });

  async function save() {
    const sess = $adminSession;
    if (!sess) return;
    saving = true;
    message = '';
    const changed: Record<string, string> = {};
    for (const c of configs) {
      if (editValues[c.key] !== c.value) changed[c.key] = editValues[c.key];
    }
    if (Object.keys(changed).length === 0) { message = 'Aucun changement'; saving = false; return; }
    const res = await adminFetch(apiBase, '/api/v1/admin/config', sess.token, {
      method: 'PATCH',
      body: JSON.stringify({ values: changed }),
    });
    if (res.ok) {
      configs = await res.json();
      editValues = Object.fromEntries(configs.map(c => [c.key, c.value]));
      message = 'Config sauvegardée';
    } else {
      const d = await res.json().catch(() => ({}));
      message = d.message || 'Erreur';
    }
    saving = false;
  }
</script>

<div class="admin-page">
  <h2>Configuration</h2>

  {#if message}
    <div class="msg">{message}</div>
  {/if}

  <div class="config-grid">
    {#each configs as c}
      <div class="config-row">
        <label class="config-key">{c.key}</label>
        {#if c.key === 'maintenance_mode'}
          <select class="config-input" bind:value={editValues[c.key]}>
            <option value="false">false</option>
            <option value="true">true</option>
          </select>
        {:else}
          <input class="config-input" type="text" bind:value={editValues[c.key]} />
        {/if}
        <span class="config-date">Modifié {c.updated_at.slice(0, 10)}</span>
      </div>
    {/each}
  </div>

  <button class="save-btn" onclick={save} disabled={saving}>
    {saving ? 'Sauvegarde...' : 'Sauvegarder'}
  </button>
</div>

<style>
  .admin-page h2 { margin: 0 0 1rem; font-family: 'Teko', sans-serif; font-size: 1.8rem; color: #e6edf3; }
  .msg { padding: 0.5rem 0.8rem; background: rgba(88,166,255,0.1); border: 1px solid rgba(88,166,255,0.3); color: #58a6ff; border-radius: 6px; font-size: 0.82rem; margin-bottom: 1rem; }

  .config-grid { display: flex; flex-direction: column; gap: 0.8rem; margin-bottom: 1.2rem; }
  .config-row { display: grid; grid-template-columns: 200px 1fr auto; gap: 0.8rem; align-items: center; }
  .config-key { font-family: 'JetBrains Mono', monospace; font-size: 0.82rem; color: #e6edf3; }
  .config-input { padding: 0.45rem 0.6rem; background: #0d1117; border: 1px solid #30363d; border-radius: 6px; color: #e6edf3; font-size: 0.85rem; }
  .config-input:focus { outline: none; border-color: #58a6ff; }
  .config-date { font-size: 0.7rem; color: #8b949e; }

  .save-btn { padding: 0.55rem 1.2rem; background: #238636; border: none; border-radius: 6px; color: #fff; font-weight: 600; cursor: pointer; font-size: 0.88rem; }
  .save-btn:hover { background: #2ea043; }
  .save-btn:disabled { opacity: 0.6; }

  @media (max-width: 600px) {
    .config-row { grid-template-columns: 1fr; }
  }
</style>
