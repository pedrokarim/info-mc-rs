<script lang="ts">
  let expandedSection = $state<string | null>(null);

  function toggle(id: string) {
    expandedSection = expandedSection === id ? null : id;
  }

  const baseUrl = 'https://mcinfo.ascencia.re';

  const sections = [
    {
      id: 'server',
      title: 'Serveurs',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/server/{address}',
          description: 'Statut en temps réel d\'un serveur Minecraft (Java, Bedrock ou auto-detect).',
          params: [
            { name: 'address', in: 'path', desc: 'Adresse du serveur (ex: play.hypixel.net)' },
            { name: 'type', in: 'query', desc: '"auto" | "java" | "bedrock" (defaut: auto)' },
          ],
          example: '/api/v1/server/play.hypixel.net?type=java',
          response: `{
  "online": true,
  "address": { "hostname": "play.hypixel.net", "ip": "172.65.197.160", "port": 25565 },
  "version": { "name": "Requires MC 1.8 / 1.21", "protocol": 769 },
  "players": { "online": 24619, "max": 200000 },
  "motd": { "raw": "...", "clean": "Hypixel Network", "html": "..." },
  "favicon": "data:image/png;base64,...",
  "latency_ms": 42
}`,
        },
      ],
    },
    {
      id: 'player',
      title: 'Joueurs',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/player/{identifier}',
          description: 'Profil complet d\'un joueur : UUID, skin, capes, stats.',
          params: [
            { name: 'identifier', in: 'path', desc: 'Pseudo (3-16 chars) ou UUID' },
          ],
          example: '/api/v1/player/Notch',
          response: `{
  "uuid": "069a79f4-44e9-4726-a5be-fca90e38aaf5",
  "username": "Notch",
  "skin": { "url": "http://textures.minecraft.net/...", "model": "classic" },
  "capes": { "official": null, "optifine": "...", "labymod": null },
  "views": 1523, "likes": 89
}`,
        },
      ],
    },
    {
      id: 'render',
      title: 'Rendu Skin 2D',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/render/{identifier}',
          description: 'Image PNG du skin : face, tete ou corps entier.',
          params: [
            { name: 'identifier', in: 'path', desc: 'Pseudo ou UUID' },
            { name: 'type', in: 'query', desc: '"face" | "head" | "full" (defaut: full)' },
            { name: 'size', in: 'query', desc: '8-512 pixels (defaut: 128)' },
            { name: 'overlay', in: 'query', desc: 'true | false (defaut: true)' },
          ],
          example: '/api/v1/render/Notch?type=head&size=128',
          response: 'image/png',
        },
      ],
    },
    {
      id: 'render3d',
      title: 'Rendu Skin 3D',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/render3d/{identifier}',
          description: 'Rendu 3D isometrique du skin avec animation.',
          params: [
            { name: 'identifier', in: 'path', desc: 'Pseudo ou UUID' },
            { name: 'width', in: 'query', desc: '8-512 (defaut: 240)' },
            { name: 'height', in: 'query', desc: '8-512 (defaut: 360)' },
            { name: 'theta', in: 'query', desc: 'Rotation horizontale en degres (defaut: 30)' },
            { name: 'phi', in: 'query', desc: 'Inclinaison verticale en degres (defaut: 21)' },
            { name: 'time', in: 'query', desc: 'Temps d\'animation (defaut: 90)' },
            { name: 'back', in: 'query', desc: '"cape" | "elytra" | "none" (defaut: cape)' },
          ],
          example: '/api/v1/render3d/Notch?width=256&height=256&theta=30&phi=21',
          response: 'image/png',
        },
      ],
    },
    {
      id: 'cape',
      title: 'Capes',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/cape/{source}/{identifier}',
          description: 'Proxy CORS pour les capes OptiFine et LabyMod.',
          params: [
            { name: 'source', in: 'path', desc: '"optifine" | "labymod"' },
            { name: 'identifier', in: 'path', desc: 'Pseudo (optifine) ou UUID sans tirets (labymod)' },
          ],
          example: '/api/v1/cape/optifine/Notch',
          response: 'image/png',
        },
      ],
    },
    {
      id: 'popular',
      title: 'Classements & Tendances',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/popular/players',
          description: 'Joueurs les plus recherches ou likes.',
          params: [
            { name: 'sort', in: 'query', desc: '"views" | "likes" (defaut: views)' },
            { name: 'limit', in: 'query', desc: '1-100 (defaut: 20)' },
            { name: 'offset', in: 'query', desc: 'Offset pagination (defaut: 0)' },
          ],
          example: '/api/v1/popular/players?sort=views&limit=10',
          response: `[
  { "uuid": "...", "username": "Dream", "views": 5230, "likes": 312, ... },
  ...
]`,
        },
        {
          method: 'GET',
          path: '/api/v1/popular/servers',
          description: 'Serveurs les plus recherches ou likes.',
          params: [
            { name: 'sort', in: 'query', desc: '"views" | "likes" (defaut: views)' },
            { name: 'limit', in: 'query', desc: '1-100 (defaut: 20)' },
            { name: 'offset', in: 'query', desc: 'Offset pagination (defaut: 0)' },
          ],
          example: '/api/v1/popular/servers?sort=views&limit=10',
          response: `[
  { "address": "play.hypixel.net", "hostname": "Hypixel", "views": 8921, ... },
  ...
]`,
        },
        {
          method: 'GET',
          path: '/api/v1/recent/players',
          description: 'Derniers joueurs recherches.',
          params: [
            { name: 'limit', in: 'query', desc: '1-100 (defaut: 20)' },
          ],
          example: '/api/v1/recent/players?limit=5',
          response: 'Meme format que popular/players',
        },
        {
          method: 'GET',
          path: '/api/v1/recent/servers',
          description: 'Derniers serveurs recherches.',
          params: [
            { name: 'limit', in: 'query', desc: '1-100 (defaut: 20)' },
          ],
          example: '/api/v1/recent/servers?limit=5',
          response: 'Meme format que popular/servers',
        },
      ],
    },
    {
      id: 'likes',
      title: 'Likes & Favoris',
      endpoints: [
        {
          method: 'GET',
          path: '/api/v1/player/{uuid}/like',
          description: 'Verifie si le joueur est like par l\'IP actuelle.',
          params: [{ name: 'uuid', in: 'path', desc: 'UUID du joueur' }],
          example: '/api/v1/player/069a79f4.../like',
          response: '{ "liked": true }',
        },
        {
          method: 'POST',
          path: '/api/v1/player/{uuid}/like',
          description: 'Liker un joueur.',
          params: [{ name: 'uuid', in: 'path', desc: 'UUID du joueur' }],
          example: 'POST /api/v1/player/069a79f4.../like',
          response: '201 Created',
        },
        {
          method: 'DELETE',
          path: '/api/v1/player/{uuid}/like',
          description: 'Retirer le like.',
          params: [{ name: 'uuid', in: 'path', desc: 'UUID du joueur' }],
          example: 'DELETE /api/v1/player/069a79f4.../like',
          response: '204 No Content',
        },
      ],
    },
    {
      id: 'health',
      title: 'Health Check',
      endpoints: [
        {
          method: 'GET',
          path: '/health',
          description: 'Verifie que l\'API est en ligne.',
          params: [],
          example: '/health',
          response: '{ "status": "ok", "version": "0.1.0" }',
        },
      ],
    },
  ];
