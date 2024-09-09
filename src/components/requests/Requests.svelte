<script lang="ts">
  import StateModal from "../StateModal.svelte"
  import { invoke } from "@tauri-apps/api/tauri"
  import type { Request } from "$lib/type"
  import { onMount } from "svelte"

  export let requests: Map<string, Request> = new Map();

  let modalVisible = false;
  let activeRequest: string;

  async function getRequestStates() {
    try {
      requests = await invoke("request_state", {});
    } catch (error) {
      console.error("Error fetching request states", error);
    }
  }

  function toggleModal(event?: MouseEvent) {
    if (!event) {
      modalVisible = !modalVisible;
      return;
    }

    const target = event.target as HTMLButtonElement;

    activeRequest = requests.get(target.innerText) ?? requests.keys().next().value

    modalVisible = !modalVisible;
  }

  onMount(() => {
    getRequestStates();
  });

</script>
<div class="prose w-screen">

  <div class="flex justify-center">
    <h1>Requests</h1>
  </div>

  <button
    on:click={getRequestStates}
    value="Settings"
    class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 border-b-2 border-blue-500 text-blue-500"
  />

  <table class="mx-auto text-center">
    <thead>
    <tr>
      <th>ID</th>
      <th>State</th>
      <th>Current Height</th>
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
            <button on:click={toggleModal}>
              {key}
            </button>
          </th>
          <td>{value.status}</td>
          <td>{value.currentHeight}</td>
        </tr>
      {/each}
    {/if}
    </tbody>
  </table>

  {#if modalVisible}
    <StateModal {toggleModal} request={requests.get(activeRequest)} />
  {/if}
</div>
