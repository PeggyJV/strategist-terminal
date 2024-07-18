<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Config from "../components/config.svelte";
  import ScheduleRequest from "../components/ScheduleRequest.svelte";
  import Queue from "../components/Queue.svelte";
  import { queue } from "$stores/AdapterQueue";
  import Template from "../components/AdaptorTemplate.svelte"
  import adaptorList, { type Adaptor } from "$lib/adaptorList"

  let version = "";

  let config = true;
  let aavev3 = true;
  let aavev3a = true;
  let aavev3debt = true;

  let cellar_id = "";
  let block_height = "";
  let chain_id = "";
  let deadline = "";


  let activeAdaptor = adaptorList[0];

  async function status() {
    version = await invoke("version", {});
  }

  function selectAdaptor(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    activeAdaptor = adaptorList.find((a: Adaptor) => a.name ===  target.innerText)
  }
</script>

<div class="min-h-screen flex flex-col justify-center items-center bg-gray-100">

  <h1 class="mb-4 text-3xl font-bold">Strategist Terminal</h1>

  <div class="flex mt-8 space-x-12">
    {#if config}
      <div class="prose flex-1">
        <Config />
      </div>
    {/if}

    <div class="prose flex-1">
      <ScheduleRequest />
    </div>

    {#if $queue.length > 0}
        <Queue />
    {/if}
  </div>

  <div class="flex flex-wrap gap-2.5 mt-12 mx-20">
    {#each adaptorList as adaptor}
      <button
        on:click={selectAdaptor}
        class="p-2.5 border rounded focus:outline-none {adaptor.name === activeAdaptor.name ? 'bg-blue-500 text-white border-blue-500' : 'bg-gray-100 text-black border-gray-300'}"
      >{adaptor.name}</button>
    {/each}
  </div>

  <Template adaptor={activeAdaptor} />

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
    <!-- Empty lines of spaces -->
    <br /><br /><br />
  </div>

</div>