</script>

<main class="page">
  <section class="docs-hero">
    <p class="eyebrow">Documentation</p>
    <h1>MCInfo API</h1>
    <p class="docs-intro">
      API REST gratuite et ouverte. Statut serveur, profils joueur, rendus skin 2D/3D, capes et classements.
      Aucune cle API requise.
    </p>
    <div class="docs-base">
      <span class="docs-base-label">Base URL</span>
      <code class="docs-base-url">{baseUrl}</code>
    </div>
  </section>

  <section class="docs-info">
    <div class="info-card">
      <h3>Gratuit</h3>
      <p>Pas de cle API, pas d'inscription. Utilisez directement.</p>
    </div>
    <div class="info-card">
      <h3>Rate limit</h3>
      <p>Limites raisonnables par IP. Pas de spam, pas de probleme.</p>
    </div>
    <div class="info-card">
      <h3>Cache</h3>
      <p>Serveurs : 60s. Joueurs : 5min. Capes : 1h.</p>
    </div>
    <div class="info-card">
      <h3>Formats</h3>
      <p>JSON pour les donnees, PNG pour les images.</p>
    </div>
  </section>

  {#each sections as section}
    <section class="docs-section">
      <button class="docs-section-head" onclick={() => toggle(section.id)}>
        <h2>{section.title}</h2>
        <span class="docs-chevron" class:open={expandedSection === section.id}></span>
      </button>

      {#if expandedSection === section.id}
        <div class="docs-section-body">
          {#each section.endpoints as ep}
            <div class="endpoint">
              <div class="endpoint-head">
                <span class="method method--{ep.method.toLowerCase()}">{ep.method}</span>
                <code class="endpoint-path">{ep.path}</code>
              </div>
              <p class="endpoint-desc">{ep.description}</p>

              {#if ep.params.length > 0}
                <table class="params-table">
                  <thead>
                    <tr><th>Param</th><th>In</th><th>Description</th></tr>
                  </thead>
                  <tbody>
                    {#each ep.params as p}
                      <tr>
                        <td><code>{p.name}</code></td>
                        <td class="param-in">{p.in}</td>
                        <td>{p.desc}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              {/if}

              <div class="example">
                <span class="example-label">Exemple</span>
                <code class="example-url">{ep.example}</code>
              </div>

              {#if ep.response && ep.response !== 'image/png'}
                <div class="response">
                  <span class="response-label">Reponse</span>
                  <pre class="response-body">{ep.response}</pre>
                </div>
              {:else if ep.response === 'image/png'}
                <div class="response">
                  <span class="response-label">Reponse</span>
                  <code>image/png</code>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </section>
  {/each}
</main>

<style>
  .docs-hero {
    padding: 2rem 0 1.5rem;
  }

  .eyebrow {
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--blue-0);
    margin: 0 0 0.3rem;
  }

  .docs-hero h1 {
    margin: 0;
    font-family: 'Teko', sans-serif;
    font-size: 2.8rem;
    font-weight: 700;
    color: var(--ink-0);
    line-height: 1;
  }

  .docs-intro {
    margin: 0.5rem 0 1.2rem;
    color: var(--ink-1);
    font-size: 0.95rem;
    max-width: 60ch;
  }

  .docs-base {
    display: inline-flex;
    align-items: center;
    gap: 0.6rem;
    background: rgba(255, 255, 255, 0.5);
    border: 1px solid var(--line-0);
    border-radius: 8px;
    padding: 0.5rem 0.9rem;
  }

  .docs-base-label {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2);
  }

  .docs-base-url {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.85rem;
    color: var(--ink-0);
  }

  /* Info cards */
  .docs-info {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.7rem;
    padding: 0 0 1.5rem;
  }

  .info-card {
    background: rgba(255, 255, 255, 0.35);
    border: 1px solid rgba(84, 126, 181, 0.18);
    border-radius: 10px;
    padding: 0.9rem;
  }

  .info-card h3 {
    margin: 0 0 0.2rem;
    font-family: 'Teko', sans-serif;
    font-size: 1.15rem;
    color: var(--ink-0);
  }

  .info-card p {
    margin: 0;
    font-size: 0.8rem;
    color: var(--ink-1);
  }

  /* Sections */
  .docs-section {
    border-top: 1px solid rgba(72, 112, 156, 0.28);
  }

  .docs-section-head {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: none;
    border: none;
    padding: 1rem 0;
    cursor: pointer;
    color: inherit;
  }

  .docs-section-head h2 {
    margin: 0;
    font-family: 'Teko', sans-serif;
    font-size: 1.5rem;
    color: var(--ink-0);
  }

  .docs-chevron {
    display: inline-block;
    width: 0.5rem;
    height: 0.5rem;
    border-right: 2px solid var(--ink-2);
    border-bottom: 2px solid var(--ink-2);
    transform: rotate(45deg);
    transition: transform 200ms ease;
  }

  .docs-chevron.open {
    transform: rotate(-135deg);
  }

  .docs-section-body {
    padding: 0 0 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Endpoint */
  .endpoint {
    background: rgba(255, 255, 255, 0.3);
    border: 1px solid rgba(84, 126, 181, 0.18);
    border-radius: 10px;
    padding: 1rem 1.2rem;
  }

  .endpoint-head {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin-bottom: 0.4rem;
  }

  .method {
    font-size: 0.65rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 0.15em 0.55em;
    border-radius: 4px;
    color: #fff;
  }

  .method--get { background: #169a60; }
  .method--post { background: #2563eb; }
  .method--patch { background: #d97706; }
  .method--delete { background: #dc2626; }

  .endpoint-path {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.85rem;
    color: var(--ink-0);
  }

  .endpoint-desc {
    margin: 0 0 0.7rem;
    font-size: 0.84rem;
    color: var(--ink-1);
  }

  /* Params table */
  .params-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8rem;
    margin-bottom: 0.7rem;
    display: block;
    overflow-x: auto;
  }

  .params-table th {
    text-align: left;
    padding: 0.3rem 0.5rem;
    color: var(--ink-2);
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid rgba(72, 112, 156, 0.2);
  }

  .params-table td {
    padding: 0.3rem 0.5rem;
    border-bottom: 1px solid rgba(72, 112, 156, 0.1);
    color: var(--ink-0);
  }

  .params-table code {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    color: var(--blue-1);
  }

  .param-in {
    font-size: 0.72rem;
    color: var(--ink-2);
  }

  /* Example & response */
  .example, .response {
    margin-top: 0.5rem;
  }

  .example-label, .response-label {
    display: block;
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2);
    margin-bottom: 0.2rem;
  }

  .example-url {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    color: var(--ink-0);
    background: rgba(0, 0, 0, 0.04);
    padding: 0.3rem 0.5rem;
    border-radius: 4px;
    display: inline-block;
    word-break: break-all;
  }

  .response-body {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.74rem;
    color: #d4d4d4;
    background: #1e1e2e;
    padding: 0.7rem 0.9rem;
    border-radius: 8px;
    overflow-x: auto;
    margin: 0;
    line-height: 1.5;
  }

  @media (max-width: 768px) {
    .docs-info {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 480px) {
    .docs-info {
      grid-template-columns: 1fr;
    }
  }
</style>
