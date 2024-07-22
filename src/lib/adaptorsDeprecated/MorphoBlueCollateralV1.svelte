<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let market_add = "";
  let collateral_to_deposit = "";

  let market_remove = "";
  let collateral_amount = "";

  const MorphoBlueCollateralV1Address = "";

  function scheduleAddCollateral() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueCollateralV1Address, "MorphoBlueCollateralV1", {
          AddCollateral: {
            market: market_add,
            collateral_to_deposit
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRemoveCollateral() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueCollateralV1Address, "MorphoBlueCollateralV1", {
          RemoveCollateral: {
            market: market_remove,
            collateral_amount
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. Add Collateral</h1>
<div>
  <label for="market_add" title="Identifier of a Morpho Blue Market.">Identifier of a Morpho Blue Market:</label>
  <input type="text" id="market_add" bind:value={market_add} placeholder="Market" />
</div>

<div>
  <label for="collateral_to_deposit" title="The amount of collateral to add.">The amount of collateral to add:</label>
  <input type="text" id="collateral_to_deposit" bind:value={collateral_to_deposit} placeholder="Amount" />
</div>

<button on:click={scheduleAddCollateral}>Add</button>

<h1>2. Remove Collateral</h1>
<div>
  <label for="market_remove" title="Identifier of a Morpho Blue Market.">Identifier of a Morpho Blue Market:</label>
  <input type="text" id="market_remove" bind:value={market_remove} placeholder="Market" />
</div>

<div>
  <label for="collateral_amount" title="The amount of collateral to remove.">The amount of collateral to remove:</label>
  <input type="text" id="collateral_amount" bind:value={collateral_amount} placeholder="Amount" />
</div>

<button on:click={scheduleRemoveCollateral}>Remove</button>
