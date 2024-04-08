<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let token = "";
    export let amount = "";
    export let asset = "";
    export let use_as_collateral = false;
    export let category_id = 0;
    export let performance_fee = 0;
    export let management_fee = 0;
    export let new_frequency = 0;
    export let new_max_gas = 0;


    /// async functions communicating with protos
    
    /// Base Adaptor Functions - TODO: do we need revokeApproval here, I don't see a `message` within the proto.

    /// Adaptor Specific Functions
    async function updatePerformanceFees() {
        const result = await invoke("UpdatePerformanceFees", { performance_fee });
        console.log(result);
    }

    async function updateManagementFees() {
        const result = await invoke("UpdateManagementFees", { management_fee });
        console.log(result);
    }

    async function changeUpkeepFrequency() {
        const result = await invoke("ChangeUpkeepFrequency", { new_frequency });
        console.log(result);
    }

    async function changeUpkeepMaxGas() {
        const result = await invoke("ChangeUpkeepMaxGas", { new_max_gas });
        console.log(result);
    }  
    
    async function setupMetaData() {
        const result = await invoke("SetupMetaData", { management_fee, performance_fee });
        console.log(result);
    }

    async function AdjustIsolationModeAssetAsCollateral() {
        const result = await invoke("AdjustIsolationModeAssetAsCollateral", { asset, use_as_collateral });
        console.log(result);
    }

    async function addAssetsToReserves() {
        const result = await invoke("AddAssetsToReserves", { amount });
        console.log(result);
    }  

    async function withdrawAssetsFromReserves() {
        const result = await invoke("WithdrawAssetsFromReserves", { amount });
        console.log(result);
    }  

    async function prepareFees() {
        const result = await invoke("PrepareFees", { amount });
        console.log(result);
    }  

</script>

<!-- UpdatePerformanceFees -->

<h1>1. UpdatePerformanceFees </h1>
<div>
    <label for="performance_fee" title="Enter the performance fee as a uint32.">Performance Fee (as a uint32):</label>
    <input type="uint32" id="performance_fee" bind:value={performance_fee} placeholder=0 />
</div>

<button on:click={updatePerformanceFees}>Update Performance Fees</button>

<!-- UpdateManagementFees -->

<h1>2. UpdateManagementFees </h1>
<div>
    <label for="management_fee" title="Enter the management fee as a uint32.">Management Fee (as a uint32):</label>
    <input type="uint32" id="management_fee" bind:value={management_fee} placeholder=0 />
</div>

<button on:click={updateManagementFees}>Update Management Fees</button>

<!-- ChangeUpkeepFrequency -->

<h1>3. ChangeUpkeepFrequency </h1>
<div>
    <label for="new_frequency" title="Enter the new upkeep frequency fee as a uint64.">Upkeep Frequency (as a uint64):</label>
    <input type="uint64" id="new_frequency" bind:value={new_frequency} placeholder=0 />
</div>

<button on:click={changeUpkeepFrequency}>Update New Upkeep Frequency</button>

<!-- ChangeUpkeepMaxGas -->

<h1>4. Change Upkeep Max Gas</h1>
<div>
    <label for="new_max_gas" title="Enter the new_max_gas as a uint64">New Upkeep Max Gas:</label>
    <input type="uint64" id="new_max_gas" bind:value={new_max_gas} placeholder=0 />
</div>
<button on:click={changeUpkeepMaxGas}>Change Upkeep Max Gas</button>

<!-- SetupMetaData -->

<h1>5. Setup MetaData </h1>
<div>
    <label for="management_fee" title="Enter the new MetaData info for management_fee as a uint32">New management_fee:</label>
    <input type="uint32" id="management_fee" bind:value={management_fee} placeholder=0 />
</div>
<button on:click={setupMetaData}>Change MetaData</button>

<!-- AddAssetsToReserves -->

<h1>6. Add Assets to Reserves</h1>
<div>
    <label for="amount" title="Enter the amount of assets to add to the reserves as a string">Assets to Add to Reserves:</label>
    <input type="text" id="amount" bind:value={amount} placeholder=0 />
</div>
<button on:click={addAssetsToReserves}>Add Assets to Reserves</button>

<!-- WithdrawAssetsFromReserves -->

<h1>7. Withdraw Assets From Reserves</h1>
<div>
    <label for="amount" title="Enter the amount of assets to withdraw from the reserves as a string">Assets to Add to Reserves:</label>
    <input type="text" id="amount" bind:value={amount} placeholder=0 />
</div>
<button on:click={withdrawAssetsFromReserves}>Withdraw Assets from Reserves</button>

<!-- PrepareFees -->

<h1>8. Prepare Fees</h1>
<div>
    <label for="amount" title="Enter the amount to prepare as fees as a string">Amount to Prepare as Fees:</label>
    <input type="text" id="amount" bind:value={amount} placeholder=0 />
</div>
<button on:click={prepareFees}>Prepare Fees</button>


<!-- Additional text information at the bottom of the page -->
<div>
    <h2>Specific Strategist Information</h2>
    <p>A. The strategist is meant to add to the Fees and Reserves smart contract and split the fees accordingly using the FeesAndReserves adaptor and FeesAndReserves smart contract itself. </p>
    <p>B. TODO - for more info, see the docs! </p>
    <!-- Empty lines of spaces -->
    <br /><br /><br />
</div>