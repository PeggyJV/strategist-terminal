<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let f_token = "";
  let amount_to_deposit = "";
  const FTokenV1Address = "";

  function scheduleLendFrax() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FTokenV1Address, "FTokenV1", {
          LendFrax: {
            f_token,
            amount_to_deposit
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<h1>1. LendFrax</h1>
<div>
  <label for="f_token" title="The address of the fToken to lend.">The address of the fToken to lend:</label>
  <input type="text" id="f_token" bind:value={f_token} placeholder="Address" />
</div>

<div>
  <label for="amount_to_deposit" title="The amount of the fToken to lend.">The amount of the fToken to lend:</label>
  <input type="text" id="amount_to_deposit" bind:value={amount_to_deposit} placeholder="Amount" />
</div>


<button on:click={scheduleLendFrax}>Swap assets</button>
