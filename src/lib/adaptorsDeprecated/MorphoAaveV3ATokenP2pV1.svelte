<script lang="ts">

  import { CellarCall, queue } from "$stores/AdapterQueue"

  let token_to_deposit = "";
  let amount_to_deposit = "";
  let max_iterations_deposit = "";
  let token_to_withdraw = "";
  let amount_to_withdraw = "";
  let max_iterations_withdraw = "";

  const MorphoAaveV3ATokenP2pV1Address = "";

  function scheduleDepositToAaveV3Morpho() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV3ATokenP2pV1Address, "MorphoAaveV3ATokenP2pV1", {
          DepositToAaveV3Morpho: { token_to_deposit, amount_to_deposit, max_iterations: max_iterations_deposit },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdrawFromAaveV3Morpho() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV3ATokenP2pV1Address, "MorphoAaveV3ATokenP2pV1", {
          WithdrawFromAaveV3Morpho: { token_to_withdraw, amount_to_withdraw, max_iterations: max_iterations_withdraw },
        }),
      );
      return callQueue;
    });
  }


</script>

<!-- Deposit to Aave V3 Morpho -->

<h1>1. Deposit to Aave V3 Morpho</h1>
<div>
  <label for="token_to_deposit" title="Enter the ERC-20 token contract address as a string.">ERC-20 Token Contract Address:</label>
  <input type="text" id="token_to_deposit" bind:value={token_to_deposit} placeholder="0xTokenAddress" />
</div>
<div>
  <label for="amount_to_deposit" title="Enter the amount of the ERC-20 asset to deposit as a string.">Amount of ERC-20 Asset:</label>
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount" />
</div>
<div>
  <label for="max_iterations_deposit" title="Enter the maximum number of iterations as a string.">Max Iterations:</label>
  <input type="text" id="max_iterations_deposit" bind:value={max_iterations_deposit} placeholder="Max Iterations" />
</div>
<button on:click={scheduleDepositToAaveV3Morpho}>Deposit</button>

<!-- Withdraw from Aave V3 Morpho -->

<h1>2. Withdraw from Aave V3 Morpho</h1>
<div>
  <label for="token_to_withdraw" title="Enter the ERC-20 token contract address as a string.">ERC-20 Token Contract Address:</label>
  <input type="text" id="token_to_withdraw" bind:value={token_to_withdraw} placeholder="0xTokenAddress" />
</div>
<div>
  <label for="amount_to_withdraw" title="Enter the amount of the ERC-20 asset to withdraw as a string.">Amount of ERC-20 Asset:</label>
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Amount" />
</div>
<div>
  <label for="max_iterations_withdraw" title="Enter the maximum number of iterations as a string.">Max Iterations:</label>
  <input type="text" id="max_iterations_withdraw" bind:value={max_iterations_withdraw} placeholder="Max Iterations" />
</div>
<button on:click={scheduleWithdrawFromAaveV3Morpho}>Withdraw</button>