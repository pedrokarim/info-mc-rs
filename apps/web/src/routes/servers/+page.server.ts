import type { PageServerLoad } from './$types';
import { fetchPopularServers } from '$lib/server/mc-api';

export const load: PageServerLoad = async ({ fetch }) => {
  const popularServers = await fetchPopularServers(fetch, 'views', 12);
  return { popularServers };
};
