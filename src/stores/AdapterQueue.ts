import { writable, type Writable } from "svelte/store";

export var queue: Writable<Array<CellarCall>> = writable([]);

export class CellarCall {
  address: String;
  name: String;
  fields: any;
  constructor(address: String, name: String, fields: any) {
    this.address = address;
    this.fields = fields;
    this.name = name;
  }
}
