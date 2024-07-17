<script lang="ts">
    import { queue, CellarCall } from "$stores/AdapterQueue";

    let  token_id = "";
    let amount = "";
    const aaveDepositAdaptorAddress = "";

    function scheduleDeposit() {
        // const result = await invoke("aavev3_deposit", { tokenId, amount });
        // console.log(result);

      queue.update((callQueue) => {
        callQueue.push(
          new CellarCall(aaveDepositAdaptorAddress, "AaveATokenAdaptorV1", {
            DepositToAave: {
              token_id,
              amount,
            },
          }),
        );
        return callQueue;
      });
    }
</script>

<h1>Aave V3 Deposit</h1>
  ERC-20 Token Contract Address
<input
    bind:value={token_id}
    placeholder="0xTokenID"
/>
<br />
Amount of ERC-20 Asset <input bind:value={amount} placeholder="Amount" />
<br />
<button on:click={scheduleDeposit}>Deposit</button>
