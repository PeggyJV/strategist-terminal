<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue.js"
  
  // Variables for Mint
  let amount_1 = "";
  let min_amount_out_1 = "";
  let wildcard_1 = "";

  // Variables for RequestBurn
  let amount_2 = "";
  let wildcard_2 = "";

  // Variables for CompleteBurn
  let id_3 = "";
  let min_amount_out_3 = "";
  let wildcard_3 = "";
  
  // Variables for CancelBurn
  let id_4 = "";
  let wildcard_4 = "";

  // Variables for Wrap
  let amount_5 = "";
  let min_amount_out_5 = "";
  let wildcard_5 = "";

  // Variables for Unwrap
  let amount_6 = "";
  let min_amount_out_6 = "";
  let wildcard_6 = "";

  // Variables for MintERC20
  let deposit_asset = "";
  let amount_7 = "";
  let min_amount_out_7 = "";
  let wildcard_7 = "";

  // Variables for MintERC20
  let id_8 = "";
  let wildcard_8 = "";

  let StakingV1Address = "";

  function scheduleMint() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          Mint: { amount: amount_1, min_amount_out: min_amount_out_1, wildcard: wildcard_1 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleRequestBurn() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          RequestBurn: { amount: amount_2, wildcard: wildcard_2 },
        }),
      );
      return callQueue;
    });
  }
  
  function scheduleCompleteBurn() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          CompleteBurn: { id: id_3, min_amount_out: min_amount_out_3, wildcard: wildcard_3 },
        }),
      );
      return callQueue;
    });
  }
  
  function scheduleCancelBurn() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          CancelBurn: { id: id_4, wildcard: wildcard_4 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWrap() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          Wrap: { amount: amount_5, min_amount_out: min_amount_out_5, wildcard: wildcard_5 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleUnwrap() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          Unwrap: { amount: amount_6, min_amount_out: min_amount_out_6, wildcard: wildcard_6 },
        }),
      );
      return callQueue;
    });
  }

  function scheduleMintERC20() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          MintErc20: { deposit_asset, amount: amount_7, min_amount_out: min_amount_out_7, wildcard: wildcard_7 },
        }),
      );
      return callQueue;
    });
  }


  function scheduleRemoveClaimed() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(StakingV1Address, "StakingV1", {
          RemoveClaimedRequest: { id: id_8, wildcard: wildcard_8 },
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- Mint UI Component -->
<h2>1. Mint</h2>

<div>
  <label for="amount_1" title="Enter the amount of native asset to use for minting, as a string.">Amount:</label>
  <input type="text" id="amount_1" bind:value={amount_1} placeholder="Amount">
</div>
<div>
  <label for="min_amount_out_1" title="Enter the minimum amount of the asset to receive, as a string.">Min Amount Out:</label>
  <input type="text" id="min_amount_out_1" bind:value={min_amount_out_1} placeholder="Min Amount Out">
</div>
<div>
  <label for="wildcard_1" title="Enter the Arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_1" bind:value={wildcard_1} placeholder="Wildcard Data">
</div>
<button on:click={scheduleMint}>Mint</button>

<!-- RequestBurn UI Component -->
<h2>2. Request Burn</h2>
<div>
  <label for="amount_2" title="Enter the amount of derivative to burn/withdraw as a string.">Amount:</label>
  <input type="text" id="amount_2" bind:value={amount_2} placeholder="Amount">
</div>
<div>
  <label for="wildcard_2" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors, as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_2" bind:value={wildcard_2} placeholder="Wildcard Data">
</div>
<button on:click={scheduleRequestBurn}>Request Burn</button>

<!-- CompleteBurn UI Component -->
<h2>3. Complete Burn</h2>

<div>
  <label for="id_3" title="Enter the request ID as a string.">Burn Request ID:</label>
  <input type="text" id="id_3" bind:value={id_3} placeholder="Burn Request ID">
</div>
<div>
  <label for="min_amount_out_3" title="Enter the  minimum amount of the asset to receive as a string.">Min Amount Out:</label>
  <input type="text" id="min_amount_out_3" bind:value={min_amount_out_3} placeholder="Min Amount Out">
</div>
<div>
  <label for="wildcard_3" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_3" bind:value={wildcard_3} placeholder="Wildcard Data">
</div>
<button on:click={scheduleCompleteBurn}>Complete Burn</button>

<!-- CancelBurn UI Component -->
<h2>4. Cancel Burn</h2>
<div>
  <label for="id_4" title="Enter the id of the burn request as a string.">Burn Request ID:</label>
  <input type="text" id="id_4" bind:value={id_4} placeholder="Burn Request ID">
</div>
<div>
  <label for="wildcard_4" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_4" bind:value={wildcard_4} placeholder="Wildcard Data">
</div>
<button on:click={scheduleCancelBurn}>Cancel Burn</button>

<!-- Wrap UI Component -->
<h2>5. Wrap Derivative Asset</h2>
<div>
  <label for="amount_5" title="Enter the amount of derivative to wrap, as a string.">Amount to Wrap:</label>
  <input type="text" id="amount_5" bind:value={amount_5} placeholder="Enter amount">
</div>
<div>
  <label for="min_amount_out_5" title="Enter the minimum amount of the asset to receive as a string.">Min Amount Out:</label>
  <input type="text" id="min_amount_out_5" bind:value={min_amount_out_5} placeholder="Enter minimum amount out">
</div>
<div>
  <label for="wildcard_5" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_5" bind:value={wildcard_5} placeholder="Enter wildcard data">
</div>
<button on:click={scheduleWrap}>Wrap</button>

<!-- Unwrap UI Component -->
<h2>6. Unwrap Derivative Asset</h2>
<div>
  <label for="amount_6" title="Enter the amount of wrapped derivative to unwrap as a string.">Amount to Unwrap:</label>
  <input type="text" id="amount_6" bind:value={amount_6} placeholder="Enter amount">
</div>
<div>
  <label for="min_amount_out_6" title="Enter the minimum amount of the asset to receive as a string.">Min Amount Out:</label>
  <input type="text" id="min_amount_out_6" bind:value={min_amount_out_6} placeholder="Enter minimum amount out">
</div>
<div>
  <label for="wildcard_6" title="Enter the arbitrary ABI encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_6" bind:value={wildcard_6} placeholder="Enter wildcard data">
</div>
<button on:click={scheduleUnwrap}>Unwrap</button>

<!-- MintERC20 UI Component -->
<h2>7. Mint Derivative Asset with ERC20</h2>
<div>
  <label for="deposit_asset" title="Enter the ERC20 asset to mint with as a string.">Deposit Asset Address:</label>
  <input type="text" id="deposit_asset" bind:value={deposit_asset} placeholder="Enter deposit asset address">
</div>
<div>
  <label for="amount_7" title="Enter the the amount of `depositAsset` to mint with as a string.">Amount:</label>
  <input type="text" id="amount_7" bind:value={amount_7} placeholder="Enter amount">
</div>
<div>
  <label for="min_amount_out_7" title="Enter the the minimum amount of derivative out as a string.">Min Amount Out:</label>
  <input type="text" id="min_amount_out_7" bind:value={min_amount_out_7} placeholder="Enter minimum amount out">
</div>
<div>
  <label for="wildcard_7" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_7" bind:value={wildcard_7} placeholder="Enter wildcard data">
</div>
<button on:click={scheduleMintERC20}>Mint with ERC20</button>

<!-- RemoveClaimedRequest UI Component -->
<h2>8. Remove Claimed Request</h2>
<div>
  <label for="id_8" title="Enter the request ID to remove, as a string.">Request ID:</label>
  <input type="text" id="id_8" bind:value={id_8} placeholder="Enter request ID">
</div>
<div>
  <label for="wildcard_8" title="Enter the arbitrary abi encoded data that can be used by inheriting adaptors, as a string.">Wildcard Data:</label>
  <input type="text" id="wildcard_8" bind:value={wildcard_8} placeholder="Enter wildcard data">
</div>
<button on:click={scheduleRemoveClaimed}>Remove Claimed Request</button>
