<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Aavev3Deposit from "$lib/aavev3Deposit.svelte";
    import Config from "$lib/config.svelte";

    let version = "";

    let config = true;
    let aavev3 = true;

    async function status() {
        version = await invoke("version", {});
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

{#if config}
    <Config />
{/if}

{#if aavev3}
    <Aavev3Deposit />
{/if}
