<script>
    export let tokenId = "";
    export let amount = "";
    const aaveDepositAdaptorAddress = "";

    import { CellarCall, queue } from "$stores/AdapterQueue.js"

    async function scheduleDeposit() {
        // const result = await invoke("aavev3_deposit", { tokenId, amount });
        // console.log(result);

      queue.update((callQueue) => {
        callQueue.push(
          new CellarCall(aaveDepositAdaptorAddress, "AaveATokenAdaptorV1", {
            DepositToAave: {
              tokenId,
              amount,
            },
          }),
        );
        return callQueue;
      });
    }
</script>

<h1>Aave V3 Deposit</h1>
ERC-20 Token Contract Address<input
    bind:value={tokenId}
    placeholder="0xTokenID"
/>
<br />
Amount of ERC-20 Asset <input bind:value={amount} placeholder="Amount" />
<br />
<button on:click={scheduleDeposit}>Deposit</button>
