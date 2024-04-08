<script>
    import { invoke } from "@tauri-apps/api/tauri";
  
    // Variables for SingleSwap
    let pool_id = "";
    let kind = ""; // This would be an enum in the actual implementation
    let asset_in = "";
    let asset_out = "";
    let amount = "";
    let user_data = ""; // This needs to be handled as bytes
  
    // Variables for SwapData
    let min_amounts_for_swaps = []; // This would be an array of strings
    let swap_deadlines = []; // This would be an array of strings
  
    // Variables for JoinPool
    let target_bpt = "";
    let swaps_before_join = []; // This would require a complex input method for multiple SingleSwaps
    let swap_data = {}; // This would be the input for SwapData
    let minimum_bpt = "";
  
    // Variables for ExitPoolRequest
    let assets = []; // Array of strings
    let min_amounts_out = []; // Array of strings
    let exit_user_data = ""; // As bytes
    let to_internal_balance = false;
  
    // Variables for ExitPool
    let exit_target_bpt = "";
    let swaps_after_exit = []; // Complex input for multiple SingleSwaps
    let exit_swap_data = {}; // Input for SwapData
    let request = {}; // Input for ExitPoolRequest
  
    // Variables for StakeBPT
    let bpt = "";
    let liquidity_gauge = "";
    let amount_in = "";
  
    // Variables for UnstakeBPT
    let unstake_bpt = "";
    let unstake_liquidity_gauge = "";
    let amount_out = "";
  
    // Variables for ClaimRewards
    let gauge = "";
  
    // Async functions for invoking the respective operations
    async function performSingleSwap() {
      const result = await invoke("SingleSwap", { pool_id, kind, asset_in, asset_out, amount, user_data });
      console.log(result);
    }
  
    async function performSwapData() {
      const result = await invoke("SwapData", { min_amounts_for_swaps, swap_deadlines });
      console.log(result);
    }
  
    async function performJoinPool() {
      const result = await invoke("JoinPool", { target_bpt, swaps_before_join, swap_data, minimum_bpt });
      console.log(result);
    }
  
    async function performExitPoolRequest() {
      const result = await invoke("ExitPoolRequest", { assets, min_amounts_out, exit_user_data, to_internal_balance });
      console.log(result);
    }
  
    async function performExitPool() {
      const result = await invoke("ExitPool", { exit_target_bpt, swaps_after_exit, exit_swap_data, request });
      console.log(result);
    }
  
    async function performStakeBPT() {
      const result = await invoke("StakeBPT", { bpt, liquidity_gauge, amount_in });
      console.log(result);
    }
  
    async function performUnstakeBPT() {
      const result = await invoke("UnstakeBPT", { unstake_bpt, unstake_liquidity_gauge, amount_out });
      console.log(result);
    }
  
    async function performClaimRewards() {
      const result = await invoke("ClaimRewards", { gauge });
      console.log(result);
    }
  </script>
  
  <!-- UI Components for Each Message Type -->
  <!-- Due to the complexity and the large number of fields, only a representative sample is provided for SingleSwap and JoinPool TODO - FINISH SINGLESWAP-->
  
  <h1>Perform SingleSwap</h1>
  <div>
      <label for="pool_id" title="TODO - PLACEHOLDER">Pool ID:</label>
      <input type="text" id="pool_id" bind:value={pool_id} placeholder="Enter Pool ID">
  </div>
  <div>
      <label for="kind" title="TODO - PLACEHOLDER">Swap Kind:</label>
      <select id="kind" bind:value={kind}>
          <option value="">Select Kind</option>
          <!-- Populate options here based on enum values -->
      </select>
  </div>
  <!-- Continue with other fields for SingleSwap -->
  <button on:click={performSingleSwap}>Perform Swap</button>
  
  <h1>Join Pool</h1>
  <div>
      <label for="target_bpt" title="TODO - PLACEHOLDER">Target BPT:</label>
      <input type="text" id="target_bpt" bind:value={target_bpt} placeholder="Enter Target BPT">
  </div>
  <!-- Inputs for swaps_before_join would need a more complex UI component, possibly a list of SingleSwap components -->
  <!-- Continue with other fields for JoinPool TODO - FINISH OFF THIS-->
  <button on:click={performJoinPool}>Join Pool</button>
  
  <!-- ExitPoolRequest UI -->
