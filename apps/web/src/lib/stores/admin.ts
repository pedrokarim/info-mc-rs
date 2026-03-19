import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

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

export function saveSession(session: AdminSession) {
  if (browser) localStorage.setItem(STORAGE_KEY, JSON.stringify(session));
  adminSession.set(session);
}

export function clearSession() {
  if (browser) localStorage.removeItem(STORAGE_KEY);
  adminSession.set(null);
}

export function adminFetch(apiBase: string, path: string, token: string, options: RequestInit = {}) {
  return fetch(`${apiBase}${path}`, {
    ...options,
    headers: {
      ...options.headers as Record<string, string>,
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
}
