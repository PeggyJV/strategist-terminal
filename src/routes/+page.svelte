<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Config from "../components/config.svelte";
  import ScheduleRequest from "../components/ScheduleRequest.svelte";
  import Queue from "../components/Queue.svelte";
  import { queue } from "$stores/AdapterQueue";
  import Template from "../components/AdaptorTemplate.svelte"
  import adaptorList, { type Adaptor } from "$lib/adaptorList"
  import StateModal from "../components/StateModal.svelte"

  let version = "";

  let activeAdaptor = adaptorList[0];
  let searchQuery = "";

  let modalVisible = false;


  $: filteredAdaptors = adaptorList.filter(adaptor =>
    adaptor.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  async function status() {
    version = await invoke("version", {});
  }

  function selectAdaptor(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    activeAdaptor = adaptorList.find(
        (a: Adaptor) => a.name ===  target.innerText
      )
      ?? adaptorList[0]
  }

  function toggleModal() {
    modalVisible = !modalVisible;
  }

</script>

<div class="min-h-screen flex flex-col items-center bg-gray-100 overflow-hidden">
    <h1 class="m-4 text-3xl font-bold">Strategist Terminal</h1>
    <button on:click={toggleModal} class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600">
        Track Schedule ID: 123 
    </button>

  <div class="flex mt-8  w-screen">
    <!-- Left column, 15% width -->
    <div class="px-10 w-1/5 min-w-[250px]">
      <Config />
    </div>

    <!-- Middle column, 70% width -->
    <div class="flex-1 flex flex-col items-center min-w-[500px] w-3/5">

      <div class="mb-2 w-full max-w-md">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search adaptors..."
          class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
        />
      </div>

      <div class="flex flex-wrap gap-2.5 mt-12 justify-center">
        {#each filteredAdaptors as adaptor}
          <button
            on:click={selectAdaptor}
            class="p-2.5 border rounded focus:outline-none {adaptor.name === activeAdaptor.name ? 'bg-blue-500 text-white border-blue-500' : 'bg-gray-100 text-black border-gray-300'}"
          >{adaptor.name}</button>
        {/each}
      </div>

      <Template adaptor={activeAdaptor} />
    </div>

    <!-- Right column, 15% width -->
    <div class="flex flex-col justify-start px-10 w-1/5 min-w-[250px]">
      <div class="prose min-w-200" >
        <ScheduleRequest />
      </div>
      {#if $queue.length > 0}
        <Queue />
      {/if}
    </div>
  </div>


  <!-- Additional text information at the bottom of the page -->
  <div class="mt-8 prose">
    <h2>Specific Strategist Information</h2>
    <p>
      A. Keep in mind regarding isolated markets. If they are isolated, then
      they may not be usable as collateral for other borrow positions.
    </p>
    <p>
      B. See the <a href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture">docs</a> for more information pertaining to this adaptor.
    </p>
    <br /><br /><br />
  </div>

  {#if modalVisible}
    <StateModal {toggleModal}/>
  {/if}

</div>
