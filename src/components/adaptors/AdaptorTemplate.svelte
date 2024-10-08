<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue";
  import { type Adaptor, type AdaptorCall, Functions } from "$lib/type"
  import { toast, ToastType } from "$stores/ToastStore"

  export let adaptor: Adaptor;

  // functionName: { fieldName: value }
  let fieldValues: Record<string, Record<string, any>> = {};

  $: {
    resetFieldValues();
  }

  function resetFieldValues() {
    fieldValues = {};
    const form = document.getElementById("form") as HTMLFormElement | null;
    form?.reset();
  }


  function handleInput(callFunction: string, fieldName: string, event: Event) {
    const target = event.target as HTMLInputElement;
    let value: string | number | boolean | [] | null   = target.value;

    if (target.type === 'number') {
      value = target.value ? Number(target.value) : null;
    } else if (target.type === 'checkbox') {
      value = target.checked;
    }

    if (!fieldValues[callFunction]) {
      fieldValues[callFunction] = {};
    }
    fieldValues[callFunction][fieldName] = value;
  }

  function addCallToQueue(call: AdaptorCall) {
    const relevantFields: Record<string, any> = {};

    call.fields.forEach(field => {
      if (fieldValues[call.function] && fieldValues[call.function][field.name]) {
        relevantFields[field.name] = fieldValues[call.function][field.name];
      }
      if(field.type === 'checkbox' && !fieldValues[call.function][field.name]) {
        relevantFields[field.name] = false;
      }
    });

    for (const fieldName of Object.keys(relevantFields)) {
      const field = call.fields
        .find((f) => f.name === fieldName);

      // If the fields type is array, create js array out of string
      if (field && field.type === 'array') {
        try {
          const value = relevantFields[fieldName];

          const parsedValue = JSON.parse(value);

          if (Array.isArray(parsedValue)) {
            relevantFields[fieldName] = parsedValue;
          } else {
            toast.set({
              type: ToastType.Error,
              description: `Error with array format on ${fieldName}.`
            });
            return;
          }
        } catch (error) {
          console.error("Error parsing array field", fieldName, error);
          toast.set({
            type: ToastType.Error,
            description: `Error parsing array on ${fieldName}. ${error}`
          });
          relevantFields[fieldName] = [];
          return;
        }
      }
    }

    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(
          Functions.CallOnAdaptor,
          { [call.function]: relevantFields },
          adaptor.address,
          adaptor.name
        )
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
        <div class="flex mt-2">
          <label for="{`${call.function}-${field.name}`}" class="w-[250px] mr-[70px]">{field.label}:</label>
          <input
            type={field.type ?? 'text'}
            value={fieldValues[call.function]?.[field.name] ?? ''}
            on:input={(event) => handleInput(call.function, field.name, event)}
            id="{`${call.function}-${field.name}`}"
            placeholder="{field.placeholder}"
            checked={false}
            class="px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500
                  {field.type === 'checkbox' ? 'w-[25px]' : 'w-[250px]'}"
          />
        </div>
      {/each}

      <button
        on:click={() => addCallToQueue(call)}
        type="button"
        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
      >{call.action}</button>
    {/each}
  </form>
</div>
