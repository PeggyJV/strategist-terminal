<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { cellarId, blockHeight, chainId, deadline } from "$stores/scheduleRequestStore";
    import { queue, CellarCall } from "$stores/AdapterQueue";

    async function scheduleRequest() {
        let calls = $queue.map((call) => call.json_fields());

        const result = await invoke("schedule_request", {
            cellarId: $cellarId,
            blockHeight: $blockHeight,
            chainId: $chainId,
            deadline: $deadline,
            queue: calls,
        }).catch((error) => {
            console.error(error);
        });

        queue.set([]);
    }
</script>

<h1 class="text-2xl font-bold mb-4">Schedule Request</h1>

<div class="mb-4">
    <label for="cellar_id" class="block mb-1">Cellar ID:</label>
    <input
      type="text"
      id="cellar_id"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={$cellarId}
      placeholder="Enter Cellar ID"
    />
</div>

<div class="mb-4">
    <label for="block_height" class="block mb-1">Block Height:</label>
    <input
      type="text"
      id="block_height"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={$blockHeight}
      placeholder="Enter Block Height"
    />
</div>

<div class="mb-4">
    <label for="chain_id" class="block mb-1">Chain ID:</label>
    <input
      type="text"
      id="chain_id"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={$chainId}
      placeholder="Enter Chain ID"
    />
</div>

<div class="mb-4">
    <label for="deadline" class="block mb-1">Deadline:</label>
    <input
      type="text"
      id="deadline"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={$deadline}
      placeholder="Enter Deadline"
    />
</div>

<button
  class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
  on:click={scheduleRequest}
>
    Schedule Request
</button>
