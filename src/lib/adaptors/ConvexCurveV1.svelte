<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue"

  // Variables for DepositLPTInConvexAndStake
  let pid = "";
  let base_reward_pool_deposit = "";
  let lpt = "";
  let pool = "";
  let selector = "";
  let amount_to_deposit = "";

  // Variables for WithdrawFromBaseRewardPoolAsLPT
  let base_reward_pool_withdraw = "";
  let amount_to_withdraw = "";
  let claim = false;

  // Variables for GetRewards
  let base_reward_pool_rewards = "";
  let claim_extras = false;

  const ConvexCurveV1Address = "";

  function scheduleDeposit() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(ConvexCurveV1Address, "ConvexCurveV1", {
          depositLPTInConvexAndStake: { pid, base_reward_pool: base_reward_pool_deposit, lpt, pool, selector, amount_to_deposit },
        }),
      );
      return callQueue;
    });
  }
  
  function scheduleWithdraw() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(ConvexCurveV1Address, "ConvexCurveV1", {
          withdrawFromBaseRewardPoolAsLPT: { base_reward_pool: base_reward_pool_withdraw, amount_to_withdraw, claim },
        }),
      );
      return callQueue;
    });
  }
  
  function scheduleGetRewards() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(ConvexCurveV1Address, "ConvexCurveV1", {
          getRewards: { base_reward_pool: base_reward_pool_rewards, claim_extras },
        }),
      );
      return callQueue;
    });
  }
  
</script>

<!-- DepositLPTInConvexAndStake UI -->
<h1>1. Deposit and Stake LPT in Convex</h1>

<!-- depositLPTInConvexAndStake(uint256 _pid, address baseRewardPool, ERC20 _lpt, CurvePool _pool, bytes4 _selector, uint256 _amount) -->
<div>
  <label for="pid" title="Enter the Pool ID (PID) as a string.">PID:</label>
  <input type="text" id="pid" bind:value={pid} placeholder="Enter PID">
</div>
<div>
  <label for="base_reward_pool_deposit" title="Enter the Base Reward Pool address as a string.">Base Reward Pool:</label>
  <input type="text" id="base_reward_pool_deposit" bind:value={base_reward_pool_deposit} placeholder="Enter Base Reward Pool Address">
</div>
<div>
  <label for="lpt" title="lpt">lpt:</label>
  <input type="text" id="lpt" bind:value={lpt} placeholder="lpt">
</div>
<div>
  <label for="pool" title="pool">pool:</label>
  <input type="text" id="pool" bind:value={pool} placeholder="pool">
</div>
<div>
  <label for="selector" title="selector">selector:</label>
  <input type="text" id="selector" bind:value={selector} placeholder="selector">
</div>
<div>
  <label for="amount_to_deposit" title="Amount to deposit.">Amount to deposit:</label>
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount">
</div>
<button on:click={scheduleDeposit}>Deposit and Stake</button>

<!-- WithdrawFromBaseRewardPoolAsLPT UI -->
<h1>2. Withdraw from Convex</h1>

<div>
  <label for="base_reward_pool_withdraw" title="Enter the Base Reward Pool address as a string.">Base Reward Pool:</label>
  <input type="text" id="base_reward_pool_withdraw" bind:value={base_reward_pool_withdraw} placeholder="Enter Base Reward Pool Address">
</div>
<div>
  <label for="amount_to_withdraw" title="Enter the amount to withdraw from ConvexCurve market, as a string.">Amount to Withdraw:</label>
  <input type="text" id="amount_to_withdraw" bind:value={amount_to_withdraw} placeholder="Enter amount to withdraw">
</div>
<div>
  <label for="claim" title="Enter whether or not to rewards associated to respective ConvexCurve market position, as a bool.">Claim Rewards:</label>
  <input type="checkbox" id="claim" bind:checked={claim}>
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- GetRewards UI -->
<h1>3. Get Rewards from Convex</h1>
<div>
  <label for="base_reward_pool_rewards" title="Enter the Base Reward Pool address as a string.">Base Reward Pool:</label>
  <input type="text" id="base_reward_pool_rewards" bind:value={base_reward_pool_rewards} placeholder="Enter Base Reward Pool Address">
</div>
<div>
  <label for="claim_extras" title="Enter whether or not to claim extra rewards in addition to baseRewards with respective ConvexCurve market, as a bool.">Claim Extras:</label>
  <input type="checkbox" id="claim_extras" bind:checked={claim_extras}>
</div>
<button on:click={scheduleGetRewards}>Get Rewards</button>
