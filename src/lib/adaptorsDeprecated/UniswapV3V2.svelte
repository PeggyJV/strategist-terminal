<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue.js"

  let token_0_1 = "";
  let token_0_6 = "";
  let token_0_8 = "";

  let token_1_1 = "";
  let token_1_6 = "";
  let token_1_8 = "";

  let amount_0_1 = "";
  let amount_0_3 = "";
  let amount_0_5 = "";

  let amount_1_1 = "";
  let amount_1_3 = "";
  let amount_1_5 = "";

  let pool_fee = 0;

  let min_0_1 = "";
  let min_0_2 = "";
  let min_0_3 = "";
  let min_0_4 = "";

  let min_1_1 = "";
  let min_1_2 = "";
  let min_1_3 = "";
  let min_1_4 = "";

  let token_id_2 = "";
  let token_id_3 = "";
  let token_id_4 = "";
  let token_id_5 = "";
  let token_id_7 = "";
  let token_id_8 = "";

  let tick_lower = 0;
  let tick_upper = 0;
  let liquidity = "";
  let take_fees = false;

  const UniswapV3V2Address = "";

  // Functions for operations

  function scheduleOpenPosition() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          OpenPosition: { token_0: token_0_1, token_1: token_1_1, pool_fee, amount_0: amount_0_1, amount_1: amount_1_1, min_0: min_0_1, min_1: min_1_1, tick_lower, tick_upper },
        }),
      );
      return callQueue;
    });
  }

  function scheduleClosePosition() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          ClosePosition: { token_id: token_id_2, min_0: min_0_2, min_1: min_1_2 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleAddToPosition() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          AddToPosition: { token_id: token_id_3, amount_0: amount_0_3, amount_1: amount_1_3, min_0: min_0_3, min_1: min_1_3 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleTakeFromPosition() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          TakeFromPosition: { token_id: token_id_4, liquidity, min_0: min_0_4, min_1: min_1_4, take_fees },
        }),
      );
      return callQueue;
    });
  }

  function scheduleCollectFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          CollectFees: { token_id: token_id_5, amount_0: amount_0_5, amount_1:  amount_1_5},
        }),
      );
      return callQueue;
    });
  }

  function schedulePurgeAllZeroLiquidityPositions() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          PurgeAllZeroLiquidityPositions: { token_0: token_0_6, token_1: token_1_6 },
        }),
      );
      return callQueue;
    });
  }

   function schedulePurgeSinglePosition() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          PurgeSinglePosition: { token_id: token_id_7 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRemoveUnownedPositionFromTracker() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(UniswapV3V2Address, "UniswapV3V2", {
          RemoveUnownedPositionFromTracker: { token_id: token_id_8, token_0: token_0_8, token_1: token_1_8 },
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- UI for OpenPosition -->
<h1>1. Open Position</h1>
<div>
  <label for="token_0_1" title="Enter the Open Position #0 ERC-20 token contract address as a string.">Token 0:</label>
  <input type="text" id="token_0_1" bind:value={token_0_1} placeholder="Enter Token 0 Address">
</div>
<div>
  <label for="token_1_1" title="Enter the Open Position #1 ERC-20 token contract address as a string.">Token 1:</label>
  <input type="text" id="token_1_1" bind:value={token_1_1} placeholder="Enter Token 1 Address">
</div>
<div>
  <label for="op_pool_fee" title="Enter the pool fee as a number.">Pool Fee:</label>
  <input type="number" id="op_pool_fee" bind:value={pool_fee} placeholder="Enter Pool Fee">
</div>
<div>
  <label for="amount_0_1" title="Enter the amount of token0 involved with this function call as a string.">Amount 0:</label>
  <input type="text" id="amount_0_1" bind:value={amount_0_1} placeholder="Enter Amount 0">
</div>
<div>
  <label for="amount_1_1" title="Enter the amount of token1 involved with this function call as a string.">Amount 1:</label>
  <input type="text" id="amount_1_1" bind:value={amount_1_1} placeholder="Enter Amount 1">
</div>
<div>
  <label for="min_0_1" title="Enter the minimum amount of `token0` to add to liquidity.">Min 0:</label>
  <input type="text" id="min_0_1" bind:value={min_0_1} placeholder="Enter Min 0">
</div>
<div>
  <label for="min_1_1" title="Enter the minimum amount of `token1` to add to liquidity.">Min 1:</label>
  <input type="text" id="min_1_1" bind:value={min_1_1} placeholder="Enter Min 1">
</div>
<div>
  <label for="op_tick_lower" title="Enter the lower liquidity tick.">Tick Lower:</label>
  <input type="number" id="op_tick_lower" bind:value={tick_lower} placeholder="Enter Tick Lower">
</div>
<div>
  <label for="op_tick_upper" title="Enter the upper liquidity tick.">Tick Upper:</label>
  <input type="number" id="op_tick_upper" bind:value={tick_upper} placeholder="Enter Tick Upper">
</div>
<button on:click={scheduleOpenPosition}>Open Position</button>


<!-- UI for ClosePosition -->
<h1>2. Close Position</h1>
<div>
  <label for="token_id_2" title="Enter the UniV3 LP NFT tokenId to close." >Token ID:</label>
  <input type="text" id="token_id_2" bind:value={token_id_2} placeholder="Enter Token ID">
</div>
<div>
  <label for="min_0_2" title="Enter the minimum amount of `token0` to get from closing this position.">Min 0:</label>
  <input type="text" id="min_0_2" bind:value={min_0_2} placeholder="Enter Minimum 0">
</div>
<div>
  <label for="min_1_2" title="Enter the minimum amount of `token1` to get from closing this position.">Min 1:</label>
  <input type="text" id="min_1_2" bind:value={min_1_2} placeholder="Enter Minimum 1">
</div>
<button on:click={scheduleClosePosition}>Close Position</button>

<!-- UI for AddToPosition -->
<h1>3. Add To Position</h1>
<div>
  <label for="token_id_3" title="Enter the UniV3 LP NFT id to add liquidity to.">Token ID:</label>
  <input type="text" id="token_id_3" bind:value={token_id_3} placeholder="Enter Token ID">
</div>
<div>
  <label for="amount_0_3" title="Enter the amount of `token0` to add to liquidity.">Amount 0:</label>
  <input type="text" id="amount_0_3" bind:value={amount_0_3} placeholder="Enter Amount 0">
</div>
<div>
  <label for="amount_1_3" title="Enter the amount of `token1` to add to liquidity.">Amount 1:</label>
  <input type="text" id="amount_1_3" bind:value={amount_1_3} placeholder="Enter Amount 1">
</div>
<div>
  <label for="min_0_3" title="Enter the minimum amount of `token0` to add to liquidity.">Min 0:</label>
  <input type="text" id="min_0_3" bind:value={min_0_3} placeholder="Enter Min 0">
</div>
<div>
  <label for="min_1_3" title="Enter the minimum amount of `token1` to add to liquidity.">Min 1:</label>
  <input type="text" id="min_1_3" bind:value={min_1_3} placeholder="Enter Min 1">
</div>
<button on:click={scheduleAddToPosition}>Add To Position</button>

<!-- UI for TakeFromPosition -->
<h1>4. Take From Position</h1>
<div>
  <label for="token_id_4" title="Enter the UniV3 LP NFT tokenId to take from.">Token ID:</label>
  <input type="text" id="token_id_4" bind:value={token_id_4} placeholder="Enter Token ID">
</div>
<div>
  <label for="tfp_liquidity" title="Enter the amount of liquidity to take from the position.">Liquidity:</label>
  <input type="text" id="tfp_liquidity" bind:value={liquidity} placeholder="Enter Liquidity">
</div>
<div>
  <label for="min_0_4" title="Enter the minimum amount of `token0` to get from taking liquidity.">Min 0:</label>
  <input type="text" id="min_0_4" bind:value={min_0_4} placeholder="Enter Min 0">
</div>
<div>
  <label for="min_1_4" title="Enter the minimum amount of `token1` to get from taking liquidity.">Min 1:</label>
  <input type="text" id="min_1_4" bind:value={min_1_4} placeholder="Enter Min 1">
</div>
<div>
  <label for="tfp_take_fees" title="Enter bool indicating whether to collect principal(if false), or principal + fees (if true).">Take Fees:</label>
  <input type="checkbox" id="tfp_take_fees" bind:checked={take_fees}>
</div>
<button on:click={scheduleTakeFromPosition}>Take From Position</button>

<!-- UI for CollectFees -->
<h1>5. Collect Fees</h1>
<div>
  <label for="token_id_5" title="Enter the UniV3 LP NFT id to collect fees from.">Token ID:</label>
  <input type="text" id="token_id_5" bind:value={token_id_5} placeholder="Enter Token ID">
</div>
<div>
  <label for="amount_0_5" title="Enter the amount of `token0` fees to collect use type(uint128).max to get collect all.">Amount 0:</label>
  <input type="text" id="amount_0_5" bind:value={amount_0_5} placeholder="Enter Amount 0">
</div>
<div>
  <label for="amount_1_5" title="Enter the amount of `token1` fees to collect use type(uint128).max to get collect all.">Amount 1:</label>
  <input type="text" id="amount_1_5" bind:value={amount_1_5} placeholder="Enter Amount 1">
</div>
<button on:click={scheduleCollectFees}>Collect Fees</button>

<!-- UI for PurgeAllZeroLiquidityPositions -->
<h1>6. Purge All Zero Liquidity Positions</h1>
<div>
  <label for="token_0_6" title="Enter the `token0` address that are part of the positions you would like to purge, as a string.">Token 0:</label>
  <input type="text" id="token_0_6" bind:value={token_0_6} placeholder="Enter Token 0 Address">
</div>
<div>
  <label for="token_1_6" title="Enter the `token1` address that are part of the positions you would like to purge, as a string">Token 1:</label>
  <input type="text" id="token_1_6" bind:value={token_1_6} placeholder="Enter Token 1 Address">
</div>
<button on:click={schedulePurgeAllZeroLiquidityPositions}>Purge All Zero Liquidity Positions</button>

<!-- UI for PurgeSinglePosition -->
<h1>7. Purge Single Position</h1>
<div>
  <label for="token_id_7" title="Enter the tokenId of the position to purge.">Token ID:</label>
  <input type="text" id="token_id_7" bind:value={token_id_7} placeholder="Enter Token ID">
</div>
<button on:click={schedulePurgeSinglePosition}>Purge Single Position</button>

<!-- UI for RemoveUnownedPositionFromTracker -->
<h1>8. Remove Unowned Position From Tracker</h1>
<div>
  <label for="token_id_8" title="Likely unused call, but if needed, enter the tokenId of the position to remove from tracker.">Token ID:</label>
  <input type="text" id="token_id_8" bind:value={token_id_8} placeholder="Enter Token ID">
</div>
<div>
  <label for="token_0_8" title="Likely unused call, but if needed, enter the address of token0 of the position to remove from tracker.">Token 0:</label>
  <input type="text" id="token_0_8" bind:value={token_0_8} placeholder="Enter Token 0 Address">
</div>
<div>
  <label for="token_1_8" title="Likely unused call, but if needed, enter the address of token1 of the position to remove from tracker.">Token 1:</label>
  <input type="text" id="token_1_8" bind:value={token_1_8} placeholder="Enter Token 1 Address">
</div>
<button on:click={scheduleRemoveUnownedPositionFromTracker}>Remove Unowned Position</button>
