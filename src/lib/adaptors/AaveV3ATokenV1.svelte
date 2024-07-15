<script lang="ts">
    import { queue, CellarCall } from "$stores/AdapterQueue";

    let token_deposit = "";
    let amount_deposit = "";

    let token_withdraw = "";
    let amount_withdraw = "";

    let asset = "";
    let useAsCollateral = false;

    let categoryId = 0;

    const AaveV3ATokenV1Address =
      "0x1111111111111111111111111111111111111111";

    function scheduleDeposit() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(AaveV3ATokenV1Address, "AaveV3ATokenV1", {
                    DepositToAave: {
                        token_deposit,
                        amount_deposit,
                    },
                }),
            );
            return callQueue;
        });
    }

    function scheduleWithdraw() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(AaveV3ATokenV1Address, "AaveV3ATokenV1", {
                    WithdrawFromAave: {
                        token_withdraw,
                        amount_withdraw,
                    },
                }),
            );
            return callQueue;
        });
    }

    function AdjustIsolationModeAssetAsCollateral() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(AaveV3ATokenV1Address, "AaveV3ATokenV1", {
                    AdjustIsolationModeAssetAsCollateral: {
                        asset,
                        useAsCollateral,
                    },
                }),
            );
            return callQueue;
        });
    }

    function ChangeEMode() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(AaveV3ATokenV1Address, "AaveV3ATokenV1", {
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
    <label
        for="token_deposit"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token_deposit" bind:value={token_deposit} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount_deposit"
        title="Enter the amount of the ERC-20 asset to deposit as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount_deposit" bind:value={amount_deposit} placeholder="Amount" />
</div>
<button on:click={scheduleDeposit}>Deposit</button>

<!-- Aave V3 Withdraw -->

<h1>2. Aave V3 Withdraw</h1>
<div>
    <label
        for="token_withdraw"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token_withdraw" bind:value={token_withdraw} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount_withdraw"
        title="Enter the amount of the ERC-20 asset to withdraw as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount_withdraw" bind:value={amount_withdraw} placeholder="Amount" />
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
    <label for="useAsCollateral" title="Whether to use the asset as collateral."
        >Use as Collateral:</label
    >
    <input
        type="boolean"
        id="amount"
        bind:value={useAsCollateral}
        placeholder="boolean"
    />
</div>
<button on:click={AdjustIsolationModeAssetAsCollateral}
    >Adjust Isolation Mode Asset As Collateral</button
>

<!-- Change EMode -->

<h1>4. Change EMode</h1>
<div>
    <label for="categoryId" title="Enter the categoryId as a uint32"
        >Category ID:</label
    >
    <input
        type="text"
        id="categoryId"
        bind:value={categoryId}
        placeholder="ID"
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
