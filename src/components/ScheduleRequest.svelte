<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { cellarId, blockHeight, chainId, deadline } from "$stores/scheduleRequestStore";
    import { queue } from "$stores/AdapterQueue";
    import StateModal from "../components/StateModal.svelte";
    import Cellars from "$lib/cellars"

    let modalVisible = false;  
    let showTooltip = false;

    const chainIdMap = {
        Ethereum: "1",
        Arbitrum: "42161",
        Optimism: "10",
        Scroll: "534353"
    }

    function toggleModal() {
        modalVisible = !modalVisible;
    }

    async function scheduleRequest() {
        let calls = $queue.map((call) => call.json_fields());

        const deadlineDate = new Date($deadline);
        const deadlineUnixTimestamp = Math.floor(deadlineDate.getTime() / 1000);

        const result = await invoke("schedule_request", {
            cellarId: $cellarId,
            blockHeight: $blockHeight,
            chainId: $chainId,
            deadline: deadlineUnixTimestamp,
            queue: calls,
        }).then(result => {
            console.log('Schedule successful', result);
            toggleModal();  
        }).catch((error) => {
            console.error(error);
            toggleModal();  
        });
        queue.set([]);
    }

    $: isButtonEnabled = $cellarId.trim().length > 0 && 
                         $blockHeight.trim().length > 0 &&
                         $chainId.trim().length > 0 &&
                         $deadline.trim().length > 0;
</script>

<h1 class="text-2xl font-bold mb-4">Schedule Request</h1>
<div class="mb-4">
    <label for="cellar_id" class="block mb-1">Cellar:</label>

    <select name="cellar_id" id="cellar_id" bind:value={$cellarId} class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500">
        {#each Object.entries(Cellars) as [key, value]}
            <option value={value.ADDRESS}>{key}</option>
        {/each}
    </select>
 </div>
<div class="mb-4">
    <label for="block_height" class="block mb-1">Block Height:</label>
    <input type="text" id="block_height" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$blockHeight} placeholder="Enter Block Height"/>
</div>
<div class="mb-4">

    <label for="chain_id" class="block mb-1">Chain:</label>

    <select name="chain_id" id="chain_id" bind:value={$chainId} class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500">
        {#each Object.entries(chainIdMap) as [key, value]}
            <option value={value}>{key}</option>
        {/each}
    </select>
</div>

{#if $chainId !== "1"}
    <div class="mb-4">
        <label for="deadline" class="block mb-1">Deadline:</label>
        <input type="datetime-local" id="deadline" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$deadline} placeholder="Enter Deadline"/>
    </div>
{/if}

<div class="relative">
    <button
      on:click={scheduleRequest}
      disabled={!isButtonEnabled}
      on:focus={() => { if (!isButtonEnabled) showTooltip = true }}
      on:blur={() => { if (!isButtonEnabled) showTooltip = false }}
      class="px-4 py-2 rounded-md focus:outline-none {isButtonEnabled ? 'bg-blue-500 text-white hover:bg-blue-600 focus:bg-blue-600' : 'bg-gray-400 text-gray-700 cursor-not-allowed'}"
    >
        Schedule Request
    </button>
    {#if !isButtonEnabled && showTooltip}
        <div
          class="absolute inset-x-0 bottom-full mb-2 px-2 py-1 bg-black text-white text-center rounded-md translate-x-[-50%]"
          role="tooltip"
        >
            Please fill all Schedule Request fields
        </div>
    {/if}
    <button
      on:click={toggleModal}
      class="px-4 py-2 mt-5 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
    >
        Track Schedule ID: 1234
    </button>
</div>
{#if modalVisible}
    <StateModal {toggleModal}/>
{/if}
