import { writable, type Writable } from "svelte/store";

export const cellarId: Writable<String> = writable("");
export const blockHeight: Writable<String> = writable("");
export const chainId: Writable<String> = writable("");
export const deadline: Writable<String> = writable("");
