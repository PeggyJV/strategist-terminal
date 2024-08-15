<script lang="ts">
  import AdaptorSelection from "./AdaptorSelection.svelte"
  import { CellarCall, flashLoanCalls, queue } from "$stores/AdapterQueue"

  let adaptorSelectionOpen = false;
  let addCallBtnVisible = true;

  let adaptorAddress = "";

  let tokens = "";
  let amounts = "";

  function openAdaptorSelection() {
    adaptorSelectionOpen = true;
    addCallBtnVisible = false;
  }

  function closeAdaptorSelection() {
    adaptorSelectionOpen = false;
    addCallBtnVisible = true;
  }

  function removeCall(index: number): void {
    flashLoanCalls.update((calls: CellarCall[]) => {
      return calls.filter((_, i) => i !== index);
    });
  }

  function requestFlashLoan(): void {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(adaptorAddress, "AaveV3DebtTokenV1FlashLoan", {
          FlashLoan: {
            loan_tokens: tokens,
            loan_amounts: amounts,
            params: $flashLoanCalls
          },
        })
      );
      return callQueue;
    });
  }
</script>

<div class="prose">
  <div class="flex  justify-center">
    <h1>Flash Loans</h1>
  </div>

  <label for="adaptorAddress">FlashLoan Adaptor address:</label>
  <input
    bind:value={adaptorAddress}
    id="adaptorAddress"
    placeholder={adaptorAddress}
    class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
  />

  <div class="flex justify-between mt-2">
    <label for="tokens">Enter the loan tokens, as array of strings:</label>
    <input
      bind:value={tokens}
      id="tokens"
      placeholder="e.g., [0x000..., 0x000...]"
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />
  </div>

  <div class="flex justify-between mt-2">
    <label for="amounts">Enter the loan amounts, as array of strings:</label>
    <input
      bind:value={amounts}
      id="amounts"
      placeholder="e.g., [50, 100]"
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />
  </div>

  <h2>Internal Calls</h2>

  {#if $flashLoanCalls.length === 0 && addCallBtnVisible}
    <div>
      No calls added yet!
    </div>
  {/if}

  {#each $flashLoanCalls as call, index (index)}
        <h4>{call.name}</h4>
        {#each Object.entries(call.fields) as [key, value]}
          <div class="flex justify-between mt-2 ml-5">
            <div>
              {key}: {JSON.stringify(value)}
            </div>
            <button
              on:click={() => removeCall(index)}
              type="button"
              class="px-2 py-1 bg-red-500 text-white rounded-md hover:bg-red-600 focus:outline-none focus:bg-red-600"
            >Remove</button>
          </div>

        {/each}
  {/each}

  {#if addCallBtnVisible}
    <button
      on:click={openAdaptorSelection}
      type="button"
      class="px-3 py-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 my-5"
    >Add</button>
  {/if}


  {#if adaptorSelectionOpen}
    <AdaptorSelection closeAdaptorSelection={closeAdaptorSelection} />
  {/if}


</div>

<button
  on:click={requestFlashLoan}
  type="button"
  class="px-5 py-3 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 m-5 text-xl"
>Loan</button>
