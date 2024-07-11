<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  let market = ""
  let amount_to_deposit = ""
  let amount_to_withdraw = "";

  const CompoundCTokenV2Address = "";

  function scheduleDeposit() {
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

  function scheduleWithdraw() {
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

  function scheduleClaimComp() {
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
    title="Enter the compound market, as a string."
  >Market:</label
  >
  <input type="text" id="market" bind:value={market} placeholder="Enter the Compound market address" />
</div>
<div>
  <label
    for="amount_to_deposit"
    title="Enter the amount of the ERC-20 asset to deposit as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Enter amount to deposit" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- 2. withdrawFromCompound -->

<h1>2. Withdraw assets from Compound</h1>
<div>
  <label
    for="market"
    title="Enter the compound market, as a string."
  >Market:</label
  >
  <input type="text" id="market" bind:value={market} placeholder="Enter the Compound market address" />
</div>
<div>
  <label
    for="amount_to_withdraw"
    title="Enter the amount of the ERC-20 asset to withdraw as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Enter amount to withdraw" />
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- 2. claimComp -->

<h1>3. Claim COMP rewards</h1>

<button on:click={scheduleClaimComp}>Claim</button>
