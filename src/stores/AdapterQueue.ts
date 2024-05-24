import { writable, type Writable } from "svelte/store";

export var queue: Writable<Array<CellarCall>> = writable([]);

export class CellarCall {
  adaptor: string;
  name: string;
  fields: any;

  constructor(adaptor: string, name: string, fields: any) {
    this.adaptor = adaptor;
    this.name = name;
    this.fields = fields;
  }

  json_fields(): any {
    return  {
        adaptor: this.adaptor,
        name: this.name,
        fields: JSON.stringify(this.fields)
    };
  }
}
