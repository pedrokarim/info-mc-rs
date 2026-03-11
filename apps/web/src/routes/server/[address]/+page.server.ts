import type { PageServerLoad } from './$types';
import type { ServerEdition } from '$lib/types';
import { fetchServerSnapshot } from '$lib/server/mc-api';

function asEdition(value: string | null): ServerEdition {
  if (value === 'java' || value === 'bedrock') {
    return value;
  }
  return 'auto';
}

export const load: PageServerLoad = async ({ params, url, fetch }) => {
  const rawAddress = params.address;
  const address = decodeURIComponent(rawAddress);
  const edition = asEdition(url.searchParams.get('type'));

  const result = await fetchServerSnapshot(fetch, address, edition);

  return {
    address,
    edition,
    server: result.data,
    error: result.error
  };
};
