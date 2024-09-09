<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte"
  import { errorMessage } from "$stores/ErrorStore"

  let rpc_endpoint = "https://sommelier-rpc.polkachu.com:443";
  let grpc_endpoint = "https://sommelier-grpc.polkachu.com:14190";
  let publisher_domain = "https://smart-strategies.xyz";
  let client_cert_path = "/home/strategy/certs/client.pem";
  let client_cert_key_path = "/home/strategy/certs/client.key";

  interface AppConfig {
    rpc_endpoint: string;
    grpc_endpoint: string;
    publisher_domain: string;
    client_cert_path: string;
    client_cert_key_path: string;
  }

  async function configure() {
    const result = await invoke("configure", {
      sommNodeRpc: rpc_endpoint,
      sommNodeGrpc: grpc_endpoint,
      publisherDomain: publisher_domain,
      clientCertPath: client_cert_path,
      clientCertKeyPath: client_cert_key_path,
    });
    console.log(result);
  }

  async function getAppConf() {
    try {
      ({
        rpc_endpoint,
        grpc_endpoint,
        publisher_domain,
        client_cert_path,
        client_cert_key_path
      } = await invoke<AppConfig>("get_app_config", {}));
    } catch (error) {
      console.error("Error fetching request states", error);
      errorMessage.set("Error fetching request states: " + error);
    }
  }

  onMount(() => {
    getAppConf();
  });
</script>
<div class="prose w-screen flex flex-col justify-center items-center">

  <h1 class="text-2xl font-bold mb-4 ">Configure</h1>

    <label for="somm_node_rpc" class="block mb-1 w-full">Sommelier Node RPC URL:</label>
    <input
      type="text"
      id="somm_node_rpc"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={rpc_endpoint}
      placeholder="Enter Sommelier Node RPC URL"
    />

    <label for="somm_node_grpc" class="block mb-1 w-full">Sommelier Node GRPC URL:</label>
    <input
      type="text"
      id="somm_node_grpc"
      class="w-full px-2 py-1 mb-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
      bind:value={grpc_endpoint}
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
