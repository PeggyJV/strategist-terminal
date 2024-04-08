<script>
    import { invoke } from "@tauri-apps/api/tauri";
  
    let auraPool = "";
    let claimExtras = false;
  
    async function getRewards() {
      try {
        const result = await invoke("getRewards", {
          aura_pool: auraPool,
          claim_extras: claimExtras
        });
        console.log("GetRewards result:", result);
      } catch (error) {
        console.error("Error invoking getRewards:", error);
      }
    }
  </script>
  
  <form on:submit|preventDefault={getRewards}>
    <h2>Get Rewards from Aura Pool</h2>
    <div>
      <label for="auraPool" title="TODO - PLACEHOLDER">Aura Pool Address:</label>
      <input type="text" id="auraPool" bind:value={auraPool} placeholder="Enter Aura Pool Address">
    </div>
    <div>
      <label for="claimExtras" title="TODO - PLACEHOLDER">
        <input type="checkbox" id="claimExtras" bind:checked={claimExtras}>
        Claim Extra Rewards
      </label>
    </div>
    <button type="submit">Get Rewards</button>
  </form>
  