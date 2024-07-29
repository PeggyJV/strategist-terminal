import { writable, type Writable } from "svelte/store";

export var queue: Writable<Array<CellarCall>> = writable([]);
export var flashLoanCalls: Writable<Array<CellarCall>> = writable([]);

export class CellarCall {
  adaptor: string;
  name: string;
  fields:  { [x: string]: Record<string, string>; } ;

  constructor(adaptor: string, name: string, fields: { [x: string]: Record<string, string>; }) {
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
