<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let erc4626_vault_1 = "";
  let assets_1 = "";

  let erc4626_vault_2 = "";
  let assets_2 = "";

  let Erc4626V1V1Address = "";

  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(Erc4626V1V1Address, "Erc4626V1V1", {
          DepositToVault: {
            erc4626_vault: erc4626_vault_1,
            assets: assets_1
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(Erc4626V1V1Address, "Erc4626V1V1", {
          WithdrawFromVault: {
            erc4626_vault: erc4626_vault_2,
            assets: assets_2
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. Deposit To Vault</h1>
<div>
  <label for="erc4626_vault_1" title="The address of the ERC4626 vault.">The address of the ERC4626 vault:</label>
  <input type="text" id="erc4626_vault_1" bind:value={erc4626_vault_1} placeholder="Address" />
</div>

<div>
  <label for="assets_1" title="The amount of assets to deposit.">The amount of assets to deposit:</label>
  <input type="text" id="assets_1" bind:value={assets_1} placeholder="Amount" />
</div>

<button on:click={scheduleDeposit}>Deposit</button>

<h1>2. Withdraw From Vault</h1>
<div>
  <label for="erc4626_vault_2" title="The address of the ERC4626 vault.">The address of the ERC4626 vault:</label>
  <input type="text" id="erc4626_vault_2" bind:value={erc4626_vault_2} placeholder="Address" />
</div>


<div>
  <label for="assets_2" title="The amount of assets to withdraw">The amount of assets to withdraw:</label>
  <input type="text" id="assets_2" bind:value={assets_2} placeholder="Amount" />
</div>

<button on:click={scheduleWithdraw}>Withdraw</button>
