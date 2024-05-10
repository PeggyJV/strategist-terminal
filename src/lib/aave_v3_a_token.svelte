<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        cellarId,
        blockHeight,
        chainId,
        deadline,
    } from "$stores/scheduleRequestStore";

    export let token = "";
    export let amount = "";
    export let asset = "";
    export let use_as_collateral = false;
    export let category_id = 0;

    /// async functions communicating with protos

    async function scheduleDeposit() {
        const result = await invoke("DepositToAave", { token, amount });
        console.log(result);
    }

    async function scheduleWithdraw() {
        const result = await invoke("WithdrawFromAave", { token, amount });
        console.log(result);
    }

    async function AdjustIsolationModeAssetAsCollateral() {
        const result = await invoke("AdjustIsolationModeAssetAsCollateral", {
            asset,
            use_as_collateral,
        });
        console.log(result);
    }

    async function ChangeEMode() {
        const result = await invoke("ChangeEMode", { category_id });
        console.log(result);
    }
</script>

<!-- Aave V3 Deposit -->

<h1>1. Aave V3 Deposit</h1>
<div>
    <label
        for="token"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount"
        title="Enter the amount of the ERC-20 asset to deposit as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- Aave V3 Withdraw -->

<h1>2. Aave V3 Withdraw</h1>
<div>
    <label
        for="token"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount"
        title="Enter the amount of the ERC-20 asset to withdraw as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleWithdraw}>Withdraw</button>

<!-- Adjust Isolation Mode Asset As Collateral -->

<h1>3. Adjust Isolation Mode Asset As Collateral</h1>
<div>
    <label
        for="asset"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="asset" bind:value={asset} placeholder="0xtoken" />
</div>
<div>
    <label
        for="use_as_collateral"
        title="Whether to use the asset as collateral."
        >Use as Collateral:</label
    >
    <input
        type="boolean"
        id="amount"
        bind:value={use_as_collateral}
        placeholder="false"
    />
</div>
<button on:click={AdjustIsolationModeAssetAsCollateral}
    >Adjust Isolation Mode Asset As Collateral</button
>

<!-- Change EMode -->

<h1>4. Change EMode</h1>
<div>
    <label for="category_id" title="Enter the categoryId as a uint32"
        >Category ID:</label
    >
    <input
        type="uint32"
        id="category_id"
        bind:value={category_id}
        placeholder="0"
    />
</div>
<button on:click={ChangeEMode}>ChangeEMode</button>

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
