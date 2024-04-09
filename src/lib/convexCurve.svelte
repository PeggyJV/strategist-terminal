<script>
    import { invoke } from "@tauri-apps/api/tauri";
    
    // Variables for DepositLPTInConvexAndStake
    export let pid = "";
    export let baseRewardPool = "";
    export let lpt = "";
    export let pool = "";
    export let selector = "";
    export let amountToDeposit = "";
    
    // Variables for WithdrawFromBaseRewardPoolAsLPT
    export let amountToWithdraw = "";
    export let claim = false;
    
    // Variables for GetRewards
    export let claimExtras = false;
    
    // Function to deposit and stake LPTs into Convex
    async function depositLPTInConvexAndStake() {
        const result = await invoke("depositLPTInConvexAndStake", { pid, base_reward_pool: baseRewardPool, lpt, pool, selector, amount_to_deposit: amountToDeposit });
        console.log(result);
    }
    
    // Function to withdraw from Convex
    async function withdrawFromBaseRewardPoolAsLPT() {
        const result = await invoke("withdrawFromBaseRewardPoolAsLPT", { base_reward_pool: baseRewardPool, amount_to_withdraw: amountToWithdraw, claim });
        console.log(result);
    }
    
    // Function to get rewards from Convex
    async function getRewards() {
        const result = await invoke("getRewards", { base_reward_pool: baseRewardPool, claim_extras: claimExtras });
        console.log(result);
    }
    </script>
    
    <!-- DepositLPTInConvexAndStake UI -->
    <h1>Deposit and Stake LPT in Convex</h1>
    <div>
        <label for="pid" title="Enter the Pool ID (PID) as a string.">PID:</label>
        <input type="text" id="pid" bind:value={pid} placeholder="Enter PID">
    </div>
    <div>
        <label for="baseRewardPool" title="Enter the Base Reward Pool address as a string.">Base Reward Pool:</label>
        <input type="text" id="baseRewardPool" bind:value={baseRewardPool} placeholder="Enter Base Reward Pool Address">
    </div>
    <!-- Include additional fields for lpt, pool, selector, and amountToDeposit -->
    <button on:click={depositLPTInConvexAndStake}>Deposit and Stake</button>
    
    <!-- WithdrawFromBaseRewardPoolAsLPT UI -->
    <h1>Withdraw from Convex</h1>
    <div>
        <label for="amountToWithdraw" title="Enter the amount to withdraw from ConvexCurve market, as a string.">Amount to Withdraw:</label>
        <input type="text" id="amountToWithdraw" bind:value={amountToWithdraw} placeholder="Enter amount to withdraw">
    </div>
    <div>
        <label for="claim" title="Enter whether or not to rewards associated to respective ConvexCurve market position, as a bool.">Claim Rewards:</label>
        <input type="checkbox" id="claim" bind:checked={claim}>
    </div>
    <button on:click={withdrawFromBaseRewardPoolAsLPT}>Withdraw</button>
    
    <!-- GetRewards UI -->
    <h1>Get Rewards from Convex</h1>
    <div>
        <label for="claimExtras" title="Enter whether or not to claim extra rewards in addition to baseRewards with respective ConvexCurve market, as a bool.">Claim Extras:</label>
        <input type="checkbox" id="claimExtras" bind:checked={claimExtras}>
    </div>
    <button on:click={getRewards}>Get Rewards</button>
    