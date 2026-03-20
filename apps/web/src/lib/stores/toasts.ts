import { writable } from 'svelte/store';

export type ToastVariant = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: number;
  message: string;
  variant: ToastVariant;
  duration: number;
}

let nextId = 0;

export const toasts = writable<Toast[]>([]);

export function addToast(
  message: string,
  variant: ToastVariant = 'info',
  duration = 4000,
): number {
  const id = nextId++;
  toasts.update((t) => [...t, { id, message, variant, duration }]);

  if (duration > 0) {
    setTimeout(() => removeToast(id), duration);
  }

  return id;
}

export function removeToast(id: number) {
  toasts.update((t) => t.filter((toast) => toast.id !== id));
}
