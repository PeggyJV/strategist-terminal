<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let tokenId = "";
    export let amount = "";
    import {
        cellarId,
        blockHeight,
        chainId,
        deadline,
    } from "$stores/scheduleRequestStore";

    async function scheduleDeposit() {
        const result = await invoke("aavev3_deposit", { tokenId, amount });
        console.log(result);
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
