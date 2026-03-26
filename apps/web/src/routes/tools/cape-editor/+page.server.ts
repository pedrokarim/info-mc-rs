import type { PageServerLoad } from './$types';
import { getPublicApiBase } from '$lib/server/mc-api';

export const load: PageServerLoad = async () => {
  return { apiBase: getPublicApiBase() };
};
