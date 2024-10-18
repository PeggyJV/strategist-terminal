<script lang="ts">

  import adaptorList from "$lib/adaptorList"
  import CallSelection from "./CallSelection.svelte"
  import type { Adaptor } from "$lib/type"

  export let closeAdaptorSelection: () => void;

  let selectedAdaptor: Adaptor;
  let searchQuery = "";

  let adaptorListVisible = true;

  $: filteredAdaptors = adaptorList.filter(adaptor =>
    adaptor.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  function selectAdaptor(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    selectedAdaptor = adaptorList.find(
        (a: Adaptor) => a.name ===  target.innerText
      )
      ?? adaptorList[0]
    adaptorListVisible = false;
  }

</script>
<div class="border-2 rounded-2xl p-5">
  {#if adaptorListVisible === true}
    <h2>Select adaptor:</h2>
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
          class="p-2.5 border rounded focus:outline-none bg-gray-100 text-black border-gray-300"
        >{adaptor.name}</button>
      {/each}
    </div>
  {/if}

  {#if selectedAdaptor}
    <CallSelection
      adaptor={selectedAdaptor}
      closeAdaptorSelection={closeAdaptorSelection}/>
  {/if}
</div>


