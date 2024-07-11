<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let token = "";
  let amount = "";

  const AaveATokenV2Address = "";


  function scheduleDeposit() {
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

  function scheduleWithdraw() {
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
