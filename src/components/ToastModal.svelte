<script lang="ts">
  import { onDestroy } from "svelte";
  import { toast, type ToastData, ToastType } from "$stores/ToastStore"

  let toastMsg: ToastData | null = null;

  const unsubscribe = toast.subscribe(value => {
    toastMsg = value;
  });

  onDestroy(() => {
    unsubscribe();
  });

  function closeModal() {
    toast.set(null);
  }
</script>

{#if toastMsg}
  <div class="fixed bottom-10 right-10 text-white p-7 rounded-lg shadow-lg z-50 {toastMsg.type === ToastType.Error ? 'bg-red-500' : 'bg-green-500'}">
    <button
      class="absolute top-2 right-2 text-xl font-bold text-white hover:text-gray-300"
      on:click={closeModal}
    >
      &times;
    </button>
    {#if toastMsg}
      <p>{toastMsg.description}</p>
    {/if}
  </div>
{/if}