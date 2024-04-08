<script>
    import { invoke } from "@tauri-apps/api/tauri";

    /// vars
    export let token_to_deposit = "";
    export let amount_to_deposit = "";
    export let token_to_withdraw = "";
    export let amount_to_withdraw = "";

    /// async functions communicating with protos

    async function scheduleDepositToAaveV3Morpho() {
        const result = await invoke("DepositToAaveV3Morpho", { token_to_deposit, amount_to_deposit });
        console.log(result);
    }

    async function scheduleWithdrawFromAaveV3Morpho() {
        const result = await invoke("WithdrawFromAaveV3Morpho", { token_to_withdraw, amount_to_withdraw });
        console.log(result);
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
<button on:click={scheduleWithdrawFromAaveV3Morpho}>Withdraw</button>