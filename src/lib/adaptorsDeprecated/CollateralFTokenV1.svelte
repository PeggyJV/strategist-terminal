<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let fraxlend_pair_add = "";
  let collateral_to_deposit = "";

  let fraxlend_pair_remove = "";
  let collateral_amount = "";


  const CollateralFTokenV1Address = "";

  function scheduleAdd() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CollateralFTokenV1Address, "CollateralFTokenV1", {
          AddCollateral: {
            fraxlend_pair: fraxlend_pair_add,
            collateral_to_deposit,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRemove() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CollateralFTokenV1Address, "CollateralFTokenV1", {
          RemoveCollateral: {
            collateral_amount,
            fraxlend_pair: fraxlend_pair_remove,
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. Add Collateral</h1>
<div>
  <label for="fraxlend_pair_add" title="The FraxLend pair to add collateral to.">The FraxLend pair to add collateral to:</label>
  <input type="text" id="fraxlend_pair_add" bind:value={fraxlend_pair_add} placeholder="Address" />
</div>

<div>
  <label for="collateral_to_deposit" title="The amount of collateral to add to the cellar position.">The amount of collateral to add to the cellar position:</label>
  <input type="text" id="collateral_to_deposit" bind:value={collateral_to_deposit} placeholder="Amount" />
</div>

<button on:click={scheduleAdd}>Deposit</button>

<h1>2. Remove Collateral</h1>
<div>
  <label for="fraxlend_pair_remove" title="The FraxLend pair to remove collateral from.">The FraxLend pair to remove collateral from:</label>
  <input type="text" id="fraxlend_pair_remove" bind:value={fraxlend_pair_remove} placeholder="Address" />
</div>

<div>
  <label for="collateral_amount" title="The amount of collateral to remove from the cellar position.">The amount of collateral to remove from the cellar position:</label>
  <input type="text" id="collateral_amount" bind:value={collateral_amount} placeholder="Amount" />
</div>

<button on:click={scheduleRemove}>Withdraw</button>

