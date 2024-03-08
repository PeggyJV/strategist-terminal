<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Aavev3 from "$lib/aavev3.svelte";

    let version = "";

    async function status() {
        version = await invoke("version", {});
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>
<Aavev3 />

<div>Message from backend: {version}</div>
<button on:click={status}>Get Version</button>
