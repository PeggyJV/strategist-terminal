<script>
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

<h1>Configure</h1>
Sommelier Node GRPC URL
<input bind:value={somm_node_grpc} placeholder={somm_node_grpc} />
<br />
Publisher Domain
<input bind:value={publisher_domain} placeholder={publisher_domain} />
<br />
Client Cert Path
<input bind:value={client_cert_path} placeholder={client_cert_path} />
<br />
Client Cert Key Path
<input bind:value={client_cert_key_path} placeholder={client_cert_key_path} />

<button on:click={configure}>Configure</button>
