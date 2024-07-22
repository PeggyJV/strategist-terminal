<script lang="ts">

  import { CellarCall, queue } from "$stores/AdapterQueue"

  // Variables for SingleSwap
  let pool_id = "";
  let kind = ""; // This would be an enum in the actual implementation
  let asset_in = "";
  let asset_out = "";
  let amount = "";
  let user_data = ""; // This needs to be handled as bytes

  // Variables for SwapData
  let min_amounts_for_swaps = "";
  let swap_deadlines = "";

  // Variables for JoinPool
  let target_bpt = "";
  let swaps_before_join = ""; // This would require a complex input method for multiple SingleSwaps
  let swap_data = ""; // This would be the input for SwapData
  let minimum_bpt = "";

  // Variables for ExitPoolRequest
  let assets: string[] = [];
  let min_amounts_out: string[] = [];
  let exit_user_data = ""; // As bytes
  let to_internal_balance = false;

  // Variables for ExitPool
  let exit_target_bpt = "";
  let swaps_after_exit = ""; // Complex input for multiple SingleSwaps
  let exit_swap_data = ""; // Input for SwapData
  let request = ""; // Input for ExitPoolRequest

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

  const BalancerPoolV1Address = "";

  function scheduleSingleSwap() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          SingleSwap: { pool_id, kind, asset_in, asset_out, amount, user_data },
        }),
      );
      return callQueue;
    });
  }

  function scheduleSwapData() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          SwapData: { min_amounts_for_swaps, swap_deadlines },
        }),
      );
      return callQueue;
    });
  }

  function scheduleJoinPool() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          JoinPool: { target_bpt, swaps_before_join, swap_data, minimum_bpt },
        }),
      );
      return callQueue;
    });
  }

  function scheduleExitPoolRequest() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          ExitPoolRequest: { assets, min_amounts_out, exit_user_data, to_internal_balance },
        }),
      );
      return callQueue;
    });
  }

  function scheduleExitPool() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          ExitPool: { exit_target_bpt, swaps_after_exit, exit_swap_data, request },
        }),
      );
      return callQueue;
    });
  }

  function scheduleStakeBPT() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          StakeBPT: { bpt, liquidity_gauge, amount_in },
        }),
      );
      return callQueue;
    });
  }

  function scheduleUnstakeBPT() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          UnstakeBPT: { unstake_bpt, unstake_liquidity_gauge, amount_out },
        }),
      );
      return callQueue;
    });
  }

  function scheduleClaimRewards() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(BalancerPoolV1Address, "BalancerPoolV1", {
          ClaimRewards: { gauge },
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- UI Components for Each Message Type -->
<!-- Due to the complexity and the large number of fields, only a representative sample is provided for SingleSwap and JoinPool TODO - FINISH SINGLESWAP-->

<h1>1. Perform SingleSwap</h1>
<div>
  <label for="pool_id" title="TODO - PLACEHOLDER">Pool ID:</label>
  <input type="text" id="pool_id" bind:value={pool_id} placeholder="Enter Pool ID">
</div>
<div>
  <label for="kind" title="TODO - PLACEHOLDER">Swap Kind:</label>
  <input type="text" id="pool_id" bind:value={kind} placeholder="Kind">
</div>
<div>
  <label for="asset_in" title="TODO - PLACEHOLDER">The asset in:</label>
  <input type="text" id="asset_in" bind:value={asset_in} placeholder="Address">
</div>
<div>
  <label for="asset_out" title="TODO - PLACEHOLDER">The asset out:</label>
  <input type="text" id="asset_out" bind:value={asset_out} placeholder="Address">
</div>
<div>
  <label for="amount" title="TODO - PLACEHOLDER">The amount:</label>
  <input type="text" id="amount" bind:value={amount} placeholder="Amount">
</div>
<div>
  <label for="user_data" title="TODO - PLACEHOLDER">The user data:</label>
  <input type="text" id="user_data" bind:value={user_data} placeholder="User data">
</div>

<!-- Continue with other fields for SingleSwap -->
<button on:click={scheduleSingleSwap}>Perform Swap</button>

<h1>2. Swap Data</h1>
<div>
  <label for="min_amounts_for_swaps" title="TODO - PLACEHOLDER">The minimum amounts for swaps:</label>
  <input type="text" id="min_amounts_for_swaps" bind:value={min_amounts_for_swaps} placeholder="Amounts">
</div>
<div>
  <label for="swap_deadlines" title="TODO - PLACEHOLDER">The swap deadlines:</label>
  <input type="text" id="swap_deadlines" bind:value={swap_deadlines} placeholder="Deadlines">
</div>

<button on:click={scheduleSwapData}>Perform Swap</button>

<h1>3. Join Pool</h1>
<div>
  <label for="target_bpt" title="TODO - PLACEHOLDER">Target BPT:</label>
  <input type="text" id="target_bpt" bind:value={target_bpt} placeholder="BPT">
</div>
<div>
  <label for="swaps_before_join" title="TODO - PLACEHOLDER">Swap to execute before joining pool:</label>
  <input type="text" id="swaps_before_join" bind:value={swaps_before_join} placeholder="Swap">
</div>
<div>
  <label for="swap_data" title="TODO - PLACEHOLDER">Data for swaps:</label>
  <input type="text" id="swap_data" bind:value={swap_data} placeholder="Data">
</div>
<div>
  <label for="minimum_bpt" title="TODO - PLACEHOLDER">The minimum BPT to mint:</label>
  <input type="text" id="minimum_bpt" bind:value={minimum_bpt} placeholder="BPT">
</div>

<!-- Inputs for swaps_before_join would need a more complex UI component, possibly a list of SingleSwap components -->
<!-- Continue with other fields for JoinPool TODO - FINISH OFF THIS-->
<button on:click={scheduleJoinPool}>Join Pool</button>

<!-- ExitPoolRequest UI -->
<h1>4. Exit Pool Request</h1>
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
<button on:click={scheduleExitPoolRequest}>Submit Exit Pool Request</button>

<!-- ExitPool UI -->
<h1>5. Exit Pool</h1>
  <div>
    <label for="exit_target_bpt" title="TODO - PLACEHOLDER">Swaps to execute after exiting pool:</label>
    <input type="text" id="exit_target_bpt" bind:value={exit_target_bpt} placeholder="Swaps">
  </div>
  <div>
    <label for="swaps_after_exit" title="TODO - PLACEHOLDER">Target BPT:</label>
    <input type="text" id="swaps_after_exit" bind:value={swaps_after_exit} placeholder="Enter Target BPT">
  </div>
  <div>
    <label for="swap_data" title="TODO - PLACEHOLDER">Data for swaps:</label>
    <input type="text" id="swap_data" bind:value={swap_data} placeholder="Swap Data">
  </div>
  <div>
    <label for="request" title="TODO - PLACEHOLDER">Request:</label>
    <input type="text" id="request" bind:value={request} placeholder="request">
  </div>
<!-- Inputs for swaps_after_exit would need a dynamic UI component for multiple SingleSwaps -->
<button on:click={scheduleExitPool}>Exit Pool</button>

<!-- StakeBPT UI -->
<h1>6. Stake BPT</h1>
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
<button on:click={scheduleStakeBPT}>Stake BPT</button>

<!-- UnstakeBPT UI -->
<h1>7. Unstake BPT</h1>
<div>
  <label for="unstake_bpt" title="TODO - PLACEHOLDER">BPT:</label>
  <input type="text" id="unstake_bpt" bind:value={unstake_bpt} placeholder="BPT Address">
</div>
<div>
  <label for="unstake_liquidity_gauge" title="TODO - PLACEHOLDER">Liquidity Gauge:</label>
  <input type="text" id="unstake_liquidity_gauge" bind:value={unstake_liquidity_gauge} placeholder="Liquidity Gauge Address">
</div>
<div>
  <label for="amount_out" title="TODO - PLACEHOLDER">Amount:</label>
  <input type="text" id="amount_out" bind:value={amount_out} placeholder="Amount to Unstake">
</div>
<button on:click={scheduleUnstakeBPT}>Unstake BPT</button>

<!-- ClaimRewards UI -->
<h1>8. Claim Rewards</h1>
<div>
  <label for="gauge" title="TODO - PLACEHOLDER">Gauge:</label>
  <input type="text" id="gauge" bind:value={gauge} placeholder="Gauge Address">
</div>
<button on:click={scheduleClaimRewards}>Claim Rewards</button>
