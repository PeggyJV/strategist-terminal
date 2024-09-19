import { writable, type Writable } from "svelte/store";
import type { Functions } from "$lib/type"

export var queue: Writable<Array<CellarCall>> = writable([]);
export var flashLoanCalls: Writable<Array<CellarCall>> = writable([]);

export class CellarCall {
  adaptorAddress?: string;
  adaptorName?: string;
  fields: any;
  functionName: string;

  constructor(functionName: Functions, fields: any, adaptorAddress?: string, adaptorName?: string) {
    this.adaptorAddress = adaptorAddress;
    this.adaptorName = adaptorName;
    this.fields = fields;
    this.functionName = functionName;
  }

  json_fields(): any {
    return  {
        adaptor: this.adaptorAddress,
        name: this.adaptorName,
        function_name: this.functionName,
        fields: JSON.stringify(this.fields)
    };
  }
}
