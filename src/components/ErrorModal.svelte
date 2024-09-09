<script lang="ts">
  import { onDestroy } from "svelte";
  import { errorMessage } from "$stores/ErrorStore";

  let message: string | null = null;

  const unsubscribe = errorMessage.subscribe(value => {
    message = value;
  });

  onDestroy(() => {
    unsubscribe();
  });

  function closeModal() {
    errorMessage.set(null);
  }
</script>

{#if message}
  <div class="fixed bottom-10 right-10 bg-black text-white p-7 rounded-lg shadow-lg z-50">
    <button
      class="absolute top-2 right-2 text-xl font-bold text-white hover:text-gray-300"
      on:click={closeModal}
    >
      &times;
    </button>
    {#if message}
      <p>{message}</p>
    {/if}
  </div>
{/if}