<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        cellarId,
        blockHeight,
        chainId,
        deadline,
    } from "$stores/scheduleRequestStore";
    import { queue, CellarCall } from "$stores/AdapterQueue";

    export let aaveATokenAdaptorAddress =
        "0x1111111111111111111111111111111111111111";
    export let token = "";
    export let amount = "";
    export let asset = "";
    export let useAsCollateral = false;
    export let categoryId = 0;
    export let cellarAddress = "";

    /// async functions communicating with protos

    async function scheduleDeposit() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(aaveATokenAdaptorAddress, "AaveV3ATokenV1", {
                    DepositToAave: {
                        token,
                        amount,
                    },
                }),
            );
            return callQueue;
        });
    }

    async function scheduleWithdraw() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(aaveATokenAdaptorAddress, "AaveV3ATokenV1", {
                    WithdrawFromAave: {
                        token,
                        amount,
                    },
                }),
            );
            return callQueue;
        });
    }

    async function AdjustIsolationModeAssetAsCollateral() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(aaveATokenAdaptorAddress, "AaveV3ATokenV1", {
                    AdjustIsolationModeAssetAsCollateral: {
                        asset,
                        useAsCollateral,
                    },
                }),
            );
            return callQueue;
        });
    }

    async function ChangeEMode() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(aaveATokenAdaptorAddress, "AaveV3ATokenV1", {
                    ChangeEMode: {
                        categoryId,
                    },
                }),
            );
            return callQueue;
        });
    }
</script>

<!-- Aave V3 Deposit -->

<h1>1. Aave V3 Deposit</h1>
<div>
    <label for="address" title="Enter the address of Aave V3"
        >ERC-20 Token Contract Address:</label
    >
    <input
        type="text"
        id="token"
        bind:value={cellarAddress}
        placeholder="0xcellar"
    />
</div>
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
    <label for="address" title="Enter the address of Aave V3"
        >ERC-20 Token Contract Address:</label
    >
    <input
        type="text"
        id="token"
        bind:value={cellarAddress}
        placeholder="0xcellar"
    />
</div>

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
    <label for="address" title="Enter the address of Aave V3"
        >ERC-20 Token Contract Address:</label
    >
    <input
        type="text"
        id="token"
        bind:value={cellarAddress}
        placeholder="0xcellar"
    />
</div>
<div>
    <label
        for="asset"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="asset" bind:value={asset} placeholder="0xtoken" />
</div>
<div>
    <label for="useAsCollateral" title="Whether to use the asset as collateral."
        >Use as Collateral:</label
    >
    <input
        type="boolean"
        id="amount"
        bind:value={useAsCollateral}
        placeholder="false"
    />
</div>
<button on:click={AdjustIsolationModeAssetAsCollateral}
    >Adjust Isolation Mode Asset As Collateral</button
>

<!-- Change EMode -->

<h1>4. Change EMode</h1>
<div>
    <label for="address" title="Enter the address of Aave V3"
        >ERC-20 Token Contract Address:</label
    >
    <input
        type="text"
        id="token"
        bind:value={cellarAddress}
        placeholder="0xcellar"
    />
</div>
<div>
    <label for="categoryId" title="Enter the categoryId as a uint32"
        >Category ID:</label
    >
    <input
        type="uint32"
        id="categoryId"
        bind:value={categoryId}
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
