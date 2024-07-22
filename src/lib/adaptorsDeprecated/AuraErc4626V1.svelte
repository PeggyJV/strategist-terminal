<script lang="ts">
  import { CellarCall, queue } from "$stores/AdapterQueue"

  let aura_pool = "";
  let claim_extras = false;

  let AuraErc4626V1Address = "";

  function scheduleGetRewards() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AuraErc4626V1Address, "AuraErc4626V1", {
          GetRewards: {
            aura_pool,
            claim_extras
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h2>1. Get Rewards from Aura Pool</h2>
<div>
  <label for="auraPool" title="Aura Pool Address">Aura Pool Address:</label>
  <input type="text" id="auraPool" bind:value={aura_pool} placeholder="Enter Aura Pool Address">
</div>
<div>
  <label for="claimExtras" title="Claim Extra Rewards">Claim Extra Rewards</label>
  <input type="checkbox" id="claimExtras" bind:checked={claim_extras}>
</div>
<button on:click={scheduleGetRewards}>Get Rewards</button>
