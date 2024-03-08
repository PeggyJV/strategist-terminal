<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import * as steward from "steward-proto-ts";
    export let tokenId = "";
    export let amount = "";
</script>

<h1>Aave V3 Deposit</h1>
<input bind:value={tokenId} placeholder="0xTokenID" />
<input bind:value={amount} placeholder="Amount" />

<button
    on:click={async () => {
        let deposit = new steward.AaveV3ATokenAdaptorV1.DepositToAave();
        deposit.setToken(tokenId);
        deposit.setAmount(amount);
        console.log(deposit);
    }}>Deposit</button
>
