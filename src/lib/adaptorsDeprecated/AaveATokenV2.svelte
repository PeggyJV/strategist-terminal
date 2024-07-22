<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let token_deposit = "";
  let amount_deposit = "";

  let token_withdraw = "";
  let amount_withdraw = "";

  const AaveATokenV2Address = "";


  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveATokenV2Address, "AaveATokenV2", {
          DepositToAave: {
            token: token_deposit,
            amount: amount_deposit,
          },
        }),
      );
      return callQueue;
    })
  }

  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveATokenV2Address, "AaveATokenV2", {
          WithdrawFromAave: {
            token: token_withdraw,
            amount: amount_withdraw,
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
    for="token_deposit"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token_deposit" bind:value={token_deposit} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount_deposit"
    title="Enter the amount of the ERC-20 asset to borrow as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_deposit" bind:value={amount_deposit} placeholder="Amount" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- Aave Withdraw -->

<h1>2. Withdraw From Aave</h1>
<div>
  <label
    for="token_withdraw"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token_withdraw" bind:value={token_withdraw} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount_withdraw"
    title="Enter the amount of the ERC-20 asset to repay as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_withdraw" bind:value={amount_withdraw} placeholder="Amount" />
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>
