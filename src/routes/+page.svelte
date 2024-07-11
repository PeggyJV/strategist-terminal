<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Aavev3Deposit from "$lib/adaptors/aavev3Deposit.svelte";
    import Config from "$lib/config.svelte";
    import AaveV3AToken from "$lib/adaptors/AaveV3ATokenV1.svelte";
    import Aavev3Debt from "$lib/adaptors/AaveV3DebtTokenV1.svelte";
    import ScheduleRequest from "$lib/ScheduleRequest.svelte";
    import AaveATokenV1 from "$lib/adaptors/AaveATokenV1.svelte"
    import AaveDebtTokenV1 from "$lib/adaptors/AaveDebtTokenV1.svelte"
    import CompoundCTokenV2 from "$lib/adaptors/CompoundCTokenV2.svelte"
    import AaveATokenV2 from "$lib/adaptors/AaveATokenV2.svelte"
    import AaveDebtTokenV2 from "$lib/adaptors/AaveDebtTokenV2.svelte"
    import OneInchV1 from "$lib/adaptors/OneInchV1.svelte"
    import FeesAndReservesV1 from "$lib/adaptors/FeesAndReservesV1.svelte"
    import ZeroXV1 from "$lib/adaptors/ZeroXV1.svelte"
    import SwapWithUniswapV1 from "$lib/adaptors/SwapWithUniswapV1.svelte"
    import VestingSimpleV2 from "$lib/adaptors/VestingSimpleV2.svelte"

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
        "Aavev3Debt": Aavev3Debt,
        "AaveATokenV1": AaveATokenV1,
        "AaveDebtTokenV1": AaveDebtTokenV1,
        "CompoundCTokenV2": CompoundCTokenV2,
        "AaveATokenV2": AaveATokenV2,
        "AaveDebtTokenV2": AaveDebtTokenV2,
        "OneInchV1": OneInchV1,
        "FeesAndReservesV1": FeesAndReservesV1,
        "ZeroXV1": ZeroXV1,
        "SwapWithUniswapV1": SwapWithUniswapV1,
        "VestingSimpleV2": VestingSimpleV2
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
        flex-wrap: wrap;
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

<!-- Additional text information at the bottom of the page -->
<div>
    <h2>Specific Strategist Information</h2>
    <p>
        A. Keep in mind regarding isolated markets. If they are isolated, then
        they may not be usable as collateral for other borrow positions.
    </p>
    <p>
        B. See the <a
      href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture"
    >docs</a
    > for more information pertaining to this adaptor.
    </p>
    <!-- Empty lines of spaces -->
    <br /><br /><br />
</div>

