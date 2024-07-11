<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  export let token_in = "";
  export let token_out = "";
  export let amount = "";
  export let swap_call_data = "";
  const OneInchV1Address = "";

  async function scheduleSwap() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(OneInchV1Address, "OneInchV1", {
          SwapWithOneInch: {
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

<h1>1. SwapWithOneInch</h1>
<div>
  <label
    for="token_in"
    title="Enter the ERC-20 token contract address as a string."
  >The address of the token to swap from</label
  >
  <input
    type="text"
    id="token_in"
    bind:value={token_in}
    placeholder="0xtoken"
  />
</div>
<div>
  <label
    for="token_out"
    title="Enter the ERC-20 token contract address as a string."
  >The address of the token to swap to</label
  >
  <input
    type="text"
    id="token_out"
    bind:value={token_out}
    placeholder="0xtoken"
  />
</div>

<div>
  <label
    for="amount"
    title="Enter the amount of the ERC-20 asset to repay as a string."
  >The amount to swap:</label
  >
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>


<div>
  <label
    for="swap_call_data"
    title="Enter the swap_call_dataas a bytes32."
  >Swap Calldata:</label
  >
  <input
    type="text"
    id="swap_call_data"
    bind:value={swap_call_data}
    placeholder="Swap Calldata"
  />
</div>

<button on:click={scheduleSwap}>Swap assets</button>
