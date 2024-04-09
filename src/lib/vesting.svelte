<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables
    let vestingContract = "";
    let depositId = "";
    let amount = "";
    export let vestingContractWithdrawAll = "";
    export let withdrawAmount = "";
    export let vestingContractAny = "";
    export let withdrawAmountAny = "";
    
    // Functions
    async function depositToVesting() {
        const result = await invoke("depositToVesting", { vesting_contract: vestingContract, amount });
        console.log(result);
    }
    
    // Function to handle the withdrawal from the vesting contract
    async function withdrawFromVesting() {
        const result = await invoke("withdrawFromVesting", {
            vesting_contract: vestingContract,
            deposit_id: depositId,
            amount: withdrawAmount,
        });
        console.log(result);
    }

    // Function to handle the withdrawal from any deposit in the vesting contract
    async function withdrawAnyFromVesting() {
        const result = await invoke("withdrawAnyFromVesting", {
            vesting_contract: vestingContractAny,
            amount: withdrawAmountAny,
        });
        console.log(result);
    }
        
    async function withdrawAllFromVesting() {
        const result = await invoke("withdrawAllFromVesting", { vesting_contract: vestingContract });
        console.log(result);
    }
    </script>
    
    <!-- Deposit To Vesting -->
    <h1>Deposit To Vesting</h1>
    <div>
        <label for="vestingContract" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
        <input type="text" id="vestingContract" bind:value={vestingContract} placeholder="Vesting contract address">
    </div>
    <div>
        <label for="amount" title="Enter the amount to deposit as a string.">Amount to Deposit:</label>
        <input type="text" id="amount" bind:value={amount} placeholder="Amount">
    </div>
    <button on:click={depositToVesting}>Deposit</button>
    
    <!-- UI for WithdrawFromVesting Operation -->
    <h1>Withdraw From Vesting</h1>

    <div>
        <label for="vestingContract" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
        <input type="text" id="vestingContract" bind:value={vestingContract} placeholder="0xVestingContractAddress" />
    </div>

    <div>
        <label for="depositId" title="Enter the depositID as a string.">Deposit ID:</label>
        <input type="text" id="depositId" bind:value={depositId} placeholder="Deposit ID" />
    </div>

    <div>
        <label for="withdrawAmount" title="Enter the amount to withdraw as a string.">Amount to Withdraw:</label>
        <input type="text" id="withdrawAmount" bind:value={withdrawAmount} placeholder="Amount" />
    </div>

    <button on:click={withdrawFromVesting}>Withdraw</button>
    
    <!-- UI for WithdrawAnyFromVesting Operation -->
    <h1>Withdraw Any From Vesting</h1>

    <div>
        <label for="vestingContractAny" title="Enter the vesting contract address as a string.">Vesting Contract Address:</label>
        <input type="text" id="vestingContractAny" bind:value={vestingContractAny} placeholder="0xVestingContractAddress" />
    </div>

    <div>
        <label for="withdrawAmountAny" title="Enter the amount to withdraw as a string.">Amount to Withdraw:</label>
        <input type="text" id="withdrawAmountAny" bind:value={withdrawAmountAny} placeholder="Amount" />
    </div>

    <button on:click={withdrawAnyFromVesting}>Withdraw Any</button>
    
 <!-- UI for WithdrawAllFromVesting Operation -->
<h1>Withdraw All From Vesting</h1>

<div>
    <label for="vestingContractWithdrawAll" title="Enter the vesting contract address from which to withdraw all.">Vesting Contract Address:</label>
    <input type="text" id="vestingContractWithdrawAll" bind:value={vestingContractWithdrawAll} placeholder="0xVestingContractAddress" />
</div>

<button on:click={withdrawAllFromVesting}>Withdraw All</button>
    