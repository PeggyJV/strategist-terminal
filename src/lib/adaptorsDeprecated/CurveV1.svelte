<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue.js"

  // Variables for AddLiquidity
  let pool_1 = "";
  let lp_token_1 = "";
  let ordered_underlying_token_amounts_1 = "";
  let min_lp_amount_1 = "";
  let gauge_1 = "";
  let selector_1 = "";

  // Variables for AddLiquidityETH
  let pool_2 = "";
  let lp_token_2 = "";
  let ordered_underlying_token_amounts_2 = "";
  let min_lp_amount_2 = "";
  let use_underlying_2 = false;
  let gauge_2 = "";
  let selector_2 = "";

  // Variables for RemoveLiquidity
  let pool_3 = "";
  let lp_token_3 = "";
  let lp_token_amount_3 = "";
  let ordered_minimum_underlying_token_amounts_out_3 = "";
  let gauge_3 = "";
  let selector_3 = "";

  // Variables for RemoveLiquidityETH
  let pool_4 = "";
  let lp_token_4 = "";
  let lp_token_amount_4 = "";
  let ordered_minimum_underlying_token_amounts_out_4 = "";
  let use_underlying_4 = false;
  let gauge_4 = "";
  let selector_4 = "";

  // Variables for StakeInGauge
  let lp_token_5 = "";
  let gauge_5 = "";
  let amount_5 = "";
  let pool_5 = "";
  let selector_5 = "";

  // Variables for UnstakeInGauge
  let gauge_6 = "";
  let amount_6 = "";

  // Variables for ClaimRewards
  let gauge_7 = "";

  const CurveV1Address = "";

  function scheduleAddLiquidity() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          AddLiquidity: { pool: pool_1, lp_token: lp_token_1, ordered_underlying_token_amounts: JSON.parse(ordered_underlying_token_amounts_1), min_lp_amount: min_lp_amount_1, gauge: gauge_1, selector: selector_1 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleAddLiquidityETH() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          AddLiquidityETH: { pool: pool_2, lp_token: lp_token_2, ordered_underlying_token_amounts: JSON.parse(ordered_underlying_token_amounts_2), min_lp_amount: min_lp_amount_2, use_underlying: use_underlying_2, gauge: gauge_2, selector: selector_2 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRemoveLiquidity() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          RemoveLiquidity: { pool: pool_3, lp_token: lp_token_3, lp_token_amount: lp_token_amount_3, ordered_minimum_underlying_token_amounts_out: JSON.parse(ordered_minimum_underlying_token_amounts_out_3), gauge: gauge_3, selector: selector_3 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRemoveLiquidityETH() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          RemoveLiquidityETH: { pool: pool_4, lp_token: lp_token_4, lp_token_amount: lp_token_amount_4, ordered_minimum_underlying_token_amounts_out: JSON.parse(ordered_minimum_underlying_token_amounts_out_4), use_underlying: use_underlying_4, gauge: gauge_4, selector: selector_4 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleStakeInGauge() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          StakeInGauge: { lp_token: lp_token_5, gauge: gauge_5, amount: amount_5, pool: pool_5, selector: selector_5 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleUnstakeInGauge() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          UnstakeFromGauge: { gauge: gauge_6, amount: amount_6 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleClaimRewards() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(CurveV1Address, "CurveV1", {
          ClaimRewards: { gauge: gauge_7 },
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- AddLiquidity UI Component-->
<h2>1. Add Liquidity (Non-ETH)</h2>

<div>
  <label for="pool_1" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
  <input type="text" id="pool_1" bind:value={pool_1} placeholder="Curve Pool Address">
</div>
<div>
  <label for="lp_token_1" title="Enter the curve pool token address as a string.">LP Token Address:</label>
  <input type="text" id="lp_token_1" bind:value={lp_token_1} placeholder="LP Token Address">
</div>
<div>
  <label for="ordered_underlying_token_amounts_1" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Underlying Token Amounts:</label>
  <input type="text" id="ordered_underlying_token_amounts_1" bind:value={ordered_underlying_token_amounts_1} placeholder="e.g., [100, 200]">
</div>
<div>
  <label for="min_lp_amount_1" title="Enter the minimum amount of LP tokens to receive, as a string.">Min LP Amount:</label>
  <input type="text" id="min_lp_amount_1" bind:value={min_lp_amount_1} placeholder="Min LP Amount">
</div>
<div>
  <label for="gauge_1" title="Enter the curve gauge address as a string.">Curve Gauge Address:</label>
  <input type="text" id="gauge_1" bind:value={gauge_1} placeholder="Gauge Address">
</div>
<div>
  <label for="selector_1" title="Enter the selector as a string.">Selector:</label>
  <input type="text" id="selector_1" bind:value={selector_1} placeholder="Selector">
</div>
<button on:click={scheduleAddLiquidity}>Add Liquidity</button>

<!-- AddLiquidityETH UI Component -->
<h2>2. Add Liquidity with ETH</h2>

<div>
  <label for="pool_2" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
  <input type="text" id="pool_2" bind:value={pool_2} placeholder="Curve Pool Address">
</div>
<div>
  <label for="lp_token_2" title="Enter the Curve LP Token  address as a string.">LP Token Address:</label>
  <input type="text" id="lp_token_2" bind:value={lp_token_2} placeholder="LP Token Address">
</div>
<div>
  <label for="ordered_underlying_token_amounts_2" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Underlying Token Amounts:</label>
  <input type="text" id="ordered_underlying_token_amounts_2" bind:value={ordered_underlying_token_amounts_2} placeholder="e.g., [100, 200]">
</div>
<div>
  <label for="min_lp_amount_2" title="Enter the minimum amount of LP tokens to receive, as a string.">Min LP Amount:</label>
  <input type="text" id="min_lp_amount_2" bind:value={min_lp_amount_2} placeholder="Min LP Amount">
</div>
<div>
  <label for="use_underlying_2" title="Enter a bool for whether to use the underlying asset or the wrapped asset.">Use Underlying:</label>
  <input type="checkbox" id="use_underlying_2" bind:checked={use_underlying_2}>
</div>
<div>
  <label for="gauge_2" title="Enter the respective curve gauge address as a string.">Curve Gauge Address:</label>
  <input type="text" id="gauge_2" bind:value={gauge_2} placeholder="Gauge Address">
</div>
<div>
  <label for="selector_2" title="Enter the selector as a string.">Selector:</label>
  <input type="text" id="selector_2" bind:value={selector_2} placeholder="Selector">
</div>
<button on:click={scheduleAddLiquidityETH}>Add Liquidity with ETH</button>

<!-- RemoveLiquidity UI Component -->
<h2>3. Remove Liquidity (Non-ETH)</h2>
<div>
  <label for="pool_3" title="Enter the Curve Pool address as a string.">Curve Pool Address:</label>
  <input type="text" id="pool_3" bind:value={pool_3} placeholder="Enter Curve Pool Address">
</div>
<div>
  <label for="lp_token_3" title="Enter the curve pool token address as a string.">LP Token Address:</label>
  <input type="text" id="lp_token_3" bind:value={lp_token_3} placeholder="Enter LP Token Address">
</div>
<div>
  <label for="lp_token_amount_3" title="Enter the Base Reward Pool address as a string.">LP Token Amount:</label>
  <input type="text" id="lp_token_amount_3" bind:value={lp_token_amount_3} placeholder="Enter LP Token Amount to Remove">
</div>
<div>
  <label for="ordered_minimum_underlying_token_amounts_out_3" title="Enter the minimum amount of each underlying token to receive, as an array of strings.">Ordered Minimum Token Amounts Out:</label>
  <input type="text" id="ordered_minimum_underlying_token_amounts_out_3" bind:value={ordered_minimum_underlying_token_amounts_out_3} placeholder="e.g., [50, 100]">
</div>
<div>
  <label for="gauge_3" title="Enter the curve gauge address as a string.">Curve Gauge Address:</label>
  <input type="text" id="gauge_3" bind:value={gauge_3} placeholder="Enter Gauge Address">
</div>
<div>
  <label for="selector_3" title="Enter the selector as a string.">Function Selector:</label>
  <input type="text" id="selector_3" bind:value={selector_3} placeholder="Enter Selector">
</div>
<button on:click={scheduleRemoveLiquidity}>Remove Liquidity</button>

<!-- RemoveLiquidityETH UI Component -->
<h2>4. Remove Liquidity with ETH</h2>
<div>
  <label for="use_underlying_4" title="Enter bool indicating whether or not to add a true bool to the end of abi.encoded `removeLiquidity` call.">Use Underlying:</label>
  <input type="checkbox" id="use_underlying_4" bind:checked={use_underlying_4}>
</div>
<div>
  <label for="pool_4">Curve Pool Address:</label>
  <input type="text" id="pool_4" bind:value={pool_4} placeholder="Enter Curve Pool Address">
</div>
<div>
  <label for="lp_token_4">LP Token Address:</label>
  <input type="text" id="lp_token_4" bind:value={lp_token_4} placeholder="Enter LP Token Address">
</div>
<div>
  <label for="lp_token_amount_4">LP Token Amount:</label>
  <input type="text" id="lp_token_amount_4" bind:value={lp_token_amount_4} placeholder="Enter LP Token Amount to Remove">
</div>
<div>
  <label for="ordered_minimum_underlying_token_amounts_out_4">Minimum Underlying Token Amounts Out:</label>
  <input type="text" id="ordered_minimum_underlying_token_amounts_out_4" bind:value={ordered_minimum_underlying_token_amounts_out_4} placeholder="Enter JSON array of min amounts out">
</div>
<div>
  <label for="gauge_4">Curve Gauge Address:</label>
  <input type="text" id="gauge_4" bind:value={gauge_4} placeholder="Enter Gauge Address">
</div>
<div>
  <label for="selector_4">Function Selector:</label>
  <input type="text" id="selector_4" bind:value={selector_4} placeholder="Enter Function Selector">
</div>
<button on:click={scheduleRemoveLiquidityETH}>Remove Liquidity with ETH</button>

<!-- StakeInGauge UI Component -->
<h2>5. Stake In Gauge</h2>
<div>
  <label for="lp_token_5" title="Enter the address of the LP token as a string.">LP Token Address:</label>
  <input type="text" id="lp_token_5" bind:value={lp_token_5} placeholder="Enter LP Token Address">
</div>
<div>
  <label for="gauge_5" title="Enter the Curve Pool Gauge address as a string.">Gauge Address:</label>
  <input type="text" id="gauge_5" bind:value={gauge_5} placeholder="Enter Gauge Address">
</div>
<div>
  <label for="amount_5" title="Enter the amount of LP tokens to stake, as a string.">Amount:</label>
  <input type="text" id="amount_5" bind:value={amount_5} placeholder="Enter Amount to Stake">
</div>
<div>
  <label for="pool_5" title="Enter the address of the Curve Pool as a string.">Curve Pool Address:</label>
  <input type="text" id="pool_5" bind:value={pool_5} placeholder="Enter Curve Pool Address">
</div>
<div>
  <label for="selector_5" title="Enter the selector for the function to call, as a string.">Function Selector:</label>
  <input type="text" id="selector_5" bind:value={selector_5} placeholder="Enter Function Selector">
</div>
<button on:click={scheduleStakeInGauge}>Stake in Gauge</button>

<!-- UnstakeFromGauge UI Component -->
<h2>6. Unstake From Gauge</h2>
<div>
  <label for="gauge_6" title="Enter the Curve Gauge address as a string.">Gauge Address:</label>
  <input type="text" id="gauge_6" bind:value={gauge_6} placeholder="Enter Gauge Address">
</div>
<div>
  <label for="amount_6" title="Enter the amount of LP tokens to unstake, as a string.">Amount:</label>
  <input type="text" id="amount_6" bind:value={amount_6} placeholder="Enter Amount to Unstake">
</div>
<button on:click={scheduleUnstakeInGauge}>Unstake from Gauge</button>

<!-- ClaimRewards UI Component -->
<h2>7. Claim Rewards</h2>
<div>
  <label for="gauge_7" title="Enter the address of the Curve Gauge as a string.">Gauge Address:</label>
  <input type="text" id="gauge_7" bind:value={gauge_7} placeholder="Enter Gauge Address">
</div>
<button on:click={scheduleClaimRewards}>Claim Rewards</button>