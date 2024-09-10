import { writable } from 'svelte/store';
export enum ToastType {
  Error,
  Success,
  Info
}
export interface ToastData {
  type: ToastType,
  description: string
}

export const toast = writable<ToastData | null>(null);
