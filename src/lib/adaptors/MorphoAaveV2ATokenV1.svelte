<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue.js"

  /// vars
  let a_token_deposit = "";
  let amount_to_deposit = "";
  let a_token_withdraw = "";
  let amount_to_withdraw = "";
  let MorphoAaveV2ATokenV1Address = "";

  function scheduleDepositToAaveV2Morpho() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV2ATokenV1Address, "MorphoAaveV2ATokenV1", {
          DepositToAaveV2Morpho: { a_token: a_token_deposit, amount_to_deposit },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdrawFromAaveV2Morpho() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(MorphoAaveV2ATokenV1Address, "MorphoAaveV2ATokenV1", {
          WithdrawFromAaveV2Morpho: { a_token: a_token_withdraw, amount_to_withdraw }
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- Deposit to Aave V2 Morpho -->

<h1>1. Deposit to Aave V2 Morpho</h1>
<div>
  <label for="a_token_deposit" title="Enter the Aave V2 aToken contract address as a string.">Aave V2 aToken Contract Address:</label>
  <input type="text" id="a_token_deposit" bind:value={a_token_deposit} placeholder="0xaTokenAddress" />
</div>
<div>
  <label for="amount_to_deposit" title="Enter the amount of the asset to deposit as a string.">Amount of Asset to Deposit:</label>
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount" />
</div>
<button on:click={scheduleDepositToAaveV2Morpho}>Deposit</button>

<!-- Withdraw from Aave V2 Morpho -->

<h1>2. Withdraw from Aave V2 Morpho</h1>
<div>
  <label for="a_token_withdraw" title="Enter the Aave V2 aToken contract address as a string.">Aave V2 aToken Contract Address:</label>
  <input type="text" id="a_token_withdraw" bind:value={a_token_withdraw} placeholder="0xaTokenAddress" />
</div>
<div>
  <label for="amount_to_withdraw" title="Enter the amount of the asset to withdraw as a string.">Amount of Asset to Withdraw:</label>
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Amount" />
</div>
<button on:click={scheduleWithdrawFromAaveV2Morpho}>Withdraw</button>