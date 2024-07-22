<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let token_borrow = "";
  let amount_borrow = "";

  let token_repay = "";
  let amount_repay = "";

  let token_in = "";
  let token_to_repay = "";
  let amount_swap = "";
  let exchange = "";

  const AaveDebtTokenV1Address = "";

  function scheduleBorrow() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
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
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
          RepayAaveDebt: {
            token: token_repay,
            amount: amount_repay,
          },
        }),
      );
      return callQueue;
    })
  }

  function scheduleSwapAndRepay() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveDebtTokenV1Address, "AaveDebtTokenV1", {
          SwapAndRepay: {
            token_in,
            token_to_repay,
            amount: amount_swap,
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
    for="amount_swap"
    title="Enter the amount of the ERC-20 asset to repay as a string."
  >The amount to swap:</label
  >
  <input type="text" id="amount_swap" bind:value={amount_swap} placeholder="Amount" />
</div>


<div>
  <label
    for="token_to_repay"
    title="Enter number."
  >The exchange to make the swap on</label
  >
  <input
    type="text"
    id="exchange"
    bind:value={exchange}
    placeholder="Exchange"
  />
</div>

<button on:click={scheduleSwapAndRepay}>Swap assets and repay loans in one call</button>
