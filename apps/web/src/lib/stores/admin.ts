import { writable, derived } from 'svelte/store';
import { env } from '$env/dynamic/public';
import { identifyAdmin, resetIdentity } from '$lib/analytics';

export const API_BASE = env.PUBLIC_API_BASE || '';

export interface AdminUser {
	account_id: string;
	email: string | null;
	display_name: string | null;
	username: string | null;
	avatar_url: string | null;
	role: 'admin' | 'super_admin';
}

export interface AdminSession {
	authenticated: true;
	user: AdminUser;
}

export const adminSession = writable<AdminSession | null>(null);
export const adminSessionReady = writable(false);
export const isAdmin = derived(adminSession, (session) => session !== null);
export const adminUser = derived(adminSession, (session) => session?.user ?? null);

/** Recharge la session HttpOnly sans exposer de jeton au JavaScript. */
export async function loadAdminSession(): Promise<AdminSession | null> {
	try {
		const response = await adminFetch('/api/v1/admin/auth/me');
		if (!response.ok) {
			adminSession.set(null);
			return null;
		}
		const session = (await response.json()) as AdminSession;
		adminSession.set(session);
		identifyAdmin(session.user.account_id);
		return session;
	} catch {
		adminSession.set(null);
		return null;
	} finally {
		adminSessionReady.set(true);
	}
}

export function clearSession(): void {
	adminSession.set(null);
	adminSessionReady.set(true);
	resetIdentity();
}

export function adminFetch(path: string, options: RequestInit = {}): Promise<Response> {
	const headers = new Headers(options.headers);
	if (options.body && !headers.has('Content-Type')) {
		headers.set('Content-Type', 'application/json');
	}
	return fetch(`${API_BASE}${path}`, {
		...options,
		headers,
		credentials: 'include'
	});
}
