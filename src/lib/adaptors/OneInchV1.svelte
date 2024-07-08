<script>
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
    title="Enter number."
  >swapCallData in bytes</label
  >
  <input
    type="number"
    id="swap_call_data"
    bind:value={swap_call_data}
    placeholder="swap_call_data"
  />
</div>

<button on:click={scheduleSwap}>Swap assets</button>

<!-- Additional text information at the bottom of the page -->
<div>
  <h2>Specific Strategist Information</h2>
  <p>
    A. Keep in mind regarding isolated markets. If they are isolated, then
    they may not be usable as collateral for other borrow positions.
  </p>
  <p>
    B. See the <a
    href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture"
  >docs</a
  > for more information pertaining to this adaptor.
  </p>
  <!-- Empty lines of spaces -->
  <br /><br /><br />
</div>
