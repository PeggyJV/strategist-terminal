<script lang="ts">
  import { CellarCall, flashLoanCalls } from "$stores/AdapterQueue"
  import { type Adaptor, type AdaptorCall, Functions } from "$lib/type"
  import { parseArrayField } from "$lib/utils"

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

    const target = event.target as HTMLInputElement;
    let value: string | number | boolean | [] | null   = target.value;

    if (target.type === 'number') {
      value = target.value ? Number(target.value) : null;
    } else if (target.type === 'checkbox') {
      value = target.checked;
    }

    if (!fieldValues[fieldName]) {
      fieldValues[fieldName] = {};
    }
    fieldValues[fieldName] = value;
  }

  function addCall(call: AdaptorCall) {

    // Assign false values to empty checkboxes
    call.fields.forEach((field) => {
      if (fieldValues[call.function] && fieldValues[field.name]) {
        fieldValues[field.name] = fieldValues[field.name];
      }
      if (field.type === "checkbox" && !fieldValues[field.name]) {
        fieldValues[field.name] = false;
      }
    });

    // Parse array fields
    for (const fieldName of Object.keys(fieldValues)) {

      const field = call.fields.find((f) => f.name === fieldName);

      if (field && field.type === "array") {
        const parsedValue = parseArrayField(fieldValues[fieldName], fieldName);

        if (parsedValue !== null) {
          fieldValues[fieldName] = parsedValue;
        } else {
          return;
        }
      }
    }

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
        type={field.type ?? 'text'}
        value={fieldValues[field.name] ?? ''}
        on:input={(event) => handleInput(field.name, event)}
        id={field.name}
        placeholder="{field.placeholder}"
        checked={false}
        class="px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500
                  {field.type === 'checkbox' ? 'w-[25px]' : 'w-[250px]'}"
      />
    </div>
  {/each}

  <button
    on:click={() => addCall(selectedCall)}
    class="p-2.5 border rounded focus:outline-none bg-blue-500 text-white border-gray-300"
  >Add call to flashloan</button>
{/if}
