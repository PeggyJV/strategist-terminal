<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Aavev3Deposit from "$lib/adaptors/aavev3Deposit.svelte";
    import Config from "$lib/config.svelte";
    import AaveV3AToken from "$lib/adaptors/AaveV3ATokenV1.svelte";
    import Aavev3Debt from "$lib/adaptors/AaveV3DebtTokenV1.svelte";
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

    const map: {  [key: string]: ConstructorOfATypedSvelteComponent} = {
        "Aavev3Deposit": Aavev3Deposit,
        "AaveV3AToken": AaveV3AToken,
        "Aavev3Debt": Aavev3Debt
    }
    let displayedAdaptor = Object.values(map)[0]
    let activeButton = Object.keys(map)[0];

    async function status() {
        version = await invoke("version", {});
    }

    function selectAdaptor(event: MouseEvent) {
        const target = event.target as HTMLButtonElement;
        activeButton = target.innerText;
        displayedAdaptor = map[target.innerText];
    }


</script>

<style>
    .horizontal-list {
        display: flex;
        gap: 10px;
        margin: 30px;
    }
    .horizontal-list button {
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        background-color: #f9f9f9;
    }
    .horizontal-list button.active {
        background-color: #007bff;
        color: white;
        border-color: #007bff;
    }
</style>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

{#if config}
    <Config />
{/if}

<ScheduleRequest />

<div class="horizontal-list">
    {#each Object.keys(map) as item}
        <button
          on:click={selectAdaptor}
          class:active={item === activeButton}
        >{item}</button>
    {/each}
</div>

<svelte:component this={displayedAdaptor} />
