import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import { env } from '$env/dynamic/public';
import { identifyAdmin, resetIdentity } from '$lib/analytics';

// Empty = relative URLs (proxied via SvelteKit hooks), fallback for local dev
export const API_BASE = env.PUBLIC_API_BASE || '';

export interface AdminUser {
  discord_id: string;
  discord_username: string;
  discord_avatar: string | null;
  role: string;
}

export interface AdminSession {
  token: string;
  expires_at: string;
  user: AdminUser;
}

const STORAGE_KEY = 'mcinfo_admin_session';

function loadSession(): AdminSession | null {
  if (!browser) return null;
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    const session = JSON.parse(raw) as AdminSession;
    if (new Date(session.expires_at) < new Date()) {
      localStorage.removeItem(STORAGE_KEY);
      return null;
    }
    return session;
  } catch {
    return null;
  }
}

export const adminSession = writable<AdminSession | null>(loadSession());

export const isAdmin = derived(adminSession, ($s) => $s !== null);
export const adminUser = derived(adminSession, ($s) => $s?.user ?? null);
export const adminToken = derived(adminSession, ($s) => $s?.token ?? null);

// L'identification passe par ici, et non par l'écran de connexion : celui-ci a
// deux chemins de succès — avec et sans 2FA — et la déconnexion vit ailleurs
// encore. Trois points d'appel à tenir alignés, contre un seul si on branche
// l'endroit par lequel une session commence et finit forcément.
export function saveSession(session: AdminSession) {
  if (browser) localStorage.setItem(STORAGE_KEY, JSON.stringify(session));
  adminSession.set(session);
  // Seul l'identifiant du compte : ni pseudo ni avatar. Voir `$lib/analytics`.
  identifyAdmin(session.user.discord_id);
}

export function clearSession() {
  if (browser) localStorage.removeItem(STORAGE_KEY);
  adminSession.set(null);
  // Sur un poste partagé, sans ça, la navigation du visiteur suivant serait
  // attribuée à l'administrateur qui vient de partir.
  resetIdentity();
}

export function adminFetch(path: string, token: string, options: RequestInit = {}) {
  const headers = new Headers(options.headers);
  headers.set('Authorization', `Bearer ${token}`);
  if (!headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json');
  }
  return fetch(`${API_BASE}${path}`, { ...options, headers });
}
