<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  import type { Adaptor } from "$lib/adaptorList"

  export let  adaptor: Adaptor

  let fieldValues: Record<string, string> = {};

  function scheduleCall(functionName: string, fields: Record<string, string>) {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(adaptor.address, adaptor.name, {
          [functionName]: fields,
        })
      );
      return callQueue;
    });
    fieldValues = {}
  }
</script>
<div class="prose mt-10 w-screen">
  <h1>{adaptor.name}</h1>

  {#each adaptor.calls as call, index}
    <h2>{index + 1}. {call.function}</h2>

    {#each call.fields as field}
      <div class="flex justify-between mt-2">
        <label
          for="{field.name}"
        >{field.label}:</label>
        <input
          bind:value={fieldValues[field.name]}
          id="{field.name}"
          placeholder="{field.placeholder}"
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
    {/each}

    <button
      on:click={() => scheduleCall(call.function, fieldValues)}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
    >{call.action}</button>
  {/each}
</div>

