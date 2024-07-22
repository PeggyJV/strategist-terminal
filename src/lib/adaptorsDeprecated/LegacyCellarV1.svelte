<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let cellar_deposit = "";
  let assets_deposit = "";
  let oracle_deposit = "";

  let cellar_withdraw = "";
  let assets_withdraw = "";
  let oracle_withdraw  = "";

  const LegacyCellarV1Address = "";

  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(LegacyCellarV1Address, "LegacyCellarV1", {
          DepositToCellar: {
            cellar: cellar_deposit,
            assets: assets_deposit,
            oracle: oracle_deposit
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(LegacyCellarV1Address, "LegacyCellarV1", {
          WithdrawFromCellar: {
            cellar: cellar_withdraw,
            assets: assets_withdraw,
            oracle: oracle_withdraw
          },
        }),
      );
      return callQueue;
    });
  }
</script>

<h1>1. Deposit To Cellar</h1>
<div>
  <label for="cellar_deposit" title="Cellar.">Cellar:</label>
  <input type="text" id="cellar_deposit" bind:value={cellar_deposit} placeholder="Cellar" />
</div>

<div>
  <label for="assets_deposit" title="Assets.">Assets to deposit:</label>
  <input type="text" id="assets_deposit" bind:value={assets_deposit} placeholder="Assets" />
</div>

<div>
  <label for="oracle_deposit" title="Oracle.">Oracle:</label>
  <input type="text" id="oracle_deposit" bind:value={oracle_deposit} placeholder="Oracle" />
</div>

<button on:click={scheduleDeposit}>Deposit</button>

<h1>2. Withdraw From Cellar</h1>
<div>
  <label for="cellar_withdraw" title="Cellar.">Cellar:</label>
  <input type="text" id="cellar_withdraw" bind:value={cellar_withdraw} placeholder="Cellar" />
</div>

<div>
  <label for="assets_withdraw" title="Assets.">Assets to withdraw:</label>
  <input type="text" id="assets_withdraw" bind:value={assets_withdraw} placeholder="Assets" />
</div>

<div>
  <label for="oracle_withdraw" title="Oracle.">Oracle:</label>
  <input type="text" id="oracle_withdraw" bind:value={oracle_withdraw} placeholder="Oracle" />
</div>

<button on:click={scheduleWithdraw}>Withdraw</button>
