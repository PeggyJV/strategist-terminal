<script lang="ts">

import AdaptorTemplate from "./AdaptorTemplate.svelte"
import adaptorList, { type Adaptor } from "$lib/adaptorList"

let activeAdaptor = adaptorList[0];
let searchQuery = "";

$: filteredAdaptors = adaptorList.filter(adaptor =>
  adaptor.name.toLowerCase().includes(searchQuery.toLowerCase())
);

function selectAdaptor(event: MouseEvent) {
  const target = event.target as HTMLButtonElement;
  activeAdaptor = adaptorList.find(
      (a: Adaptor) => a.name ===  target.innerText
    )
    ?? adaptorList[0]
}

</script>

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

<AdaptorTemplate adaptor={activeAdaptor} />

<!-- Additional text information at the bottom of the page -->
<div class="mt-20 prose">
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