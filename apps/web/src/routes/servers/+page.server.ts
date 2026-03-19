import type { PageServerLoad } from './$types';
import { fetchPopularServers, fetchServerSnapshot } from '$lib/server/mc-api';

// All hardcoded server addresses from the page
const HARDCODED_ADDRESSES = [
  'play.hypixel.net', 'play.cubecraft.net', 'us.mineplex.com', 'geo.hivebedrock.network',
  'play.wynncraft.com', 'play.earthmc.net', '2b2t.org', 'minehut.com',
  'play.epicube.fr', 'play.funcraft.net', 'play.minestrator.com', 'minefight.fr',
];

export const load: PageServerLoad = async ({ fetch }) => {
  // Fetch popular servers + live data for hardcoded servers in parallel
  const [allStored, ...liveResults] = await Promise.all([
    fetchPopularServers(fetch, 'views', 100),
    ...HARDCODED_ADDRESSES.map(addr => fetchServerSnapshot(fetch, addr, 'auto')),
  ]);

  const popularServers = allStored.slice(0, 12);

  // Build motd map from stored + live data
  const storedMap: Record<string, { motd_html?: string }> = {};

  // From popular/stored DB entries
  for (const s of allStored) {
    if (s.motd_html) {
      storedMap[s.address.toLowerCase()] = { motd_html: s.motd_html };
      storedMap[s.hostname.toLowerCase()] = { motd_html: s.motd_html };
    }
  }

  // From live fetches (has fresh motd.html with colors)
  for (let i = 0; i < HARDCODED_ADDRESSES.length; i++) {
    const addr = HARDCODED_ADDRESSES[i].toLowerCase();
    const result = liveResults[i];
    if (result.data?.motd?.html) {
      storedMap[addr] = { motd_html: result.data.motd.html };
    }
  }

  return { popularServers, storedMap };
};
