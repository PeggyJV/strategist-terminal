<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  let market_deposit = "";
  let amount_to_deposit = "";

  let market_withdraw = "";
  let amount_to_withdraw = "";

  const CompoundCTokenV2Address = "";

  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CompoundCTokenV2Address, "CompoundCTokenV2", {
          DepositToCompound: {
            market: market_deposit,
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
            market: market_withdraw,
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
    for="market_deposit"
    title="Enter the compound market, as a string."
  >Enter the Compound market address:</label
  >
  <input type="text" id="market_deposit" bind:value={market_deposit} placeholder="Market" />
</div>
<div>
  <label
    for="amount_to_deposit"
    title="Enter the amount of the ERC-20 asset to deposit as a string."
  >Enter amount to deposit:</label
  >
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- 2. withdrawFromCompound -->

<h1>2. Withdraw assets from Compound</h1>
<div>
  <label
    for="market_withdraw"
    title="Enter the compound market, as a string."
  >Enter the Compound market address:</label
  >
  <input type="text" id="market_withdraw" bind:value={market_withdraw} placeholder="Market" />
</div>
<div>
  <label
    for="amount_to_withdraw"
    title="Enter the amount of the ERC-20 asset to withdraw as a string."
  >Enter amount to withdraw:</label
  >
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Amount" />
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- 2. claimComp -->

<h1>3. Claim COMP rewards</h1>

<button on:click={scheduleClaimComp}>Claim</button>
