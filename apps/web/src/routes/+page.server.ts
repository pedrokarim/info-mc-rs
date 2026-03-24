import type { PageServerLoad } from './$types';
import { fetchPopularPlayers, fetchPopularServers, fetchServerSnapshot, getPublicApiBase } from '$lib/server/mc-api';

const SPOTLIGHT_ADDRESSES = [
  'play.hypixel.net', 'play.cubecraft.net', 'play.wynncraft.com',
  'play.epicube.fr', 'play.funcraft.net', '2b2t.org',
];

export const load: PageServerLoad = async ({ fetch }) => {
  const [popularPlayers, popularServers, ...liveResults] = await Promise.all([
    fetchPopularPlayers(fetch, 'views', 8),
    fetchPopularServers(fetch, 'views', 6),
    ...SPOTLIGHT_ADDRESSES.map(addr => fetchServerSnapshot(fetch, addr, 'auto')),
  ]);

  // Build motd map from live fetches
  const motdMap: Record<string, string> = {};
  for (let i = 0; i < SPOTLIGHT_ADDRESSES.length; i++) {
    const addr = SPOTLIGHT_ADDRESSES[i].toLowerCase();
    const result = liveResults[i];
    if (result.data?.motd?.html) {
      motdMap[addr] = result.data.motd.html;
    }
  }

  // Also add motd from popular servers DB entries
  for (const s of popularServers) {
    const key = s.address.toLowerCase();
    if (!motdMap[key] && s.motd_html) {
      motdMap[key] = s.motd_html;
    }
  }

  return {
    popularPlayers,
    popularServers,
    motdMap,
    apiBase: getPublicApiBase(),
  };
};
