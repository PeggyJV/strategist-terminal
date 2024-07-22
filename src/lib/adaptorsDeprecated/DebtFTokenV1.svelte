<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let fraxlend_pair_borrow = "";
  let amount_to_borrow = "";

  let fraxlend_pair_repay = "";
  let debt_token_repay_amount = "";

  let fraxlend_pair_add = "";

  const DebtFTokenV1Address = "";

  function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(DebtFTokenV1Address, "DebtFTokenV1", {
          BorrowFromFraxlend: {
            fraxlend_pair: fraxlend_pair_borrow,
            amount_to_borrow,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRepay() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(DebtFTokenV1Address, "DebtFTokenV1", {
          RepayFraxlendDebt: {
            fraxlend_pair: fraxlend_pair_repay,
            debt_token_repay_amount,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleAddInterest() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(DebtFTokenV1Address, "DebtFTokenV1", {
          CallAddInterest: {
            fraxlend_pair_add
          },
        }),
      );
      return callQueue;
    });
  }
</script>

<h1>1. Borrow From Fraxlend</h1>
<div>
  <label for="fraxlend_pair_borrow" title="The address of the Frax Pair to borrow from.">The address of the Frax Pair to borrow from:</label>
  <input type="text" id="fraxlend_pair_borrow" bind:value={fraxlend_pair_borrow} placeholder="Address" />
</div>

<div>
  <label for="amount_to_borrow" title="The amount of the asset to borrow.">The amount of the asset to borrow:</label>
  <input type="text" id="amount_to_borrow" bind:value={amount_to_borrow} placeholder="Amount" />
</div>

<button on:click={scheduleBorrow}>Deposit</button>

<h1>2. Repay Fraxlend Debt</h1>
<div>
  <label for="fraxlend_pair_repay" title="The address of the Frax Pair to repay debt on.">The address of the Frax Pair to repay debt on:</label>
  <input type="text" id="fraxlend_pair_repay" bind:value={fraxlend_pair_repay} placeholder="Address" />
</div>

<div>
  <label for="debt_token_repay_amount" title="The amount of the debt token to repay.">The amount of the debt token to repay:</label>
  <input type="text" id="debt_token_repay_amount" bind:value={debt_token_repay_amount} placeholder="Amount" />
</div>

<button on:click={scheduleRepay}>Withdraw</button>

<h1>3. Call Add Interest</h1>

<div>
  <label for="fraxlend_pair_add" title="The address of the Frax Pair to call addInterest on.">The address of the Frax Pair to call addInterest on:</label>
  <input type="text" id="fraxlend_pair_add" bind:value={fraxlend_pair_add} placeholder="Address" />
</div>

<button on:click={scheduleAddInterest}>Add Interest</button>
