<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue";
  import type { Adaptor, AdaptorCall } from "$lib/adaptorList";

  export let adaptor: Adaptor;

  // functionName: { fieldName: value }
  let fieldValues: Record<string, Record<string, string>> = {};

  $: {
    resetFieldValues();
  }

  function resetFieldValues() {
    fieldValues = {};
    const form = document.getElementById("form") as HTMLFormElement | null;
    form?.reset();
  }


  function handleInput(callFunction: string, fieldName: string, event: Event) {
    const input = event.target as HTMLInputElement;
    if (!fieldValues[callFunction]) {
      fieldValues[callFunction] = {};
    }
    fieldValues[callFunction][fieldName] = input.value;
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
  <div class="flex justify-center">
    <h1>{adaptor.name}</h1>
  </div>
  <label for="adaptorAddress">Adaptor address:</label>
  <input
    bind:value={adaptor.address}
    id="adaptorAddress"
    class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
  />

  <form id="form">
    {#each adaptor.calls as call, index}
      <h2>{index + 1}. {call.function}</h2>

      {#each call.fields as field}
        <div class="flex justify-between mt-2">
          <label for="{`${call.function}-${field.name}`}">{field.label}:</label>
          <input
            value={fieldValues[call.function]?.[field.name] || ''}
            on:input={(event) => handleInput(call.function, field.name, event)}
            id="{`${call.function}-${field.name}`}"
            placeholder="{field.placeholder}"
            class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
          />
        </div>
      {/each}

      <button
        on:click={() => scheduleCall(call)}
        type="button"
        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
      >{call.action}</button>
    {/each}
  </form>
</div>
