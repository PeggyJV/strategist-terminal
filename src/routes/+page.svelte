<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Config from "../components/settings/config.svelte";
  import ScheduleRequest from "../components/ScheduleRequest.svelte";
  import Queue from "../components/Queue.svelte";
  import { queue } from "$stores/AdapterQueue";
  import AdaptorsTab from "../components/adaptors/AdaptorsTab.svelte"
  import FlashLoansTab from "../components/flashLoan/FlashLoansTab.svelte"
  import Requests from "../components/requests/Requests.svelte"
  import ErrorModal from "../components/ToastModal.svelte"
  import Management from "../components/management/Management.svelte"

  let version = "";

  async function status() {
    version = await invoke("version", {});
  }

  enum Tabs {
    Adaptors,
    FlashLoans,
    Settings,
    Requests,
    Management
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

    <button
      on:click={selectTab}
      value="Management"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {activeTab === Tabs.Management ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >Management</button>

    <button
      on:click={selectTab}
      value="Requests"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {activeTab === Tabs.Requests ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >Requests</button>

    <button
      on:click={selectTab}
      value="Settings"
      class="p-2.5 mx-5 focus:outline-none transition-colors duration-200 {activeTab === Tabs.Settings ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-500 hover:text-blue-500'}"
    >Settings</button>
  </div>


  <div class="flex mt-8  w-screen">
    <!-- Left column, 15% width -->
    <div class="px-10 w-1/5 min-w-[250px]">
    </div>

    <!-- Middle column, 70% width -->
    <div class="flex-1 flex flex-col items-center min-w-[500px] w-3/5">

      {#if activeTab === Tabs.Adaptors}
        <AdaptorsTab />
      {/if}

      {#if activeTab === Tabs.FlashLoans}
        <FlashLoansTab />
      {/if}

      {#if activeTab === Tabs.Management}
        <Management />
      {/if}

      {#if activeTab === Tabs.Settings}
        <Config />
      {/if}

      {#if activeTab === Tabs.Requests}
        <Requests />
      {/if}

    </div>

    <!-- Right column, 15% width -->
    <div class="flex flex-col justify-start px-10 w-1/5 min-w-[250px]">

    {#if activeTab === Tabs.Adaptors || activeTab === Tabs.FlashLoans}
        <div class="prose min-w-200" >
          <ScheduleRequest />
        </div>
        {#if $queue.length > 0}
          <Queue />
        {/if}
    {/if}

    </div>
  </div>

  <ErrorModal />

</div>
