<script lang="ts">
import { Functions } from "$lib/type"
import { CellarCall } from "$stores/AdapterQueue"
import ManagementCall from "./ManagementCall.svelte"

let registryId = "";
let sharePriceOracle = "";

let callModalVisible = false;
function toggleCallModal() {
  callModalVisible = !callModalVisible;
}

let call: CellarCall;

async function callSetSharePriceOracle() {
  const callData = {
    registry_id: registryId,
    share_price_oracle: sharePriceOracle
  }

  call = new CellarCall(Functions.SetSharePriceOracle, callData)

  toggleCallModal();
}

</script>
<div class="prose w-screen flex flex-col justify-center items-center">

  {#if callModalVisible}
    <ManagementCall {toggleCallModal} {call} />
  {/if}

  <h2>{Functions.SetSharePriceOracle}</h2>

  <div class="flex flex-col justify-between mt-2">
    <label for="registryId">registry_id:</label>
    <input
      bind:value={registryId}
      id="registryId"
      placeholder=""
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />

    <label for="sharePriceOracle">share_price_oracle:</label>
    <input
      bind:value={sharePriceOracle}
      id="sharePriceOracle"
      placeholder=""
      class="w-100 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
    />

    <button
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
      on:click={callSetSharePriceOracle}
    >
      Set
    </button>
  </div>


</div>
