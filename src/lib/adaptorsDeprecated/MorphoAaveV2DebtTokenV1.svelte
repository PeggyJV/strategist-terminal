<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue.js"

  let a_token_borrow = "";
  let amount_to_borrow = "";
  let a_token_repay = "";
  let amount_to_repay = "";

  const MorphoAaveV2DebtTokenV1Address = "";

  function scheduleBorrowFromAaveV2Morpho() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV2DebtTokenV1Address, "MorphoAaveV2DebtTokenV1", {
          BorrowFromAaveV2Morpho: { a_token: a_token_borrow, amount_to_borrow },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRepayAaveV2MorphoDebt() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV2DebtTokenV1Address, "MorphoAaveV2DebtTokenV1", {
          RepayAaveV2MorphoDebt: { a_token: a_token_repay, amount_to_repay },
        }),
      );
      return callQueue;
    });
  }
</script>

<!-- Borrow from Aave V2 Morpho -->

<h1>1. Borrow from Aave V2 Morpho</h1>
<div>
  <label for="a_token_borrow" title="Enter the Aave V2 aToken contract address as a string.">Aave V2 aToken Contract Address:</label>
  <input type="text" id="a_token_borrow" bind:value={a_token_borrow} placeholder="0xaTokenAddress" />
</div>
<div>
  <label for="amount_to_borrow" title="Enter the amount of the asset to borrow as a string.">Amount of Asset to Borrow:</label>
  <input type="text" id="amount_to_borrow" bind:value={amount_to_borrow} placeholder="Amount" />
</div>
<button on:click={scheduleBorrowFromAaveV2Morpho}>Borrow</button>

<!-- Repay Aave V2 Morpho Debt -->

<h1>2. Repay Aave V2 Morpho Debt</h1>
<div>
  <label for="a_token_repay" title="Enter the Aave V2 aToken contract address as a string.">Aave V2 aToken Contract Address:</label>
  <input type="text" id="a_token_repay" bind:value={a_token_repay} placeholder="0xaTokenAddress" />
</div>
<div>
  <label for="amount_to_repay" title="Enter the amount of the asset to repay as a string.">Amount of Asset to Repay:</label>
  <input type="text" id="amount_to_repay" bind:value={amount_to_repay} placeholder="Amount" />
</div>
<button on:click={scheduleRepayAaveV2MorphoDebt}>Repay</button>