<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { toast, ToastType } from "$stores/ToastStore";
  import { onMount, onDestroy } from "svelte";

  export let toggleVersionsModal: () => void;
  let versions: Map<string, string> | null = null;
  let pollingInterval: Timer;

  async function fetchVersions() {
    try {
      versions = await invoke("steward_versions", {});
      console.log(versions);

      if (versions && Object.entries(versions).length > 0) {
        clearInterval(pollingInterval);
      }
    } catch (error) {
      console.error("Error fetching steward versions", error);
      toast.set({
        type: ToastType.Error,
        description: "Error fetching steward versions: " + error,
      });
    }
  }

  function pollVersions() {
    fetchVersions();

    pollingInterval = setInterval(() => {
      if (!versions || Object.entries(versions).length === 0) {
        fetchVersions();
      }
    }, 5000);
  }

  onMount(() => {
    pollVersions();
  });

  onDestroy(() => {
    if (pollingInterval) {
      clearInterval(pollingInterval);
    }
  });
</script>

<div class="fixed inset-0 z-10 w-screen overflow-y-auto bg-gray-500 bg-opacity-75 transition-opacity" aria-labelledby="modal-title" role="dialog" aria-modal="true">
  <div class="flex min-h-full items-center justify-center p-4 text-center sm:p-0">
    <div class="absolute transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8">
      <!-- Close button -->
      <div class="absolute top-2 right-4">
        <button
          class="absolute top-2 right-2 text-xl font-bold text-gray-400 hover:text-gray-500"
          on:click={toggleVersionsModal}
        >
          &times;
        </button>
      </div>
      <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
        <span class="h-6 w-6 text-blue-600 text-5xl mx-auto sm:mx-0 sm:h-10 sm:w-10">&#x1F6C8;</span>
        <div class="sm:flex sm:items-start">
          <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Steward Versions</h3>
            <div class="mt-2">
              {#if !versions || Object.entries(versions).length === 0}
                <div class="flex flex-col justify-center items-center">
                  <div id="loading-spinner" class="w-8 h-8 border-4 border-gray-200 border-t-blue-500 rounded-full animate-spin"></div>
                  <span class="mt-2 text-gray-500">Initial loading may take a few minutes.</span>
                </div>
              {:else}
                <ul class="list-disc pl-5 text-sm text-gray-500 max-h-[500px] overflow-y-auto">
                  {#each Object.entries(versions) as [key, value]}
                    <li>{key}: <b>{value}</b></li>
                  {/each}
                </ul>
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
