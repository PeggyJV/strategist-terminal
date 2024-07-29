<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Config from "../components/config.svelte";
  import ScheduleRequest from "../components/ScheduleRequest.svelte";
  import Queue from "../components/Queue.svelte";
  import { queue } from "$stores/AdapterQueue";
  import AdaptorsTab from "../components/adaptors/AdaptorsTab.svelte"
  import FlashLoansTab from "../components/flashLoan/FlashLoansTab.svelte"

  let version = "";

  async function status() {
    version = await invoke("version", {});
  }

  enum Tabs {
    Adaptors = "adaptor",
    FlashLoans = "flashLoans"
  }

  let activeTab = Tabs.Adaptors

  function selectTab(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    const tab = target.value as keyof typeof Tabs;
    activeTab = Tabs[tab];
  }
</script>

<div class="min-h-screen flex flex-col items-center bg-gray-100 overflow-hidden">

  <h1 class="m-4 text-3xl font-bold">Strategist Terminal</h1>

  <div class="flex flex-row">
    <button
      on:click={selectTab}
      value="Adaptors"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {activeTab === Tabs.Adaptors ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >Adaptors</button>

    <button
      on:click={selectTab}
      value="FlashLoans"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {activeTab === Tabs.FlashLoans ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >Flash Loans</button>
  </div>


  <div class="flex mt-8  w-screen">
    <!-- Left column, 15% width -->
    <div class="px-10 w-1/5 min-w-[250px]">
      <Config />
    </div>

    <!-- Middle column, 70% width -->
    <div class="flex-1 flex flex-col items-center min-w-[500px] w-3/5">

      {#if activeTab === Tabs.FlashLoans}
        <FlashLoansTab />
      {/if}

      {#if activeTab === Tabs.Adaptors}
        <AdaptorsTab />
      {/if}

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

</div>
