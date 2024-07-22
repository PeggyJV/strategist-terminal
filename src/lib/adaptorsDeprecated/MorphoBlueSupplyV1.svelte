<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let market_1 = "";
  let assets_1 = "";

  let market_2 = "";
  let assets_2 = "";

  let MorphoBlueSupplyV1Address = "";

  function scheduleLend() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueSupplyV1Address, "MorphoBlueSupplyV1", {
          LendToMorphoBlue: {
            market: market_1,
            assets: assets_1
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoBlueSupplyV1Address, "MorphoBlueSupplyV1", {
          WithdrawFromMorphoBlue: {
            market: market_2,
            assets: assets_2
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. Lend To Morpho Blue</h1>
<div>
  <label for="market_1" title="Identifier of a Morpho Blue Market.">Identifier of a Morpho Blue Market:</label>
  <input type="text" id="market_1" bind:value={market_1} placeholder="Market" />
</div>

<div>
  <label for="assets_1" title="The amount of the loan token to lend.">The amount of the loan token to lend:</label>
  <input type="text" id="assets_1" bind:value={assets_1} placeholder="Amount" />
</div>

<button on:click={scheduleLend}>Lend</button>

<h1>2. Withdraw From Morpho Blue</h1>
<div>
  <label for="market_2" title="Identifier of a Morpho Blue Market.">Identifier of a Morpho Blue Market:</label>
  <input type="text" id="market_2" bind:value={market_2} placeholder="Market" />
</div>

<div>
  <label for="assets_2" title="The amount of the loan token to lend.">The amount of the loan token to lend:</label>
  <input type="text" id="assets_2" bind:value={assets_2} placeholder="Amount" />
</div>

<button on:click={scheduleWithdraw}>Withdraw</button>
