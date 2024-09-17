<script lang="ts">
  import AdaptorSelection from "./AdaptorSelection.svelte"
  import { CellarCall, flashLoanCalls, queue } from "$stores/AdapterQueue"

  let adaptorSelectionOpen = false;
  let addCallBtnVisible = true;

  let adaptorAddress = "";

  let tokens = "";
  let amounts = "";

  enum FlashLoans {
    AaveV3DebtTokenV1FlashLoan,
    BalancerPoolV1FlashLoan,
  }

  let selectedFlashLoan: FlashLoans = FlashLoans.AaveV3DebtTokenV1FlashLoan;

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
    const cTokens = convertToArray(tokens);
    const cAmounts = convertToArray(amounts);

    let cellarCall: CellarCall;

    if (selectedFlashLoan === FlashLoans.AaveV3DebtTokenV1FlashLoan) {
      cellarCall = new CellarCall(adaptorAddress, "AaveV3DebtTokenV1FlashLoan", {
        loan_tokens: cTokens,
        loan_amounts: cAmounts,
        params: []
      });
    } else {
      cellarCall = new CellarCall(adaptorAddress, "BalancerPoolV1FlashLoan", {
        tokens: cTokens,
        amounts: cAmounts,
        data: []
      });
    }

    queue.update((callQueue) => {
      callQueue.push(
        cellarCall
      );
      return callQueue;
    });
  }

  function convertToArray(value: string): any {
    try {
      const parsedValue = JSON.parse(value);

      if (Array.isArray(parsedValue)) {
        return parsedValue;
      }
    } catch (e) {}
    return value;
  }


</script>

<div class="prose">
  <div class="flex justify-center">
    <h1>Flash Loans</h1>
  </div>

  <div class="flex flex-row">
    <button
      on:click={() => selectedFlashLoan = FlashLoans.AaveV3DebtTokenV1FlashLoan}
      value="AaveV3DebtTokenV1FlashLoan"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {selectedFlashLoan === FlashLoans.AaveV3DebtTokenV1FlashLoan ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >AaveV3DebtTokenV1FlashLoan</button>
    <button
      on:click={() => selectedFlashLoan = FlashLoans.BalancerPoolV1FlashLoan}
      value="BalancerPoolV1FlashLoan"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {selectedFlashLoan === FlashLoans.BalancerPoolV1FlashLoan ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >BalancerPoolV1FlashLoan</button>
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
      placeholder='e.g., ["0x000...", "0x000..."]'
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />
  </div>

  <div class="flex justify-between mt-2">
    <label for="amounts">Enter the loan amounts, as array of strings:</label>
    <input
      bind:value={amounts}
      id="amounts"
      placeholder='e.g., ["50", "100"]'
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
              class="px-1 text-red-500 text-3xl rounded-md hover:text-red-600 "
            > &times;</button>

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
>Add to queue</button>
