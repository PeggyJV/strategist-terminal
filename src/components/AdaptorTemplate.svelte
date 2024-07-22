<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue";
  import type { Adaptor, AdaptorCall } from "$lib/adaptorList";

  export let adaptor: Adaptor;

  let adaptorAddress = adaptor.address;

  $: adaptorAddress = adaptor.address;

  // functionName: { fieldName: value }
  let fieldValues: Record<string, Record<string, string>> = {};

  function getFieldValue(callFunction: string, fieldName: string): string {
    return fieldValues[callFunction]?.[fieldName] || '';
  }

  function setFieldValue(callFunction: string, fieldName: string, value: string) {
    if (!fieldValues[callFunction]) {
      fieldValues[callFunction] = {};
    }
    fieldValues[callFunction][fieldName] = value;
  }

  function handleInput(callFunction: string, fieldName: string, event: Event) {
    const input = event.target as HTMLInputElement;
    setFieldValue(callFunction, fieldName, input.value);
  }

  function scheduleCall(call: AdaptorCall) {
    const relevantFields: Record<string, string> = {};

    call.fields.forEach(field => {
      if (fieldValues[call.function] && fieldValues[call.function][field.name]) {
        relevantFields[field.name] = fieldValues[call.function][field.name];
      }
    });

    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(adaptor.address, adaptor.name, {
          [call.function]: relevantFields,
        })
      );
      return callQueue;
    });
    fieldValues[call.function] = {};
  }
</script>

<div class="prose mt-10 w-screen">
  <h1>{adaptor.name}</h1>
  <label for="adaptorAddress">Adaptor address:</label>
  <input
    bind:value={adaptorAddress}
    id="adaptorAddress"
    placeholder={adaptorAddress}
    class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
  />

  {#each adaptor.calls as call, index}
    <h2>{index + 1}. {call.function}</h2>

    {#each call.fields as field}
      <div class="flex justify-between mt-2">
        <label for="{field.name}">{field.label}:</label>
        <input
          value={getFieldValue(call.function, field.name)}
          on:input={(event) => handleInput(call.function, field.name, event)}
          id="{field.name}"
          placeholder="{field.placeholder}"
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
    {/each}

    <button
      on:click={() => scheduleCall(call)}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
    >{call.action}</button>
  {/each}
</div>
