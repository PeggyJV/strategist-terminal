<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let token = "";
    export let amount = "";
    export let underlying_token = "";

    /// async functions communicating with protos
    
    async function scheduleBorrow() {
        const result = await invoke("BorrowFromAave", { token, amount });
        console.log(result);
    }

    async function scheduleRepayDebt() {
        const result = await invoke("RepayAaveDebt", { token, amount });
        console.log(result);
    }

    async function scheduleRepayWithATokens() {
        const result = await invoke("RepayWithATokens", { underlying_token, amount });
        console.log(result);
    }
</script>

<!-- Aave V3 Deposit -->

<h1>1. Aave V3 Borrow</h1>
<div>
    <label for="token" title="Enter the ERC-20 token contract address as a string.">ERC-20 Token Contract Address:</label>
    <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
    <label for="amount" title="Enter the amount of the ERC-20 asset to borrow as a string.">Amount of ERC-20 Asset:</label>
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. Repay Debt with Underlying</h1>
<div>
    <label for="token" title="Enter the ERC-20 token contract address as a string.">ERC-20 Token Contract Address:</label>
    <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
    <label for="amount" title="Enter the amount of the ERC-20 asset to repay as a string.">Amount of ERC-20 Asset:</label>
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleRepayDebt}>Repay Debt with Underlying</button>

<!-- Repay Debt with AToken -->

<h1>3. Repay Debt with AToken</h1>
<div>
    <label for="underlying_token" title="Enter the ERC-20 token contract address as a string.">ERC-20 Token Contract Address:</label>
    <input type="text" id="underlying_token" bind:value={underlying_token} placeholder="0xtoken" />
</div>
<div>
    <label for="amount" title="Enter the amount of the ERC-20 asset to repay as a string.">Amount of AToken to Repay:</label>
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleRepayWithATokens}>Repay Debt with AToken</button>

<!-- Additional text information at the bottom of the page -->
<div>
    <h2>Specific Strategist Information</h2>
    <p>A. Keep in mind regarding isolated markets. If they are isolated, then they may not be usable as collateral for other borrow positions.</p>
    <p>B. See the <a href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture">docs</a> for more information pertaining to this adaptor.</p>
    <!-- Empty lines of spaces -->
    <br /><br /><br />
</div>