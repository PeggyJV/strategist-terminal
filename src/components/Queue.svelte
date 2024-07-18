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
            <h2 class="text-xl font-bold not-prose">{item.name}</h2>
            {#each Object.entries(item.fields) as [key, value]}
              <p class="mt-2"><span class="font-bold">{key}:</span> {JSON.stringify(value)}</p>
            {/each}
          </div>
        {/each}
      </div>

      <button class="block w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-b-lg focus:outline-none" on:click={clearQueue}>Clear Queue</button>
    </div>
  </div>
</div>
