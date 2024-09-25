<script lang="ts">
  import type { Adaptor, AdaptorCall } from "$lib/adaptorList"
  import { CellarCall, flashLoanCalls } from "$stores/AdapterQueue"
  import { Functions } from "$lib/type"

  export let adaptor: Adaptor;
  export let closeAdaptorSelection: () => void;

  let fieldValues: Record<string, any> = {};

  let selectedCall: AdaptorCall;
  let callListVisible = true;

  function selectCall(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    selectedCall = adaptor.calls.find(c => c.function === target.innerText) ?? adaptor.calls[0];
    callListVisible = false;
  }

  function handleInput(fieldName: string, event: Event) {

    const input = event.target as HTMLInputElement;

    fieldValues[fieldName] = input.value;
  }

  function addCall() {
    flashLoanCalls.update((callQueue) => {
      callQueue.push(
        new CellarCall(
          Functions.CallOnAdaptor,
          { [selectedCall.function]: fieldValues, },
          adaptor.address,
          adaptor.name
        )
      );
      return callQueue;
    });
    fieldValues = {};
    closeAdaptorSelection();
  }
</script>

{#if callListVisible}
  <h2>Select adaptor call:</h2>
  <div class="flex flex-wrap gap-2.5 mt-12 justify-center">
    {#each adaptor.calls as call}
      <button
        on:click={selectCall}
        class="p-2.5 border rounded focus:outline-none bg-gray-100 text-black border-gray-300"
      >{call.function}</button>
    {/each}
  </div>
{/if}

{#if selectedCall}

  <label for="adaptorAddress">{adaptor.name} Adaptor address:</label>
  <input
    bind:value={adaptor.address}
    id="adaptorAddress"
    class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
  />

  <h2>{selectedCall.function}</h2>

  {#each selectedCall.fields as field}
    <div class="flex justify-between mt-2">
      <label for={field.name}>{field.label}:</label>
      <input
        value={fieldValues[field.name] ?? ''}
        on:input={(event) => handleInput(field.name, event)}
        id={field.name}
        placeholder="{field.placeholder}"
        class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      />
    </div>
  {/each}

  <button
    on:click={addCall}
    class="p-2.5 border rounded focus:outline-none bg-gray-100 text-black border-gray-300"
  >Add call</button>
{/if}
