<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  export let token = "";
  export let amount = "";
  export let token_in = ""
  export let token_to_repay = ""
  export let exchange: number | null = null;

  const AaveDebtTokenV1Address = "";

  async function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
          BorrowFromAave: {
            token,
            amount,
          },
        }),
      );
      return callQueue;
    })
  }

  async function scheduleRepayDebt() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
          RepayAaveDebt: {
            token,
            amount,
          },
        }),
      );
      return callQueue;
    })
  }

  async function scheduleSwapAndRepay() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
          SwapAndRepay: {
            token_in,
            token_to_repay,
            amount,
            exchange
          },
        }),
      );
      return callQueue;
    })
  }
</script>

<!-- Aave V3 Deposit -->

<h1>1. BorrowFromAave </h1>
<div>
  <label
    for="token"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount"
    title="Enter the amount of the ERC-20 asset to borrow as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. RepayAaveDebt</h1>
<div>
  <label
    for="token"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount"
    title="Enter the amount of the ERC-20 asset to repay as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleRepayDebt}>Repay loan debt on Aave</button>


<h1>3. SwapAndRepay</h1>
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
    for="token_to_repay"
    title="Enter the ERC-20 token contract address as a string."
  >The address of the token to swap to and repay with</label
  >
  <input
    type="text"
    id="token_to_repay"
    bind:value={token_to_repay}
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
    for="token_to_repay"
    title="Enter number."
  >The exchange to make the swap on</label
  >
  <input
    type="number"
    id="exchange"
    bind:value={exchange}
    placeholder="Exchange"
  />
</div>

<button on:click={scheduleSwapAndRepay}>Swap assets and repay loans in one call</button>

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
