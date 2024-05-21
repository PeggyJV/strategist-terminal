import { writable, type Writable } from "svelte/store";

export var queue: Writable<Array<CellarCall>> = writable([]);

export class CellarCall {
  [name: string]: any;
  constructor(name: string, fields: any) {
    this[name] = fields;
  }
}
