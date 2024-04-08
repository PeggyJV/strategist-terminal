<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables common to multiple operations
    export let pool = "";
    export let lpToken = "";
    export let orderedUnderlyingTokenAmounts = "";
    export let minLPAmount = "";
    export let gauge = "";
    export let selector = "";
    export let lpTokenAmount = "";
    export let orderedMinimumUnderlyingTokenAmountsOut = "";
    export let useUnderlying = false;
    export let amount = "";

    let poolRL = "";
    let lpTokenRL = "";
    let lpTokenAmountRL = "";
    let orderedMinimumUnderlyingTokenAmountsOutRL = ""; // Assuming a JSON string input for simplicity
    let useUnderlyingRL = false;
    let gaugeRL = "";
    let selectorRL = "";
    
    // Functions for each operation
    async function addLiquidity() {
        const result = await invoke("AddLiquidity", { pool, lp_token: lpToken, ordered_underlying_token_amounts: JSON.parse(orderedUnderlyingTokenAmounts), min_lp_amount: minLPAmount, gauge, selector });
        console.log(result);
    }
    
    async function addLiquidityETH() {
        const result = await invoke("AddLiquidityETH", { pool, lp_token: lpToken, ordered_underlying_token_amounts: JSON.parse(orderedUnderlyingTokenAmounts), min_lp_amount: minLPAmount, use_underlying: useUnderlying, gauge, selector });
        console.log(result);
    }
    
    async function removeLiquidity() {
        const result = await invoke("RemoveLiquidity", { pool, lp_token: lpToken, lp_token_amount: lpTokenAmount, ordered_minimum_underlying_token_amounts_out: JSON.parse(orderedMinimumUnderlyingTokenAmountsOut), gauge, selector });
        console.log(result);
    }
    
    async function removeLiquidityETH() {
        const result = await invoke("RemoveLiquidityETH", { pool, lp_token: lpToken, lp_token_amount: lpTokenAmount, ordered_minimum_underlying_token_amounts_out: JSON.parse(orderedMinimumUnderlyingTokenAmountsOut), use_underlying: useUnderlying, gauge, selector });
        console.log(result);
    }
    
    async function stakeInGauge() {
        const result = await invoke("StakeInGauge", { lp_token: lpToken, gauge, amount, pool, selector });
        console.log(result);
    }
    
    async function unstakeFromGauge() {
        const result = await invoke("UnstakeFromGauge", { gauge, amount });
        console.log(result);
    }
    
    async function claimRewards() {
        const result = await invoke("ClaimRewards", { gauge });
        console.log(result);
    }
    </script>
    
    <!-- AddLiquidity UI Component-->
    <h2>Add Liquidity (Non-ETH)</h2>
    <div>
        <label for="al_pool" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
        <input type="text" id="al_pool" bind:value={pool} placeholder="Curve Pool Address">
    </div>
    <div>
        <label for="al_lpToken" title="Enter the curve pool token address as a string.">LP Token Address:</label>
        <input type="text" id="al_lpToken" bind:value={lpToken} placeholder="LP Token Address">
    </div>
    <div>
        <label for="al_orderedUnderlyingTokenAmounts" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Underlying Token Amounts:</label>
        <input type="text" id="al_orderedUnderlyingTokenAmounts" bind:value={orderedUnderlyingTokenAmounts} placeholder="e.g., [100, 200]">
    </div>
    <div>
        <label for="al_minLPAmount" title="Enter the minimum amount of LP tokens to receive, as a string.">Min LP Amount:</label>
        <input type="text" id="al_minLPAmount" bind:value={minLPAmount} placeholder="Min LP Amount">
    </div>
    <div>
        <label for="al_gauge" title="Enter the curve gauge address as a string.">Curve Gauge Address:</label>
        <input type="text" id="al_gauge" bind:value={gauge} placeholder="Gauge Address">
    </div>
    <div>
        <label for="al_selector" title="Enter the selector as a string.">Selector:</label>
        <input type="text" id="al_selector" bind:value={selector} placeholder="Selector">
    </div>
    <button on:click={addLiquidity}>Add Liquidity</button>

    <!-- AddLiquidityETH UI Component -->
    <h2>Add Liquidity with ETH</h2>
    <div>
        <label for="aleth_pool" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
        <input type="text" id="aleth_pool" bind:value={pool} placeholder="Curve Pool Address">
    </div>
    <div>
        <label for="aleth_lpToken" title="Enter the Curve LP Token  address as a string.">LP Token Address:</label>
        <input type="text" id="aleth_lpToken" bind:value={lpToken} placeholder="LP Token Address">
    </div>
    <div>
        <label for="aleth_orderedUnderlyingTokenAmounts" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Underlying Token Amounts:</label>
        <input type="text" id="aleth_orderedUnderlyingTokenAmounts" bind:value={orderedUnderlyingTokenAmounts} placeholder="e.g., [100, 200]">
    </div>
    <div>
        <label for="aleth_minLPAmount" title="Enter the minimum amount of LP tokens to receive, as a string.">Min LP Amount:</label>
        <input type="text" id="aleth_minLPAmount" bind:value={minLPAmount} placeholder="Min LP Amount">
    </div>
    <div>
        <label for="aleth_useUnderlying" title="Enter a bool for whether to use the underlying asset or the wrapped asset.">Use Underlying:</label>
        <input type="checkbox" id="aleth_useUnderlying" bind:checked={useUnderlying}>
    </div>
    <div>
        <label for="aleth_gauge" title="Enter the respective curve gauge address as a string.">Curve Gauge Address:</label>
        <input type="text" id="aleth_gauge" bind:value={gauge} placeholder="Gauge Address">
    </div>
    <div>
        <label for="aleth_selector" title="Enter the selector as a string.">Selector:</label>
        <input type="text" id="aleth_selector" bind:value={selector} placeholder="Selector">
    </div>
    <button on:click={addLiquidityETH}>Add Liquidity with ETH</button>

    <!-- RemoveLiquidity UI Component -->
    <h2>Remove Liquidity (Non-ETH)</h2>
    <div>
        <label for="rl_pool" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
        <input type="text" id="rl_pool" bind:value={pool} placeholder="Enter Curve Pool Address">
    </div>
    <div>
        <label for="rl_lpToken" title="Enter the curve pool token address as a string.">LP Token Address:</label>
        <input type="text" id="rl_lpToken" bind:value={lpToken} placeholder="Enter LP Token Address">
    </div>
    <div>
        <label for="rl_lpTokenAmount" title="Enter the Base Reward Pool address as a string.">LP Token Amount:</label>
        <input type="text" id="rl_lpTokenAmount" bind:value={lpTokenAmount} placeholder="Enter LP Token Amount to Remove">
    </div>
    <div>
        <label for="rl_orderedMinimumUnderlyingTokenAmountsOut" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Minimum Token Amounts Out:</label>
        <input type="text" id="rl_orderedMinimumUnderlyingTokenAmountsOut" bind:value={orderedMinimumUnderlyingTokenAmountsOut} placeholder="e.g., [50, 100]">
    </div>
    <div>
        <label for="rl_gauge" title="Enter the curve gauge address as a string.">Curve Gauge Address:</label>
        <input type="text" id="rl_gauge" bind:value={gauge} placeholder="Enter Gauge Address">
    </div>
    <div>
        <label for="rl_selector" title="Enter the selector as a string.">Function Selector:</label>
        <input type="text" id="rl_selector" bind:value={selector} placeholder="Enter Selector">
    </div>
    <button on:click={removeLiquidity}>Remove Liquidity</button>

    <!-- RemoveLiquidityETH UI Component -->
    <h2>Remove Liquidity with ETH</h2>
    <div>
        <label for="rleth_useUnderlying" title="Enter bool indicating whether or not to add a true bool to the end of abi.encoded `removeLiquidity` call.">Use Underlying:</label>
        <input type="checkbox" id="rleth_useUnderlying" bind:checked={useUnderlying}>
    </div>
    <div>
        <label for="poolRL">Curve Pool Address:</label>
        <input type="text" id="poolRL" bind:value={poolRL} placeholder="Enter Curve Pool Address">
    </div>
    <div>
        <label for="lpTokenRL">LP Token Address:</label>
        <input type="text" id="lpTokenRL" bind:value={lpTokenRL} placeholder="Enter LP Token Address">
    </div>
    <div>
        <label for="lpTokenAmountRL">LP Token Amount:</label>
        <input type="text" id="lpTokenAmountRL" bind:value={lpTokenAmountRL} placeholder="Enter LP Token Amount to Remove">
    </div>
    <div>
        <label for="orderedMinimumUnderlyingTokenAmountsOutRL">Minimum Underlying Token Amounts Out:</label>
        <input type="text" id="orderedMinimumUnderlyingTokenAmountsOutRL" bind:value={orderedMinimumUnderlyingTokenAmountsOutRL} placeholder="Enter JSON array of min amounts out">
    </div>
    <div>
        <label for="gaugeRL">Curve Gauge Address:</label>
        <input type="text" id="gaugeRL" bind:value={gaugeRL} placeholder="Enter Gauge Address">
    </div>
    <div>
        <label for="selectorRL">Function Selector:</label>
        <input type="text" id="selectorRL" bind:value={selectorRL} placeholder="Enter Function Selector">
    </div>
    <button on:click={removeLiquidityETH}>Remove Liquidity with ETH</button>

    <!-- StakeInGauge UI Component -->
    <h2>Stake In Gauge</h2>
    <div>
        <label for="sig_lpToken" title="Enter the address of the LP token as a string.">LP Token Address:</label>
        <input type="text" id="sig_lpToken" bind:value={lpToken} placeholder="Enter LP Token Address">
    </div>
    <div>
        <label for="sig_gauge" title="Enter the Curve Pool Gauge address as a string.">Gauge Address:</label>
        <input type="text" id="sig_gauge" bind:value={gauge} placeholder="Enter Gauge Address">
    </div>
    <div>
        <label for="sig_amount" title="Enter the amount of LP tokens to stake, as a string.">Amount:</label>
        <input type="text" id="sig_amount" bind:value={amount} placeholder="Enter Amount to Stake">
    </div>
    <div>
        <label for="stake_pool" title="Enter the address of the Curve Pool as a string.">Curve Pool Address:</label>
        <input type="text" id="stake_pool" bind:value={pool} placeholder="Enter Curve Pool Address">
    </div>
    <div>
        <label for="stake_selector" title="Enter the selector for the function to call, as a string.">Function Selector:</label>
        <input type="text" id="stake_selector" bind:value={selector} placeholder="Enter Function Selector">
    </div>
    <button on:click={stakeInGauge}>Stake in Gauge</button>

    <!-- UnstakeFromGauge UI Component -->
    <h2>Unstake From Gauge</h2>
    <div>
        <label for="ufg_gauge" title="Enter the Curve Gauge address as a string.">Gauge Address:</label>
        <input type="text" id="ufg_gauge" bind:value={gauge} placeholder="Enter Gauge Address">
    </div>
    <div>
        <label for="ufg_amount" title="Enter the amount of LP tokens to unstake, as a string.">Amount:</label>
        <input type="text" id="ufg_amount" bind:value={amount} placeholder="Enter Amount to Unstake">
    </div>
    <button on:click={unstakeFromGauge}>Unstake from Gauge</button>

    <!-- ClaimRewards UI Component -->
    <h2>Claim Rewards</h2>
    <div>
        <label for="cr_gauge" title="Enter the address of the Curve Gauge as a string.">Gauge Address:</label>
        <input type="text" id="cr_gauge" bind:value={gauge} placeholder="Enter Gauge Address">
    </div>
    <button on:click={claimRewards}>Claim Rewards</button>    