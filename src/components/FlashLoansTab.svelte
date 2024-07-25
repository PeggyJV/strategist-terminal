<script lang="ts">
  import AaveV3ATokenV1 from "$lib/adaptors/AaveV3ATokenV1"
  import AaveV3DebtTokenV1 from "$lib/adaptors/AaveV3DebtTokenV1"

  let tokens = "";
  let amounts = "";

  let DepositToAave = AaveV3ATokenV1.calls[0];
  let BorrowFromAave = AaveV3DebtTokenV1.calls[0];

  let adaptorAddress = "";
  let depositToken = "";
  let depositAmount = "";

  let borrowToken = "";
  let borrowAmount = "";
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

  <h3>Internal Calls:</h3>

  <div class="ml-5">
    <h4>{DepositToAave.function}:</h4>
    <div class="mt-5 ml-5">
      <div class="flex justify-between mt-2">
        <label for="depositToken">{DepositToAave.fields[0].label}:</label>
        <input
          bind:value={depositToken}
          id="depositToken"
          placeholder={DepositToAave.fields[0].placeholder}
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
      <div class="flex justify-between mt-2">
        <label for="depositAmount">{DepositToAave.fields[1].label}:</label>
        <input
          bind:value={depositAmount}
          id="depositAmount"
          placeholder={DepositToAave.fields[1].placeholder}
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
    </div>

    <h4>{BorrowFromAave.function}:</h4>
    <div class="mt-5 ml-5">
      <div class="flex justify-between mt-2">
        <label for="borrowToken">{BorrowFromAave.fields[0].label}:</label>
        <input
          bind:value={borrowToken}
          id="borrowToken"
          placeholder={BorrowFromAave.fields[0].placeholder}
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
      <div class="flex justify-between mt-2">
        <label for="borrowAmount">{BorrowFromAave.fields[1].label}:</label>
        <input
          bind:value={borrowAmount}
          id="borrowAmount"
          placeholder={BorrowFromAave.fields[1].placeholder}
          class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>
    </div>
  </div>

  <button
    on:click={() => {}}
    type="button"
    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 my-5"
  >Loan</button>

</div>