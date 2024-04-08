<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let token_in = "";
    export let token_out = "";
    export let amount = "";
    export let swap_call_data = "";

    /// async functions communicating with protos
    
    /// Base Adaptor Functions - TODO: do we need revokeApproval here, I don't see a `message` within the proto.

    /// Adaptor Specific Functions
    async function swapWithOneInch() {
        const result = await invoke("SwapWithOneInch", { token_in, token_out, amount, swap_call_data});
        console.log(result);
    }

</script>

<!-- SwapWithOneInch -->

<h1>1. SwapWithOneInch </h1>
<div>
    <label for="token_in" title="Enter the ERC-20 token address as a string.">ERC-20 Token_In Contract Address:</label>
    <input type="text" id="token_in" bind:value={token_in} placeholder="0xtoken" />
</div>
<div>
    <label for="token_out" title="Enter the ERC-20 token address as a string.">ERC-20 Token_Out Contract Address:</label>
    <input type="text" id="token_out" bind:value={token_out} placeholder="0xtoken" />
</div>
<div>
    <label for="amount" title="Enter the amount of the ERC-20 asset to borrow as a string.">Amount of ERC-20 Asset:</label>
    <input type="text" id="amount" bind:value={amount} placeholder="Amount" />
</div>
<div>
    <label for="swap_call_data" title="Enter the swap_call_dataas a bytes32.">Swap Calldata:</label>
    <input type=0x; id="swap_call_data" bind:value={swap_call_data} placeholder="Swap Calldata" />
</div>

<button on:click={swapWithOneInch}>Swap with OneInch</button>

<!-- Additional text information at the bottom of the page -->
<div>
    <h2>Specific Strategist Information</h2>
    <p>A. TODO  </p>
    <p>B. TODO - for more info, see the docs! </p>
    <!-- Empty lines of spaces -->
    <br /><br /><br />
</div>