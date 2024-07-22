<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  let somm_node_grpc = "https://sommelier-grpc.polkachu.com:14190";
  let publisher_domain = "https://smart-strategies.xyz";
  let client_cert_path = "/home/strategy/certs/client.pem";
  let client_cert_key_path = "/home/strategy/certs/client.key";

  async function configure() {
    const result = await invoke("configure", {
      somm_node_grpc,
      publisher_domain,
      client_cert_path,
      client_cert_key_path,
    });
    console.log(result);
  }
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

  <button
    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
    on:click={configure}
  >
    Configure
  </button>
</div>