<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  export let token_in = "";
  export let token_out = "";
  export let amount = "";
  export let swap_call_data = "";
  const ZeroXV1Address = "";

  async function scheduleSwap() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(ZeroXV1Address, "ZeroXV1", {
          SwapWith0x: {
            token_in,
            token_out,
            amount,
            swap_call_data
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. SwapWith0x</h1>
<div>
  <label for="tokenIn" title="Enter the ERC-20 token contract address to swap from, as a string.">Token In (ERC-20 Contract Address):</label>
  <input type="text" id="tokenIn" bind:value={token_in} placeholder="0xTokenInAddress" />
</div>

<div>
  <label for="tokenOut" title="Enter the ERC-20 token contract address to swap to, as a string.">Token Out (ERC-20 Contract Address):</label>
  <input type="text" id="tokenOut" bind:value={token_out} placeholder="0xTokenOutAddress" />
</div>

<div>
  <label for="amount" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>

<div>
  <label for="swapCallData" title="Enter the swap call data, in bytes format.">Swap Call Data:</label>
  <input type="text" id="swapCallData" bind:value={swap_call_data} placeholder="Swap Call Data" />
</div>
<button on:click={scheduleSwap}>Swap assets</button>
