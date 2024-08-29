<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  let somm_node_rpc = "https://sommelier-rpc.polkachu.com:443";
  let somm_node_grpc = "https://sommelier-grpc.polkachu.com:14190";
  let publisher_domain = "https://smart-strategies.xyz";
  let client_cert_path = "/home/strategy/certs/client.pem";
  let client_cert_key_path = "/home/strategy/certs/client.key";

  async function configure() {
    const result = await invoke("configure", {
      sommNodeRpc: somm_node_rpc,
      sommNodeGrpc: somm_node_grpc,
      publisherDomain: publisher_domain,
      clientCertPath: client_cert_path,
      clientCertKeyPath: client_cert_key_path,
    });
    console.log(result);
  }
</script>
<div class="prose w-screen flex flex-col justify-center items-center">

  <h1 class="text-2xl font-bold mb-4 ">Configure</h1>

    <label for="somm_node_rpc" class="block mb-1 w-full">Sommelier Node RPC URL:</label>
    <input
      type="text"
      id="somm_node_rpc"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={somm_node_rpc}
      placeholder="Enter Sommelier Node RPC URL"
    />

    <label for="somm_node_grpc" class="block mb-1 w-full">Sommelier Node GRPC URL:</label>
    <input
      type="text"
      id="somm_node_grpc"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={somm_node_grpc}
      placeholder="Enter Sommelier Node GRPC URL"
    />


    <label for="publisher_domain" class="block mb-1 w-full">Publisher Domain:</label>
    <input
      type="text"
      id="publisher_domain"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={publisher_domain}
      placeholder="Enter Publisher Domain"
    />

    <label for="client_cert_path" class="block mb-1 w-full">Client Cert Path:</label>
    <input
      type="text"
      id="client_cert_path"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={client_cert_path}
      placeholder="Enter Client Cert Path"
    />

    <label for="client_cert_key_path" class="block mb-1 w-full">Client Cert Key Path:</label>
    <input
      type="text"
      id="client_cert_key_path"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={client_cert_key_path}
      placeholder="Enter Client Cert Key Path"
    />

  <button
    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
    on:click={configure}
  >
    Configure
  </button>
</div>
