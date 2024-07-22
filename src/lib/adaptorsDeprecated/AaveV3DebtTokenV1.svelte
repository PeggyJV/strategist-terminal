<script lang="ts">
    import { queue, CellarCall } from "$stores/AdapterQueue";

    let token_borrow = "";
    let amount_borrow = "";

    let token_repay = "";
    let amount_repay = "";

    let amount_repay_with_a = "";
    let underlying_token = "";

    const AaveV3DebtTokenV1Address = "";

    /// async functions communicating with protos

    function scheduleBorrow() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  BorrowFromAave: {
                      token: token_borrow,
                      amount: amount_borrow,
                  },
              }),
            );
            return callQueue;
        })
    }

    function scheduleRepayDebt() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  RepayAaveDebt: {
                      token: token_repay,
                      amount: amount_repay,
                  },
              }),
            );
            return callQueue;
        })
    }

    function scheduleRepayWithATokens() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  RepayWithATokens: {
                      underlying_token,
                      amount: amount_repay_with_a,
                  },
              }),
            );
            return callQueue;
        })
    }
</script>

<!-- Aave V3 Deposit -->

<h1>1. Aave V3 Borrow</h1>
<div>
    <label
        for="token_borrow"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token_borrow" bind:value={token_borrow} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount_borrow"
        title="Enter the amount of the ERC-20 asset to borrow as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount_borrow" bind:value={amount_borrow} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. Repay Debt with Underlying</h1>
<div>
    <label
        for="token_repay"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token_repay" bind:value={token_repay} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount_repay"
        title="Enter the amount of the ERC-20 asset to repay as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount_repay" bind:value={amount_repay} placeholder="Amount" />
</div>
<button on:click={scheduleRepayDebt}>Repay Debt with Underlying</button>

<!-- Repay Debt with AToken -->

<h1>3. Repay Debt with AToken</h1>
<div>
    <label
        for="underlying_token"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input
        type="text"
        id="underlying_token"
        bind:value={underlying_token}
        placeholder="0xtoken"
    />
</div>
<div>
    <label
        for="amount_repay_with_a"
        title="Enter the amount of the ERC-20 asset to repay as a string."
        >Amount of AToken to Repay:</label
    >
    <input type="text" id="amount_repay_with_a" bind:value={amount_repay_with_a} placeholder="Amount" />
</div>
<button on:click={scheduleRepayWithATokens}>Repay Debt with AToken</button>

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
