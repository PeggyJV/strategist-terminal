<script lang="ts">
  import StateModal from "./StateModal.svelte"
  import { invoke } from "@tauri-apps/api/tauri"
  import { ActiveRequestStatus, FailedRequestStatus, type RequestState } from "$lib/type"
  import { onMount } from "svelte"
  import { toast, ToastType } from "$stores/ToastStore"
  import VersionsModal from "./VersionsModal.svelte"

  export let requests: Map<string, RequestState> = new Map();

  let statesVisible = false;
  let versionsVisible = false;
  let activeRequest: string;

  async function getRequestStates() {
    try {
      requests = await invoke("request_state", {});
    } catch (error) {
      console.error("Error fetching request states", error);
      toast.set({
        type: ToastType.Error,
        description: "Error scheduling a request: " + error
      });
    }
  }

  onMount(() => {
    getRequestStates();
  });

  function toggleStatesModal(event?: MouseEvent) {
    if (!event) {
      statesVisible = !statesVisible;
      return;
    }
    const target = event.target as HTMLButtonElement;

    activeRequest = target.innerText ?? requests.keys().next().value
    statesVisible = !statesVisible;
  }

  function toggleVersionsModal() {
    versionsVisible = !versionsVisible;
  }


</script>
<div class="prose w-screen">

  <div class="flex justify-center">
    <h1>Requests</h1>
  </div>


  <div class="flex justify-center">
    <button
      on:click={toggleVersionsModal}
      value="Settings"
      class="p-2.5 mx-5 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
    >Steward Versions</button>
  </div>


  <table class="mx-auto text-center">
    <thead>
    <tr>
      <th>ID</th>
      <th>State</th>
    </tr>
    </thead>
    <tbody>
    {#if !requests.size || requests.size === 0}
      <tr>
        <td>No requests to show!</td>
      </tr>
    {:else}
      {#each requests as [key, value]}
        <tr>
          <th>
            <button on:click={toggleStatesModal}>
              {key}
            </button>
          </th>
          <td>
              {value.status}
          </td>
        </tr>
      {/each}
    {/if}
    </tbody>
  </table>

  {#if statesVisible}
    <StateModal {toggleStatesModal} request={requests.get(activeRequest)} />
  {/if}

  {#if versionsVisible}
    <VersionsModal {toggleVersionsModal} />
  {/if}
</div>
