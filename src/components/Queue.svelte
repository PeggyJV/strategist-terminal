<script lang="ts">
  import { queue } from "$stores/AdapterQueue";

  function clearQueue() {
    queue.set([]);
  }

</script>
<div class="prose mt-10">
  <h2 class="text-2xl font-bold mb-4">Queue:</h2>
  <div class="max-w-4xl mx-auto">
    <div class="bg-white shadow-md rounded-lg overflow-hidden">

      <div class="px-4 py-2">
        {#each $queue as item}
          <div class="mb-4">
            <h2 class="text-xl font-bold not-prose">{item.adaptorName}</h2>
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
