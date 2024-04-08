<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables for SwapWithUniV2
    let pathUniV2 = [""];
    let amountUniV2 = "";
    let amountOutMinUniV2 = "";
    
    // Variables for SwapWithUniV3
    let pathUniV3 = [""];
    let poolFeesUniV3 = [0];
    let amountUniV3 = "";
    let amountOutMinUniV3 = "";
    
    // Function to perform a swap on Uniswap V2
    async function swapWithUniV2() {
        const result = await invoke("SwapWithUniV2", { path: pathUniV2, amount: amountUniV2, amount_out_min: amountOutMinUniV2 });
        console.log(result);
    }
    
    // Function to perform a swap on Uniswap V3
    async function swapWithUniV3() {
        const result = await invoke("SwapWithUniV3", { path: pathUniV3, pool_fees: poolFeesUniV3, amount: amountUniV3, amount_out_min: amountOutMinUniV3 });
        console.log(result);
    }
    </script>
    
    <!-- UI for SwapWithUniV2 -->
    <h1>Swap with Uniswap V2</h1>
    <div>
        <label for="pathUniV2" title="Enter the address swap path, using an array of strings">Path (separated by commas):</label>
        <input type="text" id="pathUniV2" bind:value={pathUniV2} placeholder="Enter the token addresses path">
    </div>
    <div>
        <label for="amountUniV2" title="Enter the amount of tokensIn you want to swap, as a string.">Amount:</label>
        <input type="text" id="amountUniV2" bind:value={amountUniV2} placeholder="Amount of token to swap">
    </div>
    <div>
        <label for="amountOutMinUniV2" title="Enter the minimum amount of tokenOut you desire, as a string.">Minimum Amount Out:</label>
        <input type="text" id="amountOutMinUniV2" bind:value={amountOutMinUniV2} placeholder="Minimum amount to receive">
    </div>
    <button on:click={swapWithUniV2}>Swap on UniV2</button>
    
    <h1>Swap with Uniswap V3</h1>
    <div>
        <label for="pathUniV3" title="Enter the address swap path, using an array of strings">Path (separated by commas):</label>
        <input type="text" id="pathUniV3" bind:value={pathUniV3} placeholder="Enter the token addresses path">
    </div>
    <div>
        <label for="poolFeesUniV3" title="Enter the pool fees that will help distinguish which pool to interact with, represented as an array of strings.">Pool Fees (separated by commas):</label>
        <input type="text" id="poolFeesUniV3" bind:value={poolFeesUniV3} placeholder="Enter pool fees for each path">
    </div>
    <div>
        <label for="amountUniV3" title="Enter the amount of tokensIn you want to swap, as a string.">Amount:</label>
        <input type="text" id="amountUniV3" bind:value={amountUniV3} placeholder="Amount of token to swap">
    </div>
    <div>
        <label for="amountOutMinUniV3" title="Enter the minimum amount of tokenOut you desire, as a string.">Minimum Amount Out:</label>
        <input type="text" id="amountOutMinUniV3" bind:value={amountOutMinUniV3} placeholder="Minimum amount to receive">
    </div>
    <button on:click={swapWithUniV3}>Swap on UniV3</button>
        