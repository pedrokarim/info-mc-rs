import type { RequestHandler } from './$types';
import { fetchPopularPlayers, fetchPopularServers } from '$lib/server/mc-api';

const SITE_URL = 'https://mcinfo.ascencia.re';

const STATIC_PAGES = [
  { path: '/', priority: '1.0', changefreq: 'daily' },
  { path: '/skins', priority: '0.9', changefreq: 'daily' },
  { path: '/servers', priority: '0.9', changefreq: 'daily' },
];

export const GET: RequestHandler = async ({ fetch }) => {
  const [players, servers] = await Promise.all([
    fetchPopularPlayers(fetch, 'views', 50),
    fetchPopularServers(fetch, 'views', 50),
  ]);

  const now = new Date().toISOString().split('T')[0];

  const urls = [
    ...STATIC_PAGES.map(
      (p) => `  <url>
    <loc>${SITE_URL}${p.path}</loc>
    <lastmod>${now}</lastmod>
    <changefreq>${p.changefreq}</changefreq>
    <priority>${p.priority}</priority>
  </url>`
    ),
    ...players.map(
      (p) => `  <url>
    <loc>${SITE_URL}/player/${encodeURIComponent(p.username)}</loc>
    <lastmod>${now}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.7</priority>
  </url>`
    ),
    ...servers.map(
      (s) => `  <url>
    <loc>${SITE_URL}/server/${encodeURIComponent(s.address)}</loc>
    <lastmod>${now}</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.8</priority>
  </url>`
    ),
  ];

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.join('\n')}
</urlset>`;

  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'max-age=3600',
    },
  });
};
