<script>
    import { invoke } from "@tauri-apps/api/tauri";

    /// vars
    export let underlying_to_borrow = "";
    export let amount_to_borrow = "";
    export let max_iterations_borrow = "";
    export let token_to_repay = "";
    export let amount_to_repay = "";

    /// async functions communicating with protos

    async function scheduleBorrowFromAaveV3Morpho() {
        const result = await invoke("BorrowFromAaveV3Morpho", { underlying: underlying_to_borrow, amount_to_borrow, max_iterations: max_iterations_borrow });
        console.log(result);
    }

    async function scheduleRepayAaveV3MorphoDebt() {
        const result = await invoke("RepayAaveV3MorphoDebt", { token_to_repay, amount_to_repay });
        console.log(result);
    }
</script>

<!-- Borrow from Aave V3 Morpho -->

<h1>1. Borrow from Aave V3 Morpho</h1>
<div>
    <label for="underlying_to_borrow" title="Enter the underlying asset to borrow as a string.">Underlying Asset:</label>
    <input type="text" id="underlying_to_borrow" bind:value={underlying_to_borrow} placeholder="Underlying Asset" />
</div>
<div>
    <label for="amount_to_borrow" title="Enter the amount of the underlying asset to borrow as a string.">Amount to Borrow:</label>
    <input type="text" id="amount_to_borrow" bind:value={amount_to_borrow} placeholder="Amount to Borrow" />
</div>
<div>
    <label for="max_iterations_borrow" title="Enter the maximum number of iterations to perform as a string.">Max Iterations:</label>
    <input type="text" id="max_iterations_borrow" bind:value={max_iterations_borrow} placeholder="Max Iterations" />
</div>
<button on:click={scheduleBorrowFromAaveV3Morpho}>Borrow</button>

<!-- Repay Aave V3 Morpho Debt -->

<h1>2. Repay Aave V3 Morpho Debt</h1>
<div>
    <label for="token_to_repay" title="Enter the ERC-20 token contract address to repay as a string.">Token to Repay:</label>
    <input type="text" id="token_to_repay" bind:value={token_to_repay} placeholder="Token to Repay" />
</div>
<div>
    <label for="amount_to_repay" title="Enter the amount of the token to repay as a string.">Amount to Repay:</label>
    <input type="text" id="amount_to_repay" bind:value={amount_to_repay} placeholder="Amount to Repay" />
</div>
<button on:click={scheduleRepayAaveV3MorphoDebt}>Repay</button>