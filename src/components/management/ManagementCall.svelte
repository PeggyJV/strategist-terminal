<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Chains } from "$lib/cellars"
  import { CellarCall } from "$stores/AdapterQueue"
  import { toast, ToastType } from "$stores/ToastStore"

  let cellarAddress = "";
  let blockHeight = "";

  let sendingRequest = false;

  let chain = Chains.ETHEREUM;

  export let toggleCallModal: () => void;
  export let call: CellarCall;

  async function handleCall() {
    sendingRequest = true;
    const deadlineDate = new Date();
    const deadlineUnixTimestamp = Math.floor(deadlineDate.getTime() / 1000) || 1;

    const result = await invoke("schedule_request", {
      cellarId: cellarAddress,
      blockHeight: blockHeight,
      chainId: chain.chainId,
      deadline: deadlineUnixTimestamp.toString(),
      queue: [ call.json_fields() ],
    }).then(result => {
      console.log('Schedule successful', result);
      toast.set(
        {
          type: ToastType.Success,
          description: `${call.functionName} call successfully sent`
        }
      );
    }).catch((error) => {
      console.error(error);
      toast.set(
        {
          type: ToastType.Error,
          description: `Error sending ${call.functionName} call: ${error}`
        }
      );
    });
    sendingRequest = false;
    toggleCallModal();
  }


  $: isButtonEnabled = blockHeight.trim().length > 0 && !isNaN(parseInt(blockHeight))
</script>

<div class="fixed inset-0 z-10 w-screen overflow-y-auto bg-gray-500 bg-opacity-75 transition-opacity" aria-labelledby="modal-title" role="dialog" aria-modal="true">
  <div class="flex min-h-full items-center justify-center p-4 text-center sm:p-0">
    <div class="absolute transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8">
      <!-- Close button -->
      <div class="absolute top-2 right-4">
        <button
          class="absolute top-2 right-2 text-xl font-bold text-gray-400 hover:text-gray-500"
          on:click={toggleCallModal}
        >
          &times;
        </button>
      </div>
      <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4 min-w-[500px]">
        <div class="sm:flex justify-center sm:items-center">
          <div class="mt-2 text-center sm:text-left min-w-[350px]">
            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Management</h3>
            <div class="mt-2 mb-3">

              <h1 class="text-2xl font-bold mb-4">{call.functionName} Call</h1>

               Input data:
                <pre class="mt-1 text-gray-700 bg-gray-100 pl-[70px]">
                  {JSON.stringify(call.fields, null, 2)}
                </pre>


                <div class="mb-4">
                  <label for="chain" class="block mb-1">Chain:</label>
                  <select name="chain" id="chain" bind:value={chain} class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500">
                    {#each Object.entries(Chains) as [key, value]}
                      <option value={value}>{key}</option>
                    {/each}
                  </select>
                </div>
                <div class="mb-4">
                  <label for="address" class="block mb-1">Cellar Address:</label>
                  <input type="text" id="address" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={cellarAddress} placeholder="Enter Cellar Address"/>
                </div>

              <div class="mb-4">
                <label for="block_height" class="block mb-1">Block Height:</label>
                <input type="text" id="block_height" class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500" bind:value={blockHeight} placeholder="Enter Block Height"/>
              </div>

              <div class="flex flex-row">
                {#if sendingRequest}
                  Broadcasting request
                  <div id="loading-spinner" class="w-8 h-8 border-4 border-gray-200 border-t-blue-500 ml-2 rounded-full animate-spin"></div>
                {:else}
                  <button
                    on:click={handleCall}
                    disabled={!isButtonEnabled}
                    class="px-4 py-2 rounded-md focus:outline-none {isButtonEnabled ? 'bg-blue-500 text-white hover:bg-blue-600 focus:bg-blue-600' : 'bg-gray-400 text-gray-700 cursor-not-allowed'}"
                  >
                    Schedule Request
                  </button>
                {/if}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
