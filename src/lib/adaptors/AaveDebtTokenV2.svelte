<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  export let token = "";
  export let amount = "";

  const AaveDebtTokenV2Address = "";

  async function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV2Address, "AaveDebtTokenV2", {
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
        new CellarCall(AaveDebtTokenV2Address, "AaveDebtTokenV2", {
          RepayAaveDebt: {
            token,
            amount,
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
