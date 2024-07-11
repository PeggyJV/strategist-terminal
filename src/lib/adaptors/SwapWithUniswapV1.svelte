<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  export let path = "";
  export let amount = "";
  export let amount_out_min = "";
  export let pool_fees = "";
  const SwapWithUniswapV1Address = "";

  async function scheduleSwapV2() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(SwapWithUniswapV1Address, "SwapWithUniswapV1", {
          SwapWithUniV2: {
            path,
            amount,
            amount_out_min
          },
        }),
      );
      return callQueue;
    });
  }

  async function scheduleSwapV3() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(SwapWithUniswapV1Address, "SwapWithUniswapV1", {
          SwapWithUniV3: {
            path,
            pool_fees,
            amount,
            amount_out_min
          },
        }),
      );
      return callQueue;
    });
  }


</script>


<h1>1. SwapWithUniV2</h1>
<div>
  <label for="path" title="Enter address as a string.">Address:</label>
  <input type="text" id="path" bind:value={path} placeholder="Address" />
</div>

<div>
  <label for="amount" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>

<div>
  <label for="amount_out_min" title="Enter the amount of the token to swap, as a string.">Amount min:</label>
  <input type="text" id="amount_out_min" bind:value={amount_out_min} placeholder="Amount out min" />
</div>

<button on:click={scheduleSwapV2}>Swap assets</button>

<h1>2. SwapWithUniV3</h1>
<div>
  <label for="path" title="Enter address as a string.">Address:</label>
  <input type="text" id="path" bind:value={path} placeholder="Address" />
</div>

<div>
  <label for="pool_fees" title="Enter the amount of the token to swap, as a string.">Pool fees:</label>
  <input type="text" id="pool_fees" bind:value={pool_fees} placeholder="Pool fees" />
</div>

<div>
  <label for="amount" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>

<div>
  <label for="amount_out_min" title="Enter the amount of the token to swap, as a string.">Amount min:</label>
  <input type="text" id="amount_out_min" bind:value={amount_out_min} placeholder="Amount out min" />
</div>

<button on:click={scheduleSwapV3}>Swap assets</button>
