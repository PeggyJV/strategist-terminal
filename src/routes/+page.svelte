<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Aavev3Deposit from "$lib/aavev3Deposit.svelte";
    import Config from "$lib/config.svelte";
    import AaveV3AToken from "$lib/aave_v3_a_token.svelte";
    import Aavev3Debt from "$lib/aave_v3_debt.svelte";
    import ScheduleRequest from "$lib/ScheduleRequest.svelte";
    let version = "";

    let config = true;
    let aavev3 = true;
    let aavev3a = true;
    let aavev3debt = true;

    let cellar_id = "";
    let block_height = "";
    let chain_id = "";
    let deadline = "";

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

<ScheduleRequest />

{#if aavev3}
    <Aavev3Deposit
        cellarId={cellar_id}
        blockHeight={block_height}
        chainId={chain_id}
        {deadline}
    /> />
{/if}

{#if aavev3a}
    <AaveV3AToken />
{/if}

{#if aavev3debt}
    <Aavev3Debt />
{/if}
