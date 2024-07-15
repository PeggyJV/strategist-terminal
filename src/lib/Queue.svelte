<script lang="ts">
  import { queue } from "$stores/AdapterQueue";

  function clearQueue() {
    queue.set([]);
  }

</script>

<style>
    .wrapper {
        width: 300px;
        height: 100px;
        margin-right: 100px;
    }
</style>

<div class="wrapper">
  <h2>Queue:</h2>

  <div>
    {#each $queue as item}
      <h2>{item.name}</h2>
      {#each Object.entries(item.fields) as [key, value]}
        * {key}: {JSON.stringify(value, null, 2)}
      {/each}
    {/each}
  </div>

  <button on:click={clearQueue}>Clear queue</button>

</div>
