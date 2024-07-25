<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let somm_node_grpc = "https://sommelier-grpc.polkachu.com:14190";
  let publisher_domain = "https://smart-strategies.xyz";
  let client_cert_path = "/home/strategy/certs/client.pem";
  let client_cert_key_path = "/home/strategy/certs/client.key";

  let showTooltip = false; // State to manage tooltip visibility

  async function configure() {
    const result = await invoke("configure", {
      somm_node_grpc,
      publisher_domain,
      client_cert_path,
      client_cert_key_path,
    });
    console.log(result);
  }

  $: isButtonEnabled = somm_node_grpc.trim().length > 0 &&
                       publisher_domain.trim().length > 0 &&
                       client_cert_path.trim().length > 0 &&
                       client_cert_key_path.trim().length > 0;
</script>

<div class="prose">
  <h1 class="text-2xl font-bold mb-4">Configure</h1>

  <div class="mb-4">
    <label for="somm_node_grpc" class="block mb-1">Sommelier Node GRPC URL:</label>
    <input
      type="text"
      id="somm_node_grpc"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={somm_node_grpc}
      placeholder="Enter Sommelier Node GRPC URL"
    />
  </div>

  <div class="mb-4">
    <label for="publisher_domain" class="block mb-1">Publisher Domain:</label>
    <input
      type="text"
      id="publisher_domain"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={publisher_domain}
      placeholder="Enter Publisher Domain"
    />
  </div>

  <div class="mb-4">
    <label for="client_cert_path" class="block mb-1">Client Cert Path:</label>
    <input
      type="text"
      id="client_cert_path"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={client_cert_path}
      placeholder="Enter Client Cert Path"
    />
  </div>

  <div class="mb-4">
    <label for="client_cert_key_path" class="block mb-1">Client Cert Key Path:</label>
    <input
      type="text"
      id="client_cert_key_path"
      class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={client_cert_key_path}
      placeholder="Enter Client Cert Key Path"
    />
  </div>

  <div class="group relative" on:mouseover={() => showTooltip = true} on:mouseout={() => showTooltip = false}>
    <button class="px-4 py-2 rounded-md focus:outline-none {isButtonEnabled ? 'bg-blue-500 text-white hover:bg-blue-600 focus:bg-blue-600' : 'bg-gray-400 text-gray-700 cursor-not-allowed'}" on:click={configure} disabled={!isButtonEnabled}>
      Configure
    </button>
    {#if !isButtonEnabled && showTooltip}
      <div class="absolute inset-x-0 bottom-full mb-2 px-2 py-1 bg-black text-white text-center rounded-md translate-x-[-50%]">
        Please fill all fields
      </div>
    {/if}
  </div>
</div>
