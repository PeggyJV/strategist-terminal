<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let vesting_contract_deposit = "";
  let amount_deposit = "";

  let vesting_contract_withdraw = "";
  let deposit_id = "";
  let amount_withdraw = "";

  let vesting_contract_any = "";
  let withdraw_amount_any = "";

  let vesting_contract_withdraw_all = "";



  const VestingSimpleV2Address = "";


  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(VestingSimpleV2Address, "VestingSimpleV2", {
          DepositToVesting: {
            vesting_contract: vesting_contract_deposit,
            amount: amount_deposit,
          },
        }),
      );
      return callQueue;
    });
  }


  // Function to handle the withdrawal from the vesting contract
  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(VestingSimpleV2Address, "VestingSimpleV2", {
          WithdrawFromVesting: {
            vesting_contract: vesting_contract_withdraw,
            deposit_id,
            amount: amount_withdraw
          },
        }),
      );
      return callQueue;
    });
  }

  // Function to handle the withdrawal from any deposit in the vesting contract
  function scheduleWithdrawAny() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(VestingSimpleV2Address, "VestingSimpleV2", {
          WithdrawAnyFromVesting: {
            vesting_contract: vesting_contract_any,
            amount: withdraw_amount_any,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdrawAll() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(VestingSimpleV2Address, "VestingSimpleV2", {
          WithdrawAllFromVesting: {
            vesting_contract: vesting_contract_withdraw_all,
          },
        }),
      );
      return callQueue;
    });
  }


</script>

<!-- Deposit To Vesting -->
<h1>1. Deposit To Vesting</h1>
<div>
  <label for="vesting_contract_deposit" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
  <input type="text" id="vesting_contract_deposit" bind:value={vesting_contract_deposit} placeholder="Vesting contract address">
</div>
<div>
  <label for="amount_deposit" title="Enter the amount to deposit as a string.">Amount to Deposit:</label>
  <input type="text" id="amount_deposit" bind:value={amount_deposit} placeholder="Amount">
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- UI for WithdrawFromVesting Operation -->
<h1>2. Withdraw From Vesting</h1>

<div>
  <label for="vesting_contract_withdraw" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
  <input type="text" id="vesting_contract_withdraw" bind:value={vesting_contract_withdraw} placeholder="Vesting contract address" />
</div>

<div>
  <label for="deposit_id" title="Enter the depositID as a string.">Deposit ID:</label>
  <input type="text" id="deposit_id" bind:value={deposit_id} placeholder="Deposit ID" />
</div>

<div>
  <label for="amount_withdraw" title="Enter the amount to withdraw as a string.">Amount to Withdraw:</label>
  <input type="text" id="amount_withdraw" bind:value={amount_withdraw} placeholder="Amount" />
</div>

<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- UI for WithdrawAnyFromVesting Operation -->
<h1>3. Withdraw Any From Vesting</h1>

<div>
  <label for="vesting_contract_any" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
  <input type="text" id="vesting_contract_any" bind:value={vesting_contract_any} placeholder="Vesting contract address" />
</div>

<div>
  <label for="withdraw_amount_any" title="Enter the amount to withdraw as a string.">Amount to Withdraw:</label>
  <input type="text" id="withdraw_amount_any" bind:value={withdraw_amount_any} placeholder="Amount" />
</div>

<button on:click={scheduleWithdrawAny}>Withdraw Any</button>

<!-- UI for WithdrawAllFromVesting Operation -->
<h1>4. Withdraw All From Vesting</h1>

<div>
  <label for="vesting_contract_withdraw_all" title="Enter the vesting contract address from which to withdraw all.">Vesting Contract Address:</label>
  <input type="text" id="vesting_contract_withdraw_all" bind:value={vesting_contract_withdraw_all} placeholder="Vesting contract address" />
</div>

<button on:click={scheduleWithdrawAll}>Withdraw All</button>
