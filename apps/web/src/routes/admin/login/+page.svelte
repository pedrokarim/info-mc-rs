<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { browser } from '$app/environment';
  import { adminSession, saveSession, API_BASE } from '$lib/stores/admin';

  let loading = $state(false);
  let error = $state('');
  let needs2fa = $state(false);
  let tempToken = $state('');
  let totpCode = $state('');
  let pendingUser = $state<any>(null);

  // If already logged in, redirect to dashboard
  $effect(() => {
    if (browser && $adminSession) {
      goto('/admin');
    }
  });

  // Handle OAuth callback code
  $effect(() => {
    if (!browser) return;
    const code = $page.url.searchParams.get('code');
    if (code) {
      handleCallback(code);
    }
  });

  async function startLogin() {
    loading = true;
    error = '';
    try {
      const res = await fetch(`${API_BASE}/api/v1/admin/auth/login`);
      const data = await res.json();
      if (!res.ok) {
        error = data.message || 'Erreur lors de la connexion';
        return;
      }
      // Redirect to Discord
      window.location.href = data.url;
    } catch (e) {
      error = 'Impossible de contacter l\'API';
    } finally {
      loading = false;
    }
  }

  async function handleCallback(code: string) {
    loading = true;
    error = '';
    try {
      const res = await fetch(`${API_BASE}/api/v1/admin/auth/callback?code=${encodeURIComponent(code)}`);
      const data = await res.json();

      if (!res.ok) {
        error = data.message || 'Authentification refusée';
        // Clean URL
        window.history.replaceState({}, '', '/admin/login');
        return;
      }

      if (data.requires_2fa) {
        needs2fa = true;
        tempToken = data.token;
        pendingUser = data.user;
        window.history.replaceState({}, '', '/admin/login');
        return;
      }

      saveSession({
        token: data.token,
        expires_at: data.expires_at,
        user: data.user,
      });
      goto('/admin');
    } catch (e) {
      error = 'Erreur de callback';
    } finally {
      loading = false;
    }
  }

  async function verify2fa() {
    if (!totpCode.trim() || totpCode.length !== 6) {
      error = 'Entrez un code à 6 chiffres';
      return;
    }
    loading = true;
    error = '';
    try {
      const res = await fetch(`${API_BASE}/api/v1/admin/auth/2fa/verify`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ temp_token: tempToken, code: totpCode }),
      });
      const data = await res.json();

      if (!res.ok) {
        error = data.message || 'Code invalide';
        return;
      }

      saveSession({
        token: data.token,
        expires_at: data.expires_at,
        user: pendingUser,
      });
      goto('/admin');
    } catch (e) {
      error = 'Erreur de vérification';
    } finally {
      loading = false;
    }
  }
</script>

<div class="login-page">
  <div class="login-card">
    <h1>MCInfo Admin</h1>
    <p class="login-sub">Panneau d'administration</p>

    {#if error}
      <div class="login-error">{error}</div>
    {/if}

    {#if needs2fa}
      <p class="login-2fa-label">Authentification 2FA requise</p>
      <form onsubmit={(e) => { e.preventDefault(); verify2fa(); }}>
        <input
          class="login-input"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          maxlength="6"
          placeholder="Code à 6 chiffres"
          bind:value={totpCode}
          autofocus
        />
        <button class="login-btn" type="submit" disabled={loading}>
          {loading ? 'Vérification...' : 'Vérifier'}
        </button>
      </form>
    {:else}
      <button class="login-btn login-btn--discord" onclick={startLogin} disabled={loading}>
        <svg width="20" height="20" viewBox="0 0 71 55" fill="currentColor">
          <path d="M60.1 4.9A58.5 58.5 0 0045.4.2a.2.2 0 00-.2.1 40.8 40.8 0 00-1.8 3.7 54 54 0 00-16.2 0A37.4 37.4 0 0025.4.3a.2.2 0 00-.2-.1A58.4 58.4 0 0010.5 4.9a.2.2 0 00-.1.1C1.5 18.7-.9 32.2.3 45.5v.1a58.7 58.7 0 0017.7 9a.2.2 0 00.3-.1 42 42 0 003.6-5.9.2.2 0 00-.1-.3 38.6 38.6 0 01-5.5-2.6.2.2 0 01 0-.4l1.1-.9a.2.2 0 01.2 0 41.9 41.9 0 0035.6 0 .2.2 0 01.2 0l1.1.9a.2.2 0 010 .3 36.3 36.3 0 01-5.5 2.7.2.2 0 00-.1.3 47.1 47.1 0 003.6 5.8.2.2 0 00.3.1A58.5 58.5 0 0070.4 45.6v-.1c1.4-14.8-2.3-27.7-9.8-39.1a.2.2 0 00-.1 0zM23.7 37.3c-3.4 0-6.2-3.1-6.2-7s2.7-7 6.2-7 6.3 3.2 6.2 7-2.7 7-6.2 7zm22.9 0c-3.4 0-6.2-3.1-6.2-7s2.7-7 6.2-7 6.3 3.2 6.2 7-2.8 7-6.2 7z"/>
        </svg>
        {loading ? 'Connexion...' : 'Se connecter avec Discord'}
      </button>
    {/if}
  </div>
</div>

<style>
  .login-page {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: #0d1117;
  }

  .login-card {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 12px;
    padding: 2.5rem 2rem;
    width: 100%;
    max-width: 380px;
    text-align: center;
  }

  h1 {
    margin: 0;
    font-family: 'Teko', sans-serif;
    font-size: 2rem;
    color: #58a6ff;
  }

  .login-sub {
    color: #8b949e;
    font-size: 0.85rem;
    margin: 0.3rem 0 1.5rem;
  }

  .login-error {
    background: rgba(248, 81, 73, 0.1);
    border: 1px solid rgba(248, 81, 73, 0.4);
    color: #f85149;
    border-radius: 6px;
    padding: 0.5rem 0.8rem;
    font-size: 0.82rem;
    margin-bottom: 1rem;
  }

  .login-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.7rem 1rem;
    border: none;
    border-radius: 8px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 100ms;
    color: #fff;
    background: #238636;
  }

  .login-btn:disabled { opacity: 0.6; cursor: not-allowed; }

  .login-btn--discord {
    background: #5865f2;
  }

  .login-btn--discord:hover:not(:disabled) {
    background: #4752c4;
  }

  .login-input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.6rem 0.8rem;
    border: 1px solid #30363d;
    background: #0d1117;
    color: #e6edf3;
    border-radius: 6px;
    font-size: 1.2rem;
    text-align: center;
    letter-spacing: 0.3em;
    font-family: 'JetBrains Mono', monospace;
    margin-bottom: 0.8rem;
  }

  .login-input:focus {
    outline: none;
    border-color: #58a6ff;
  }

  .login-2fa-label {
    color: #e6edf3;
    font-size: 0.88rem;
    margin-bottom: 1rem;
  }
</style>
