<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue"

  function clearQueue() {
    queue.set([]);
  }
  function removeCall(index: number) {
    queue.update((calls: CellarCall[]) => {
      return calls.filter((_, i) => i !== index);
    });
  }

</script>
<div class="prose mt-10">
  <h2 class="text-2xl font-bold mb-4">Queue:</h2>
  <div class="max-w-4xl min-w-[250px] mx-auto">
    <div class="bg-white shadow-md rounded-lg overflow-hidden">

      <div class="px-4 py-2">
        {#each $queue as item, index}
          <div class="mb-4">
            <div class="flex flex-row">
              <h2 class="text-xl font-bold mt-2 not-prose">{item.adaptorName}</h2>
              <button
                on:click={() => removeCall(index)}
                type="button"
                class="px-1 text-red-500 text-3xl rounded-md hover:text-red-600 "
              > &times;</button>
            </div>

            {#each Object.entries(item.fields) as [key, value]}
              <div class="mb-2">
                <p class="font-bold">{key}:</p>
                <pre class="mt-1 text-gray-700 bg-gray-100">
                  {JSON.stringify(value, null, 2)}
                </pre>
              </div>
            {/each}

          </div>
        {/each}
      </div>

      <button class="block w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-b-lg focus:outline-none" on:click={clearQueue}>Clear Queue</button>
    </div>
  </div>
</div>
