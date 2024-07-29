import { writable, type Writable } from "svelte/store";

export const cellarId: Writable<string> = writable("");
export const blockHeight: Writable<string> = writable("");
export const chainId: Writable<string> = writable("");
export const deadline: Writable<string> = writable("");
