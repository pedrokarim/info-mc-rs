<script lang="ts">
  import { goto } from '$app/navigation';
  import { env } from '$env/dynamic/public';
  import { onMount } from 'svelte';
  import { adminSession, loadAdminSession } from '$lib/stores/admin';

  interface AscenciaWidget {
    signIn(): Promise<unknown>;
    handleRedirectCallback(): Promise<unknown>;
    on(event: 'signin' | 'error' | 'cancelled', listener: (payload?: unknown) => void): () => void;
  }

  let loading = $state(false);
  let widgetReady = $state(false);
  let error = $state('');

  function getWidget(): AscenciaWidget | undefined {
    return (window as Window & { AscenciaID?: AscenciaWidget }).AscenciaID;
  }

  $effect(() => {
    if ($adminSession) goto('/admin');
  });

  onMount(() => {
    let dispose: (() => void)[] = [];

    const bindWidget = async () => {
      const widget = getWidget();
      if (!widget) return;
      widgetReady = true;
      dispose = [
        widget.on('signin', () => void finishSignIn()),
        widget.on('error', (payload) => showWidgetError(payload)),
        widget.on('cancelled', () => {
          loading = false;
        })
      ];
      try {
        await widget.handleRedirectCallback();
      } catch {
        error = 'Le retour de connexion Ascencia ID n’a pas abouti.';
      }
    };

    if (getWidget()) {
      void bindWidget();
    } else {
      const script = document.createElement('script');
      script.src = 'https://cdn.ascencia.re/id.js?v=4e97c9a259241d5901d94d992aa7a1e73c1ab1ad';
      script.async = true;
      script.dataset.clientId = env.PUBLIC_ASCENCIA_CLIENT_ID || '';
      script.dataset.issuer = env.PUBLIC_ASCENCIA_ISSUER || 'https://id.ascencia.re';
      script.dataset.redirectUri = env.PUBLIC_ASCENCIA_REDIRECT_URI || `${window.location.origin}/admin/login`;
      script.dataset.exchangeUrl = '/api/v1/admin/auth/exchange';
      script.dataset.scopes = 'openid profile email offline_access ascencia.roles';
      script.dataset.siteName = 'MCInfo';
      script.addEventListener('load', () => void bindWidget(), { once: true });
      script.addEventListener('error', () => {
        error = 'Le module Ascencia ID est indisponible.';
      }, { once: true });
      document.head.append(script);
    }

    return () => {
      for (const unsubscribe of dispose) unsubscribe();
    };
  });

  async function startLogin(): Promise<void> {
    const widget = getWidget();
    if (!widget) return;
    loading = true;
    error = '';
    try {
      await widget.signIn();
    } catch {
      error = 'La connexion Ascencia ID a échoué.';
      loading = false;
    }
  }

  async function finishSignIn(): Promise<void> {
    const session = await loadAdminSession();
    loading = false;
    if (session) {
      await goto('/admin');
    } else {
      error = 'La session MCInfo n’a pas pu être créée.';
    }
  }

  function showWidgetError(payload: unknown): void {
    loading = false;
    const code = (payload as { error?: unknown } | null)?.error;
    if (code === 'access_request_pending') {
      error = 'Ta demande a été envoyée. Tu pourras entrer dès qu’elle aura été approuvée.';
    } else if (code === 'access_denied') {
      error = 'Ce compte n’est pas autorisé à administrer MCInfo.';
    } else {
      error = 'La connexion Ascencia ID n’a pas abouti.';
    }
  }
</script>

<svelte:head>
  <title>Administration — MCInfo</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<div class="login-page">
  <div class="login-card">
    <p class="eyebrow">ASCENCIA ID</p>
    <h1>MCInfo Admin</h1>
    <p class="login-sub">Connecte-toi avec le compte autorisé pour administrer MCInfo.</p>

    {#if error}
      <div class="login-error" role="alert">{error}</div>
    {/if}

    <button class="login-btn" onclick={startLogin} disabled={loading || !widgetReady}>
      {loading ? 'Connexion en cours…' : widgetReady ? 'Continuer avec Ascencia ID' : 'Chargement d’Ascencia ID…'}
    </button>
    <p class="access-note">Les accès sont gérés depuis Ascencia ID. Les superadmins de la plateforme sont également autorisés.</p>
  </div>
</div>

<style>
  .login-page {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 1.5rem;
    background: radial-gradient(circle at top, rgba(88, 166, 255, 0.12), transparent 42%), #0d1117;
  }

  .login-card {
    width: 100%;
    max-width: 420px;
    padding: 2.5rem 2rem;
    text-align: center;
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 12px;
    box-shadow: 0 24px 80px rgba(0, 0, 0, 0.35);
  }

  .eyebrow {
    margin: 0 0 0.4rem;
    color: #58a6ff;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.16em;
  }

  h1 {
    margin: 0;
    color: #e6edf3;
    font-family: 'Teko', sans-serif;
    font-size: 2.2rem;
  }

  .login-sub {
    margin: 0.35rem auto 1.5rem;
    color: #8b949e;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .login-error {
    margin-bottom: 1rem;
    padding: 0.7rem 0.85rem;
    color: #ff7b72;
    font-size: 0.82rem;
    line-height: 1.4;
    background: rgba(248, 81, 73, 0.1);
    border: 1px solid rgba(248, 81, 73, 0.4);
    border-radius: 7px;
  }

  .login-btn {
    width: 100%;
    min-height: 46px;
    padding: 0.75rem 1rem;
    color: #ffffff;
    font-size: 0.92rem;
    font-weight: 700;
    background: #1f6feb;
    border: 1px solid #388bfd;
    border-radius: 8px;
    cursor: pointer;
  }

  .login-btn:hover:not(:disabled) { background: #388bfd; }
  .login-btn:disabled { cursor: wait; opacity: 0.62; }

  .access-note {
    margin: 1rem 0 0;
    color: #6e7681;
    font-size: 0.75rem;
    line-height: 1.45;
  }
</style>
