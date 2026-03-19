import { env } from '$env/dynamic/public';

import type { ApiErrorResponse, PlayerApiResponse, PopularPlayerEntry, PopularServerEntry, ServerApiResponse, ServerEdition } from '$lib/types';

const FALLBACK_API_BASE = 'http://127.0.0.1:3002';

export function getApiBase(): string {
  return env.PUBLIC_API_BASE || FALLBACK_API_BASE;
}

function extractError(payload: unknown, fallback: string): string {
  if (!payload || typeof payload !== 'object') {
    return fallback;
  }

  const typed = payload as ApiErrorResponse;
  return typed.message || typed.error || fallback;
}

export async function fetchServerSnapshot(
  fetchFn: typeof fetch,
  address: string,
  edition: ServerEdition
): Promise<{ data: ServerApiResponse | null; error: string | null; apiBase: string }> {
  const apiBase = getApiBase();
  const endpoint = `${apiBase}/api/v1/server/${encodeURIComponent(address)}?type=${encodeURIComponent(edition)}`;

  try {
    const response = await fetchFn(endpoint);
    const payload = await response.json().catch(() => null);

    if (!response.ok) {
      return {
        data: null,
        error: extractError(payload, `API server error (${response.status})`),
        apiBase
      };
    }

    return {
      data: payload as ServerApiResponse,
      error: null,
      apiBase
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unable to reach API server';
    return { data: null, error: message, apiBase };
  }
}

export async function fetchPlayerSnapshot(
  fetchFn: typeof fetch,
  username: string
): Promise<{ data: PlayerApiResponse | null; error: string | null; apiBase: string }> {
  const apiBase = getApiBase();
  const endpoint = `${apiBase}/api/v1/player/${encodeURIComponent(username)}`;

  try {
    const response = await fetchFn(endpoint);
    const payload = await response.json().catch(() => null);

    if (!response.ok) {
      return {
        data: null,
        error: extractError(payload, `API player error (${response.status})`),
        apiBase
      };
    }

    return {
      data: payload as PlayerApiResponse,
      error: null,
      apiBase
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unable to reach API server';
    return { data: null, error: message, apiBase };
  }
}

export async function fetchPopularPlayers(
  fetchFn: typeof fetch,
  sort: 'views' | 'likes' = 'views',
  limit = 20
): Promise<PopularPlayerEntry[]> {
  const apiBase = getApiBase();
  try {
    const res = await fetchFn(`${apiBase}/api/v1/popular/players?sort=${sort}&limit=${limit}`);
    if (!res.ok) return [];
    return (await res.json()) as PopularPlayerEntry[];
  } catch {
    return [];
  }
}

export async function fetchPopularServers(
  fetchFn: typeof fetch,
  sort: 'views' | 'likes' = 'views',
  limit = 20
): Promise<PopularServerEntry[]> {
  const apiBase = getApiBase();
  try {
    const res = await fetchFn(`${apiBase}/api/v1/popular/servers?sort=${sort}&limit=${limit}`);
    if (!res.ok) return [];
    return (await res.json()) as PopularServerEntry[];
  } catch {
    return [];
  }
}

export async function fetchRecentPlayers(
  fetchFn: typeof fetch,
  limit = 8
): Promise<PopularPlayerEntry[]> {
  const apiBase = getApiBase();
  try {
    const res = await fetchFn(`${apiBase}/api/v1/recent/players?limit=${limit}`);
    if (!res.ok) return [];
    return (await res.json()) as PopularPlayerEntry[];
  } catch {
    return [];
  }
}

export async function fetchRecentServers(
  fetchFn: typeof fetch,
  limit = 8
): Promise<PopularServerEntry[]> {
  const apiBase = getApiBase();
  try {
    const res = await fetchFn(`${apiBase}/api/v1/recent/servers?limit=${limit}`);
    if (!res.ok) return [];
    return (await res.json()) as PopularServerEntry[];
  } catch {
    return [];
  }
}
