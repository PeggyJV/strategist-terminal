<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let market_1 = "";
  let amount_to_borrow = "";

  let market_2 = "";
  let debt_token_repay_amount = "";

  const MorphoBlueDebtV1Address = "";

  function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueDebtV1Address, "MorphoBlueDebtV1", {
          BorrowFromMorphoBlue: {
            market: market_1,
            amount_to_borrow,
          },
        }),
      );
      return callQueue;
    })
  }

  function scheduleRepayDebt() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueDebtV1Address, "MorphoBlueDebtV1", {
          RepayMorphoBlueDebt: {
            market: market_2,
            debt_token_repay_amount,
          },
        }),
      );
      return callQueue;
    })
  }

</script>

<!-- Aave V3 Deposit -->

<h1>1. Borrow From Morpho Blue </h1>
<div>
  <label
    for="market_1"
    title="Identifier of a Morpho Blue Market."
  >Identifier of a Morpho Blue Market:</label
  >
  <input type="text" id="market_1" bind:value={market_1} placeholder="Market" />
</div>
<div>
  <label
    for="amount_to_borrow"
    title="The amount of the debt token to borrow."
  >The amount of the debt token to borrow:</label
  >
  <input type="text" id="amount_to_borrow" bind:value={amount_to_borrow} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. Repay Morpho Blue Debt</h1>
<div>
  <label
    for="market_2"
    title="Identifier of a Morpho Blue Market."
  >Identifier of a Morpho Blue Market:</label
  >
  <input type="text" id="market_2" bind:value={market_2} placeholder="Market" />
</div>
<div>
  <label
    for="debt_token_repay_amount"
    title="The amount of the debt token to repay."
  >The amount of the debt token to repay:</label
  >
  <input type="text" id="debt_token_repay_amount" bind:value={debt_token_repay_amount} placeholder="Amount" />
</div>
<button on:click={scheduleRepayDebt}>Repay</button>
