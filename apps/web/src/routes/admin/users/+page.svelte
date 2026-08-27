<svelte:head>
  <title>Gestion des accès — MCInfo</title>
</svelte:head>

<div class="admin-page">
  <p class="eyebrow">ASCENCIA ID</p>
  <h2>Gestion des accès</h2>
  <p>
    MCInfo ne maintient plus de liste d’administrateurs séparée. Accorde ou retire le rôle
    <code>admin</code> depuis l’application MCInfo dans Ascencia ID ; les superadmins de la
    plateforme conservent automatiquement l’accès.
  </p>
  <a class="access-link" href="https://id.ascencia.re/console/apps/mcinfo" target="_blank" rel="noreferrer">
    Ouvrir l’application dans Ascencia ID
  </a>
</div>

<style>
  .admin-page {
    max-width: 720px;
    padding: 1.5rem;
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 10px;
  }

  .eyebrow {
    margin: 0 0 0.3rem;
    color: #58a6ff;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.14em;
  }

  h2 {
    margin: 0 0 0.75rem;
    color: #e6edf3;
    font-family: 'Teko', sans-serif;
    font-size: 1.9rem;
  }

  p {
    margin: 0 0 1.2rem;
    color: #b1bac4;
    line-height: 1.65;
  }

  code {
    padding: 0.12rem 0.35rem;
    color: #79c0ff;
    background: #0d1117;
    border-radius: 4px;
  }

  .access-link {
    display: inline-flex;
    align-items: center;
    min-height: 44px;
    padding: 0 1rem;
    color: #ffffff;
    font-size: 0.86rem;
    font-weight: 700;
    text-decoration: none;
    background: #1f6feb;
    border: 1px solid #388bfd;
    border-radius: 7px;
  }

  .access-link:hover { background: #388bfd; }
</style>
