<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let version = "";

    async function status() {
        version = await invoke("version", {});
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

<div>Message from backend: {version}</div>
<button on:click={status}>Get Version</button>
