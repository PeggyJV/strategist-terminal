<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  export let market = ""
  export let amount_to_deposit = ""
  export let amount_to_withdraw = "";

  const CompoundCTokenV2Address = "";

  async function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CompoundCTokenV2Address, "CompoundCTokenV2", {
          DepositToCompound: {
            market,
            amount_to_deposit,
          },
        }),
      );
      return callQueue;
    })
  }

  async function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CompoundCTokenV2Address, "CompoundCTokenV2", {
          WithdrawFromCompound: {
            market,
            amount_to_withdraw,
          },
        }),
      );
      return callQueue;
    })
  }

  async function scheduleClaimComp() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CompoundCTokenV2Address, "CompoundCTokenV2", {
          ClaimComp: {},
        }),
      );
      return callQueue;
    })
  }
</script>

<!-- 1. depositToCompound -->

<h1>1. Lend assets on Compound </h1>
<div>
  <label
    for="market"
    title=""
  >Market:</label
  >
  <input type="text" id="market" bind:value={market} placeholder="market" />
</div>
<div>
  <label
    for="amount_to_deposit"
    title="Enter the amount of the ERC-20 asset to deposit as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- 2. withdrawFromCompound -->

<h1>2. Withdraw assets from Compound</h1>
<div>
  <label
    for="market"
    title="Enter the ERC-20 token contract address as a string."
  >Market:</label
  >
  <input type="text" id="market" bind:value={market} placeholder="market" />
</div>
<div>
  <label
    for="amount_to_withdraw"
    title="Enter the amount of the ERC-20 asset to withdraw as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Amount" />
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- 2. claimComp -->

<h1>3. Claim COMP rewards</h1>

<button on:click={scheduleClaimComp}>Claim</button>

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
