<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";
  let token_borrow = "";
  let amount_borrow = "";

  let token_repay = "";
  let amount_repay = "";

  const AaveDebtTokenV2Address = "";

  function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV2Address, "AaveDebtTokenV2", {
          BorrowFromAave: {
            token: token_borrow,
            amount: amount_borrow,
          },
        }),
      );
      return callQueue;
    })
  }

  function scheduleRepayDebt() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV2Address, "AaveDebtTokenV2", {
          RepayAaveDebt: {
            token: token_repay,
            amount: amount_repay,
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
    for="token_borrow"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token_borrow" bind:value={token_borrow} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount_borrow"
    title="Enter the amount of the ERC-20 asset to borrow as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_borrow" bind:value={amount_borrow} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. RepayAaveDebt</h1>
<div>
  <label
    for="token_repay"
    title="Enter the ERC-20 token contract address as a string."
  >ERC-20 Token Contract Address:</label
  >
  <input type="text" id="token_repay" bind:value={token_repay} placeholder="0xtoken" />
</div>
<div>
  <label
    for="amount_repay"
    title="Enter the amount of the ERC-20 asset to repay as a string."
  >Amount of ERC-20 Asset:</label
  >
  <input type="text" id="amount_repay" bind:value={amount_repay} placeholder="Amount" />
</div>
<button on:click={scheduleRepayDebt}>Repay loan debt on Aave</button>
