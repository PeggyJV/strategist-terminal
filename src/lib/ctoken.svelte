<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables
    export let market = "";
    export let amountToDeposit = "";
    export let amountToWithdraw = "";
    
    // Async functions to communicate with the backend
    async function depositToCompound() {
        const result = await invoke("DepositToCompound", { market, amount_to_deposit: amountToDeposit });
        console.log(result);
    }
    
    async function withdrawFromCompound() {
        const result = await invoke("WithdrawFromCompound", { market, amount_to_withdraw: amountToWithdraw });
        console.log(result);
    }
    
    async function claimComp() {
        const result = await invoke("ClaimComp", {});
        console.log(result);
    }
    </script>
    
    <!-- Compound Deposit UI -->
    <h1>Deposit to Compound</h1>
    <div>
        <label for="marketDeposit" title="Enter the compound market, as a string.">Market:</label>
        <input type="text" id="marketDeposit" bind:value={market} placeholder="Enter the Compound market address" />
    </div>
    <div>
        <label for="amountToDeposit" title="Enter the amount to deposit into market, as a string.">Amount to Deposit:</label>
        <input type="text" id="amountToDeposit" bind:value={amountToDeposit} placeholder="Enter amount to deposit" />
    </div>
    <button on:click={depositToCompound}>Deposit</button>
    
    <!-- Compound Withdraw UI -->
    <h1>Withdraw from Compound</h1>
    <div>
        <label for="marketWithdraw" title="Enter the compound market, as a string.">Market:</label>
        <input type="text" id="marketWithdraw" bind:value={market} placeholder="Enter the Compound market address" />
    </div>
    <div>
        <label for="amountToWithdraw" title="Enter the amount to deposit into market, as a string.">Amount to Withdraw:</label>
        <input type="text" id="amountToWithdraw" bind:value={amountToWithdraw} placeholder="Enter amount to withdraw" />
    </div>
    <button on:click={withdrawFromCompound}>Withdraw</button>
    
    <!-- Claim COMP UI -->
    <h1>Claim COMP Rewards</h1>
    <button on:click={claimComp}>Claim COMP</button>
    