<h1>Exit Pool Request</h1>
<div>
    <label for="assets" title="TODO - PLACEHOLDER">Assets:</label>
    <input type="text" id="assets" bind:value={assets} placeholder="Asset Addresses, comma-separated">
</div>
<div>
    <label for="min_amounts_out" title="TODO - PLACEHOLDER">Minimum Amounts Out:</label>
    <input type="text" id="min_amounts_out" bind:value={min_amounts_out} placeholder="Minimum Amounts, comma-separated">
</div>
<div>
    <label for="exit_user_data" title="TODO - PLACEHOLDER">User Data:</label>
    <input type="text" id="exit_user_data" bind:value={exit_user_data} placeholder="User Data in bytes">
</div>
<div>
    <label for="to_internal_balance" title="TODO - PLACEHOLDER">To Internal Balance:</label>
    <input type="checkbox" id="to_internal_balance" bind:checked={to_internal_balance}>
</div>
<button on:click={performExitPoolRequest}>Submit Exit Pool Request</button>

<!-- ExitPool UI -->
<h1>Exit Pool</h1>
<div>
    <label for="exit_target_bpt" title="TODO - PLACEHOLDER">Target BPT:</label>
    <input type="text" id="exit_target_bpt" bind:value={exit_target_bpt} placeholder="Enter Target BPT">
</div>
<!-- Inputs for swaps_after_exit would need a dynamic UI component for multiple SingleSwaps -->
<button on:click={performExitPool}>Exit Pool</button>

<!-- StakeBPT UI -->
<h1>Stake BPT</h1>
<div>
    <label for="bpt" title="TODO - PLACEHOLDER">BPT:</label>
    <input type="text" id="bpt" bind:value={bpt} placeholder="BPT Address">
</div>
<div>
    <label for="liquidity_gauge" title="TODO - PLACEHOLDER">Liquidity Gauge:</label>
    <input type="text" id="liquidity_gauge" bind:value={liquidity_gauge} placeholder="Liquidity Gauge Address">
</div>
<div>
    <label for="amount_in" title="TODO - PLACEHOLDER">Amount:</label>
    <input type="text" id="amount_in" bind:value={amount_in} placeholder="Amount to Stake">
</div>
<button on:click={performStakeBPT}>Stake BPT</button>

<!-- UnstakeBPT UI -->
<h1>Unstake BPT</h1>
<div>
    <label for="unstake_bpt" title="TODO - PLACEHOLDER">BPT:</label>
    <input type="text" id="unstake_bpt" bind:value={unstake_bpt} placeholder="BPT Address">
</div>
<div>
    <label for="unstake_liquidity_gauge" title="TODO - PLACEHOLDER">Liquidity Gauge:</label>
    <input type="text" id="unstake_liquidity_gauge" bind:value={unstake_liquidity_gauge} placeholder="Liquidity Gauge Address">
</div>
<div>
    <label for="amount_out title="TODO - PLACEHOLDER"">Amount:</label>
    <input type="text" id="amount_out" bind:value={amount_out} placeholder="Amount to Unstake">
</div>
<button on:click={performUnstakeBPT}>Unstake BPT</button>

<!-- ClaimRewards UI -->
<h1>Claim Rewards</h1>
<div>
    <label for="gauge" title="TODO - PLACEHOLDER">Gauge:</label>
    <input type="text" id="gauge" bind:value={gauge} placeholder="Gauge Address">
</div>
<button on:click={performClaimRewards}>Claim Rewards</button>
  