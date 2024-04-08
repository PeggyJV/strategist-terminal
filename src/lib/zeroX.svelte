<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables for swap operation
    export let tokenIn = "";
    export let tokenOut = "";
    export let amount = "";
    export let swapCallData = "";
    
    // Function to handle the swap operation, calling a backend service
    async function swapWith0x() {
        const result = await invoke("swapWith0x", {
            token_in: tokenIn,
            token_out: tokenOut,
            amount,
            swap_call_data: swapCallData,
        });
        console.log(result);
    }
    </script>
    
    <!-- UI for SwapWith0x Operation -->
    <h1>Swap With 0x</h1>
    
    <div>
        <label for="tokenIn" title="Enter the ERC-20 token contract address to swap from, as a string.">Token In (ERC-20 Contract Address):</label>
        <input type="text" id="tokenIn" bind:value={tokenIn} placeholder="0xTokenInAddress" />
    </div>
    
    <div>
        <label for="tokenOut" title="Enter the ERC-20 token contract address to swap to, as a string.">Token Out (ERC-20 Contract Address):</label>
        <input type="text" id="tokenOut" bind:value={tokenOut} placeholder="0xTokenOutAddress" />
    </div>
    
    <div>
        <label for="amount" title="Enter the amount of the token to swap, as a string.">Amount to Swap:</label>
        <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
    </div>
    
    <div>
        <label for="swapCallData" title="Enter the swap call data, in bytes format.">Swap Call Data:</label>
        <input type="text" id="swapCallData" bind:value={swapCallData} placeholder="Swap Call Data" />
    </div>
    
    <button on:click={swapWith0x}>Execute Swap</button>
    