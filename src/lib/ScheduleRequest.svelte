<script>
    import {
        cellarId,
        blockHeight,
        chainId,
        deadline,
    } from "$stores/scheduleRequestStore";
</script>

<h1>Schedule Request</h1>

<div>
    <label for="cellar_id" title="Enter the cellar id as a string."
        >Cellar ID:</label
    >
    <input
        type="text"
        id="cellar_id"
        bind:value={$cellarId}
        placeholder="Cellar ID"
    />
</div>
<div>
    <label for="block_height" title="Enter the block height as a string."
        >Block Height:</label
    >
    <input
        type="text"
        id="block_height"
        bind:value={$blockHeight}
        placeholder="Block Height"
    />
</div>
<div>
    <label for="chain_id" title="Enter the chain id as a string."
        >Chain ID:</label
    >
    <input
        type="text"
        id="chain_id"
        bind:value={$chainId}
        placeholder="Chain ID"
    />
</div>
<div>
    <label for="deadline" title="Enter the deadline as a boolean."
        >Deadline:</label
    >
    <input
        type="text"
        id="deadline"
        bind:value={$deadline}
        placeholder="Deadline"
    />
</div>
