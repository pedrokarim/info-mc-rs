import type { PageServerLoad } from './$types';
import { fetchPopularPlayers, getPublicApiBase } from '$lib/server/mc-api';

export const load: PageServerLoad = async ({ fetch }) => {
  const popularPlayers = await fetchPopularPlayers(fetch, 'views', 20);
  return {
    popularPlayers,
    apiBase: getPublicApiBase()
  };
};
