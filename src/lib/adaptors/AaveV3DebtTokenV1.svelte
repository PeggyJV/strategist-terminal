<script>
    import { queue, CellarCall } from "$stores/AdapterQueue";
    export let token = "";
    export let amount = "";
    export let underlying_token = "";

    const AaveV3DebtTokenV1Address = "";

    /// async functions communicating with protos

    async function scheduleBorrow() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  BorrowFromAave: {
                      token,
                      amount,
                  },
              }),
            );
            return callQueue;
        })
    }

    async function scheduleRepayDebt() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  RepayAaveDebt: {
                      token,
                      amount,
                  },
              }),
            );
            return callQueue;
        })
    }

    async function scheduleRepayWithATokens() {
        queue.update((callQueue) => {
            callQueue.push(
              new CellarCall(AaveV3DebtTokenV1Address, "AaveV3DebtTokenV1", {
                  RepayWithATokens: {
                      underlying_token,
                      amount,
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
        for="token"
        title="Enter the ERC-20 token contract address as a string."
        >ERC-20 Token Contract Address:</label
    >
    <input type="text" id="token" bind:value={token} placeholder="0xtoken" />
</div>
<div>
    <label
        for="amount"
        title="Enter the amount of the ERC-20 asset to borrow as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<button on:click={scheduleBorrow}>Borrow</button>

<!-- Aave V3 Withdraw -->

<h1>2. Repay Debt with Underlying</h1>
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
        title="Enter the amount of the ERC-20 asset to repay as a string."
        >Amount of ERC-20 Asset:</label
    >
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
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
        for="amount"
        title="Enter the amount of the ERC-20 asset to repay as a string."
        >Amount of AToken to Repay:</label
    >
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
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
