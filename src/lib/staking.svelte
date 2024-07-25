<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Shared Variables
    let amount = "";
    let minAmountOut = "";
    let wildcard = "";
    let id = "";
    let depositAsset = "";
    let wrapAmount = "";
    let wrapMinAmountOut = "";
    let wrapWildcard = "";
    let unwrapAmount = "";
    let unwrapMinAmountOut = "";
    let unwrapWildcard = "";
    let mintERC20DepositAsset = "";
    let mintERC20Amount = "";
    let mintERC20MinAmountOut = "";
    let mintERC20Wildcard = "";
    let removeClaimedRequestId = "";
    let removeClaimedRequestWildcard = "";
    
    // Operation-specific Variables
    let requestBurnAmount = "";
    let completeBurnId = "";
    let completeBurnMinAmountOut = "";
    let cancelBurnId = "";
    let wrapAmount = "";
    let unwrapAmount = "";
    let mintERC20Amount = "";
    let mintERC20DepositAsset = "";
    let removeClaimedRequestId = "";
    
    // Functions
    async function mint() {
        const result = await invoke("mint", { amount, min_amount_out: minAmountOut, wildcard });
        console.log(result);
    }
    
    async function requestBurn() {
        const result = await invoke("requestBurn", { amount: requestBurnAmount, wildcard });
        console.log(result);
    }
    
    async function completeBurn() {
        const result = await invoke("completeBurn", { id: completeBurnId, min_amount_out: completeBurnMinAmountOut, wildcard });
        console.log(result);
    }
    
    async function cancelBurn() {
        const result = await invoke("cancelBurn", { id: cancelBurnId, wildcard });
        console.log(result);
    }
    
    async function wrap() {
        const result = await invoke("wrap", { amount: wrapAmount, min_amount_out: minAmountOut, wildcard });
        console.log(result);
    }
    
    async function unwrap() {
        const result = await invoke("unwrap", { amount: unwrapAmount, min_amount_out: minAmountOut, wildcard });
        console.log(result);
    }
    
    async function mintERC20() {
        const result = await invoke("mintERC20", { deposit_asset: mintERC20DepositAsset, amount: mintERC20Amount, min_amount_out: minAmountOut, wildcard });
        console.log(result);
    }
    
    async function removeClaimedRequest() {
        const result = await invoke("removeClaimedRequest", { id: removeClaimedRequestId, wildcard });
        console.log(result);
    }
    </script>
    
    <!-- Mint UI Component -->
    <h2>Mint</h2>
    <div>
        <label for="mint_amount" title="Enter the amount of native asset to use for minting, as a string.">Amount:</label>
        <input type="text" id="mint_amount" bind:value={amount} placeholder="Amount">
    </div>
    <div>
        <label for="mint_minAmountOut" title="Enter the minimum amount of the asset to receive, as a string.">Min Amount Out:</label>
        <input type="text" id="mint_minAmountOut" bind:value={minAmountOut} placeholder="Min Amount Out">
    </div>
    <div>
        <label for="mint_wildcard" title="Enter the Arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="mint_wildcard" bind:value={wildcard} placeholder="Wildcard Data">
    </div>
    <button on:click={mint}>Mint</button>
    
    <!-- RequestBurn UI Component -->
    <h2>Request Burn</h2>
    <div>
        <label for="requestBurn_amount" title="Enter the amount of derivative to burn/withdraw as a string.">Amount:</label>
        <input type="text" id="requestBurn_amount" bind:value={requestBurnAmount} placeholder="Amount">
    </div>
    <div>
        <label for="requestBurn_wildcard" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors, as a string.">Wildcard Data:</label>
        <input type="text" id="requestBurn_wildcard" bind:value={wildcard} placeholder="Wildcard Data">
    </div>
    <button on:click={requestBurn}>Request Burn</button>
    
    <!-- CompleteBurn UI Component -->
    <h2>Complete Burn</h2>
    <div>
        <label for="completeBurn_id" title="Enter the request ID as a string.">Burn Request ID:</label>
        <input type="text" id="completeBurn_id" bind:value={completeBurnId} placeholder="Burn Request ID">
    </div>
    <div>
        <label for="completeBurn_minAmountOut" title="Enter the  minimum amount of the asset to receive as a string.">Min Amount Out:</label>
        <input type="text" id="completeBurn_minAmountOut" bind:value={completeBurnMinAmountOut} placeholder="Min Amount Out">
    </div>
    <div>
        <label for="completeBurn_wildcard" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="completeBurn_wildcard" bind:value={wildcard} placeholder="Wildcard Data">
    </div>
    <button on:click={completeBurn}>Complete Burn</button>
    
    <!-- CancelBurn UI Component -->
    <h2>Cancel Burn</h2>
    <div>
        <label for="cancelBurn_id" title="Enter the id of the burn request as a string.">Burn Request ID:</label>
        <input type="text" id="cancelBurn_id" bind:value={cancelBurnId} placeholder="Burn Request ID">
    </div>
    <div>
        <label for="cancelBurn_wildcard" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="cancelBurn_wildcard" bind:value={wildcard} placeholder="Wildcard Data">
    </div>
    <button on:click={cancelBurn}>Cancel Burn</button>
    
    <!-- Wrap UI Component -->
    <h2>Wrap Derivative Asset</h2>
    <div>
        <label for="wrapAmount" title="Enter the amount of derivative to wrap, as a string.">Amount to Wrap:</label>
        <input type="text" id="wrapAmount" bind:value={wrapAmount} placeholder="Enter amount">
    </div>
    <div>
        <label for="wrapMinAmountOut" title="Enter the minimum amount of the asset to receive as a string.">Min Amount Out:</label>
        <input type="text" id="wrapMinAmountOut" bind:value={wrapMinAmountOut} placeholder="Enter minimum amount out">
    </div>
    <div>
        <label for="wrapWildcard" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="wrapWildcard" bind:value={wrapWildcard} placeholder="Enter wildcard data">
    </div>
    <button on:click={wrap}>Wrap</button>
    
    <!-- Unwrap UI Component -->
    <h2>Unwrap Derivative Asset</h2>
    <div>
        <label for="unwrapAmount" title="Enter the amount of wrapped derivative to unwrap as a string.">Amount to Unwrap:</label>
        <input type="text" id="unwrapAmount" bind:value={unwrapAmount} placeholder="Enter amount">
    </div>
    <div>
        <label for="unwrapMinAmountOut" title="Enter the minimum amount of the asset to receive as a string.">Min Amount Out:</label>
        <input type="text" id="unwrapMinAmountOut" bind:value={unwrapMinAmountOut} placeholder="Enter minimum amount out">
    </div>
    <div>
        <label for="unwrapWildcard" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="unwrapWildcard" bind:value={unwrapWildcard} placeholder="Enter wildcard data">
    </div>
    <button on:click={unwrap}>Unwrap</button>
    
    <!-- MintERC20 UI Component -->
    <h2>Mint Derivative Asset with ERC20</h2>
    <div>
        <label for="mintERC20DepositAsset" title="Enter the ERC20 asset to mint with as a string.">Deposit Asset Address:</label>
        <input type="text" id="mintERC20DepositAsset" bind:value={mintERC20DepositAsset} placeholder="Enter deposit asset address">
    </div>
    <div>
        <label for="mintERC20Amount" title="Enter the the amount of `depositAsset` to mint with as a string.">Amount:</label>
        <input type="text" id="mintERC20Amount" bind:value={mintERC20Amount} placeholder="Enter amount">
    </div>
    <div>
        <label for="mintERC20MinAmountOut" title="Enter the the minimum amount of derivative out as a string.">Min Amount Out:</label>
        <input type="text" id="mintERC20MinAmountOut" bind:value={mintERC20MinAmountOut} placeholder="Enter minimum amount out">
    </div>
    <div>
        <label for="mintERC20Wildcard" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
        <input type="text" id="mintERC20Wildcard" bind:value={mintERC20Wildcard} placeholder="Enter wildcard data">
    </div>
    <button on:click={mintERC20}>Mint with ERC20</button>
    
    <!-- RemoveClaimedRequest UI Component -->
    <h2>Remove Claimed Request</h2>
    <div>
        <label for="removeClaimedRequestId" title="Enter the request ID to remove, as a string.">Request ID:</label>
        <input type="text" id="removeClaimedRequestId" bind:value={removeClaimedRequestId} placeholder="Enter request ID">
    </div>
    <div>
        <label for="removeClaimedRequestWildcard" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors, as a string.">Wildcard Data:</label>
        <input type="text" id="removeClaimedRequestWildcard" bind:value={removeClaimedRequestWildcard} placeholder="Enter wildcard data">
    </div>
    <button on:click={removeClaimedRequest}>Remove Claimed Request</button>
 