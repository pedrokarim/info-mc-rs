import type { PageServerLoad } from './$types';
import { fetchPlayerSnapshot, getPublicApiBase } from '$lib/server/mc-api';

export const load: PageServerLoad = async ({ params, fetch }) => {
  const username = decodeURIComponent(params.username);
  const result = await fetchPlayerSnapshot(fetch, username);

  return {
    username,
    player: result.data,
    error: result.error,
    apiBase: getPublicApiBase()
  };
};
