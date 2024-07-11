<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let asset = "";
  let use_as_collateral = false;
  const AaveV2EnableAssetAsCollateralV1Address = "";

  async function scheduleSetUserUseReserveAsCollateral() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(AaveV2EnableAssetAsCollateralV1Address, "AaveV2EnableAssetAsCollateralV1", {
          SetUserUseReserveAsCollateral: {
            asset,
            use_as_collateral
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. AaveV2EnableAssetAsCollateralV1</h1>
<div>
  <label for="asset" title="The address of the asset to set as collateral">The address of the asset to set as collateral:</label>
  <input type="text" id="asset" bind:value={asset} placeholder="Address" />
</div>

<div>
  <label for="use_as_collateral" title="Whether to use the asset as collateral.">Use the asset as collateral:</label>
  <input type="checkbox" id="use_as_collateral" bind:value={use_as_collateral} />
</div>


<button on:click={scheduleSetUserUseReserveAsCollateral}>Swap assets</button>
