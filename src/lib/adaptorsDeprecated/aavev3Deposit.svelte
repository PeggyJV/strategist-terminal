<script lang="ts">
    import { queue, CellarCall } from "$stores/AdapterQueue";

    let  token_id = "";
    let amount = "";
    const aaveDepositAdaptorAddress = "";

    function scheduleDeposit() {
        // const result = await invoke("aavev3_deposit", { tokenId, amount });
        // console.log(result);

      queue.update((callQueue) => {
        callQueue.push(
          new CellarCall(aaveDepositAdaptorAddress, "AaveATokenAdaptorV1", {
            DepositToAave: {
              token_id,
              amount,
            },
          }),
        );
        return callQueue;
      });
    }
</script>
<div class="prose mt-10 w-screen">
  <h1>Aave V3 Deposit</h1>
  <h2>1. Deposit</h2>

  <div class="flex justify-between">
    <label
      for="token_id"
      title="Enter the ERC-20 token contract address as a string."
    >ERC-20 Token Contract Address:</label>
    <input
        bind:value={token_id}
        id="token_id"
        placeholder="0xTokenID"
        class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />
  </div>

  <div class="flex justify-between">
    <label
      for="amount"
      title="Amount of ERC-20 Asset."
    >Amount of ERC-20 Asset:</label>
    <input
      bind:value={amount}
      id="amount"
      placeholder="Amount"
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />
  </div>

  <button
    on:click={scheduleDeposit}
    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
  >Deposit</button>
</div>
