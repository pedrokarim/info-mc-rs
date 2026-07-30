import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';

const API_UPSTREAM = env.API_BASE_INTERNAL || publicEnv.PUBLIC_API_BASE || 'http://127.0.0.1:3001';

export const handle: Handle = async ({ event, resolve }) => {
  // Proxy /api/* requests to the Rust API
  //
  // `/health` est servi à la racine de l'API, pas sous `/api`, et le proxy ne
  // réécrit pas les chemins : sans cette exception la route est injoignable
  // depuis le navigateur. La sonde de `AppHeader` interrogeait donc l'origine
  // web, recevait le 404 de SvelteKit et affichait « API DOWN » en permanence,
  // alors que l'API répondait 200 — d'où aussi la version absente du tooltip.
  const isApiPath = event.url.pathname.startsWith('/api') || event.url.pathname === '/health';

  if (isApiPath) {
    const upstream = `${API_UPSTREAM}${event.url.pathname}${event.url.search}`;

    const headers = new Headers();
    // Forward relevant headers
    for (const key of ['authorization', 'content-type', 'accept', 'x-forwarded-for']) {
      const val = event.request.headers.get(key);
      if (val) headers.set(key, val);
    }
    // Pass real client IP
    const clientIp = event.getClientAddress();
    headers.set('x-forwarded-for', clientIp);

    try {
      const res = await fetch(upstream, {
        method: event.request.method,
        headers,
        body: event.request.method !== 'GET' && event.request.method !== 'HEAD'
          ? event.request.body
          : undefined,
        // @ts-expect-error -- Node fetch supports duplex for streaming body
        duplex: 'half',
      });

      return new Response(res.body, {
        status: res.status,
        statusText: res.statusText,
        headers: res.headers,
      });
    } catch {
      return new Response(JSON.stringify({ error: 'API unreachable' }), {
        status: 502,
        headers: { 'content-type': 'application/json' },
      });
    }
  }

  return resolve(event);
};
