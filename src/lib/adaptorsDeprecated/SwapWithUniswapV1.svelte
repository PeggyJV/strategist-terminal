<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let path_v2 = "";
  let amount_v2 = "";
  let amount_out_min_v2 = "";

  let path_v3 = "";
  let amount_v3 = "";
  let amount_out_min_v3 = "";
  let pool_fees = "";

  const SwapWithUniswapV1Address = "";

  function scheduleSwapV2() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(SwapWithUniswapV1Address, "SwapWithUniswapV1", {
          SwapWithUniV2: {
            path: path_v2,
            amount: amount_v2,
            amount_out_min: amount_out_min_v2
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleSwapV3() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(SwapWithUniswapV1Address, "SwapWithUniswapV1", {
          SwapWithUniV3: {
            path: path_v3,
            pool_fees,
            amount: amount_v3,
            amount_out_min: amount_out_min_v3
          },
        }),
      );
      return callQueue;
    });
  }


</script>


<h1>1. SwapWithUniV2</h1>
<div>
  <label for="path_v2" title="Enter address as a string.">Address:</label>
  <input type="text" id="path_v2" bind:value={path_v2} placeholder="Address" />
</div>

<div>
  <label for="amount_v2" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
  <input type="text" id="amount_v2" bind:value={amount_v2} placeholder="Amount" />
</div>

<div>
  <label for="amount_out_min_v2" title="Enter the amount of the token to swap, as a string.">Amount min:</label>
  <input type="text" id="amount_out_min_v2" bind:value={amount_out_min_v2} placeholder="Amount out min" />
</div>

<button on:click={scheduleSwapV2}>Swap assets</button>

<h1>2. SwapWithUniV3</h1>
<div>
  <label for="path_v3" title="Enter address as a string.">Address:</label>
  <input type="text" id="path_v3" bind:value={path_v3} placeholder="Address" />
</div>

<div>
  <label for="pool_fees" title="Enter the amount of the token to swap, as a string.">Pool fees:</label>
  <input type="text" id="pool_fees" bind:value={pool_fees} placeholder="Pool fees" />
</div>

<div>
  <label for="amount_v3" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
  <input type="text" id="amount_v3" bind:value={amount_v3} placeholder="Amount" />
</div>

<div>
  <label for="amount_out_min_v3" title="Enter the amount of the token to swap, as a string.">Amount min:</label>
  <input type="text" id="amount_out_min_v3" bind:value={amount_out_min_v3} placeholder="Amount out min" />
</div>

<button on:click={scheduleSwapV3}>Swap assets</button>
