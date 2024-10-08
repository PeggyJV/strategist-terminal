<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { flashLoanCalls, queue } from "$stores/AdapterQueue"
    import StateModal from "./requests/StateModal.svelte";
    import Cellars, { type Cellar, Chains } from "$lib/cellars"
    import { type RequestState } from "$lib/type"
    import { toast, ToastType } from "$stores/ToastStore"

    let modalVisible = false;  
    let showTooltip = false;

    let cellar: Cellar = Cellars.REAL_YIELD_ETH;
    let blockHeight = "";
    let deadline = "";

    let request: RequestState;

    function toggleStatesModal() {
        modalVisible = !modalVisible;
    }

    function handleSchedule() {
        let firstCall = $queue[0].adaptorName;
        if (firstCall === "AaveV3DebtTokenV1FlashLoan"
          || firstCall === "BalancerPoolV1FlashLoan") {
            scheduleFlashLoanCall();
        } else {
            scheduleAdaptorCall();
        }
    }

    async function scheduleAdaptorCall() {
        let calls = $queue.map((call) => call.json_fields());

        const deadlineDate = new Date(deadline);
        const deadlineUnixTimestamp = Math.floor(deadlineDate.getTime() / 1000) || 1;

        const result = await invoke("schedule_request", {
            cellarId: cellar.ADDRESS,
            blockHeight: blockHeight,
            chainId: cellar.CHAIN.chainId,
            deadline: deadlineUnixTimestamp.toString(),
            queue: calls,
        }).then(result => {
            console.log('Schedule successful', result);
            toggleStatesModal();
        }).catch((error) => {
            console.error(error);
            toast.set({
                type: ToastType.Error,
                description: `Error scheduling the request. ${error}`
            })
        });
        queue.set([]);
        // TODO: Create a request object from the result
    }

    async function scheduleFlashLoanCall() {
        let calls = $queue.map((call) => call.json_fields());
        let params = $flashLoanCalls.map((call) => call.json_fields());

        const deadlineDate = new Date(deadline);
        const deadlineUnixTimestamp = Math.floor(deadlineDate.getTime() / 1000) || 1;

        const result = await invoke("schedule_request", {
            cellarId: cellar.ADDRESS,
            blockHeight: blockHeight,
            chainId: cellar.CHAIN.chainId,
            deadline: deadlineUnixTimestamp.toString(),
            flashLoanCall: calls[0],
            queue: params,
        }).then(result => {
            console.log('Schedule successful', result);
            toggleStatesModal();
        }).catch((error) => {
            console.error(error);
            toast.set(
              {
                  type: ToastType.Error,
                  description: "Error scheduling a request: " + error
              }
            );
        });
        queue.set([]);
        // TODO: Create a request object from the result
    }

    $: isButtonEnabled = blockHeight.trim().length > 0
      && $queue.length > 0
      && !isNaN(parseInt(blockHeight))
</script>

<h1 class="text-2xl font-bold mb-4">Schedule Request</h1>
<div class="mb-4">
    <label for="cellar_id" class="block mb-1">Cellar:</label>

    <select name="cellar_id" id="cellar_id" bind:value={cellar} class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500">
        {#each Object.entries(Cellars) as [key, value]}
            <option value={value}>{key}</option>
        {/each}
    </select>
 </div>
{#if cellar === Cellars.CUSTOM}
    <div class="mb-4">
        <label for="chain" class="block mb-1">Chain:</label>
        <select name="chain" id="chain" bind:value={cellar.CHAIN} class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500">
            {#each Object.entries(Chains) as [key, value]}
                <option value={value}>{key}</option>
            {/each}
        </select>
    </div>
    <div class="mb-4">
        <label for="address" class="block mb-1">Address:</label>
        <input type="text" id="address" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={cellar.ADDRESS} placeholder="Enter Cellar Address"/>
    </div>
{/if}
<div class="mb-4">
    <label for="block_height" class="block mb-1">Block Height:</label>
    <input type="text" id="block_height" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={blockHeight} placeholder="Enter Block Height"/>
</div>

<!--{#if cellar.CHAIN !== Chains.ETHEREUM}-->
    <div class="mb-4">
        <label for="deadline" class="block mb-1">Deadline:</label>
        <input type="datetime-local" id="deadline" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={deadline} placeholder="Enter Deadline"/>
    </div>
<!--{/if}-->

<div class="relative">
    <button
      on:click={handleSchedule}
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
      on:click={toggleStatesModal}
      class="px-4 py-2 mt-5 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
    >
        Track Schedule ID: 1234
    </button>
</div>
{#if modalVisible}
    <StateModal {toggleStatesModal} {request}/>
{/if}
