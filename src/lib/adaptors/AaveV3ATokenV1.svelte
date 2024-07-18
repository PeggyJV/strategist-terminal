<script lang="ts">
    import { queue, CellarCall } from "$stores/AdapterQueue";

    let token_deposit = "";
    let amount_deposit = "";

    let token_withdraw = "";
    let amount_withdraw = "";

    let asset = "";
    let use_as_collateral = false;

    let category_id = 0;

    const AaveV3ATokenV1Address =
      "0x1111111111111111111111111111111111111111";

    function scheduleDeposit() {
        queue.update((callQueue) => {
            callQueue.push(
                new CellarCall(AaveV3ATokenV1Address, "AaveV3ATokenV1", {
                    DepositToAave: {
                        token: token_deposit,
                        amount: amount_deposit,
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
                        token: token_withdraw,
                        amount: amount_withdraw,
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
                        use_as_collateral,
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
                        category_id,
                    },
                }),
            );
            return callQueue;
        });
    }
</script>

<!-- Aave V3 Deposit -->

<div class="prose mt-10 w-screen">
    <h1>AaveV3ATokenV1</h1>
    <h2>1. Aave V3 Deposit</h2>
    <div class="flex justify-between">
        <label
            for="token_deposit"
            title="Enter the ERC-20 token contract address as a string."
            >ERC-20 Token Contract Address:</label
        >
        <input
          type="text"
          id="token_deposit"
          bind:value={token_deposit}
          placeholder="0xtoken"
          class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <div class="flex justify-between">
        <label
            for="amount_deposit"
            title="Enter the amount of the ERC-20 asset to deposit as a string."
            >Amount of ERC-20 Asset:</label
        >
        <input
          type="text"
          id="amount_deposit"
          bind:value={amount_deposit}
          placeholder="Amount"
          class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <button
      on:click={scheduleDeposit}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
    >Deposit</button>

    <!-- Aave V3 Withdraw -->

    <h2>2. Aave V3 Withdraw</h2>
    <div class="flex justify-between">
        <label
            for="token_withdraw"
            title="Enter the ERC-20 token contract address as a string."
            >ERC-20 Token Contract Address:</label
        >
        <input
          type="text"
          id="token_withdraw"
          bind:value={token_withdraw}
          placeholder="0xtoken"
          class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <div class="flex justify-between">
        <label
            for="amount_withdraw"
            title="Enter the amount of the ERC-20 asset to withdraw as a string."
            >Amount of ERC-20 Asset:</label
        >
        <input
          type="text"
          id="amount_withdraw"
          bind:value={amount_withdraw}
          placeholder="Amount"
          class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <button
      on:click={scheduleWithdraw}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
    >Withdraw</button>

    <!-- Adjust Isolation Mode Asset As Collateral -->

    <h2>3. Adjust Isolation Mode Asset As Collateral</h2>
    <div class="flex justify-between">
        <label
            for="asset"
            title="Enter the ERC-20 token contract address as a string."
            >ERC-20 Token Contract Address:</label
        >
        <input
          type="text"
          id="asset"
          bind:value={asset}
          placeholder="0xtoken"
          class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <div class="flex justify-between">
        <label for="use_as_collateral" title="Whether to use the asset as collateral."
            >Use as Collateral:</label
        >
        <input
            type="text"
            id="use_as_collateral"
            bind:value={use_as_collateral}
            placeholder="boolean"
            class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <button
      on:click={AdjustIsolationModeAssetAsCollateral}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
        >Adjust</button
    >

    <!-- Change EMode -->

    <h2>4. Change EMode</h2>
    <div class="flex justify-between">
        <label for="category_id" title="Enter the category_id as a uint32"
            >Category ID:</label
        >
        <input
            type="text"
            id="category_id"
            bind:value={category_id}
            placeholder="ID"
            class="w-60 px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
    </div>
    <button
      on:click={ChangeEMode}
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
    >Change</button>

</div>