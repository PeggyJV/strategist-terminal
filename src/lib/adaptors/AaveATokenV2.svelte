<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  export let token = "";
  export let amount = "";

  const AaveATokenV2Address = "";


  async function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveATokenV2Address, "AaveATokenV2", {
          DepositToAave: {
            token,
            amount,
          },
        }),
      );
      return callQueue;
    })
  }

  async function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveATokenV2Address, "AaveATokenV2", {
          WithdrawFromAave: {
            token,
            amount,
          },
        }),
      );
      return callQueue;
    })
  }

</script>

<!-- Aave Deposit -->

<h1>1. Deposit To Aave</h1>
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
<button on:click={scheduleDeposit}>Deposit</button>

<!-- Aave Withdraw -->

<h1>2. Withdraw From Aave</h1>
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
<button on:click={scheduleWithdraw}>Withdraw</button>


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
