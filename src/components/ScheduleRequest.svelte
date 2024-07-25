<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { cellarId, blockHeight, chainId, deadline } from "$stores/scheduleRequestStore";
    import { queue, CellarCall } from "$stores/AdapterQueue";
    import StateModal from "../components/StateModal.svelte"; 

    let modalVisible = false;  
    let showTooltip = false; // State to manage tooltip visibility

    function toggleModal() {
        modalVisible = !modalVisible;
    }

    async function scheduleRequest() {
        let calls = $queue.map((call) => call.json_fields());
        const result = await invoke("schedule_request", {
            cellarId: $cellarId,
            blockHeight: $blockHeight,
            chainId: $chainId,
            deadline: $deadline,
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
    <label for="cellar_id" class="block mb-1">Cellar ID:</label>
    <input type="text" id="cellar_id" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$cellarId} placeholder="Enter Cellar ID"/>
</div>
<div class="mb-4">
    <label for="block_height" class="block mb-1">Block Height:</label>
    <input type="text" id="block_height" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$blockHeight} placeholder="Enter Block Height"/>
</div>
<div class="mb-4">
    <label for="chain_id" class="block mb-1">Chain ID:</label>
    <input type="text" id="chain_id" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$chainId} placeholder="Enter Chain ID"/>
</div>
<div class="mb-4">
    <label for="deadline" class="block mb-1">Deadline:</label>
    <input type="text" id="deadline" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={$deadline} placeholder="Enter Deadline"/>
</div>
<div class="group relative" on:mouseover={() => showTooltip = true} on:mouseout={() => showTooltip = false}>
    <button class="px-4 py-2 rounded-md focus:outline-none {isButtonEnabled ? 'bg-blue-500 text-white hover:bg-blue-600 focus:bg-blue-600' : 'bg-gray-400 text-gray-700 cursor-not-allowed'}" on:click={scheduleRequest} disabled={!isButtonEnabled}>
        Schedule Request
    </button>
    {#if !isButtonEnabled && showTooltip}
        <div class="absolute inset-x-0 bottom-full mb-2 px-2 py-1 bg-black text-white text-center rounded-md translate-x-[-50%]">
            Please fill all Schedule Request fields
        </div>
    {/if}
</div>
{#if modalVisible}
    <StateModal {toggleModal}/>
{/if}
