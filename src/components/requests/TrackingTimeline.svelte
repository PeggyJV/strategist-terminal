<script lang="ts">
  const states = [
    { name: "Initialized", completed: true },
    { name: "Broadcasting", completed: true },
    { name: "Awaiting Consensus", completed: true },
    { name: "Scheduled", completed: false },
    { name: "Awaiting Relay", completed: false },
    { name: "Relayed", completed: false },
    { name: "Success", completed: false }
  ];
</script>

<ol class="list-none">
  {#each states as state, i}
    <li class="relative mb-6 flex">
      <div class="flex flex-col items-center">
        <div class="z-10 flex items-center justify-center w-6 h-6 rounded-full ring-2 shrink-0 {state.completed ? 'ring-green-500' : 'ring-gray-500'}">
          {#if state.completed}
            <svg class="w-4 h-4 text-green-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
              <path d="M16.707 5.293a1 1 0 0 0-1.414 0L9 11.586 6.707 9.293a1 1 0 0 0-1.414 1.414l3 3a1 1 0 0 0 1.414 0l7-7a1 1 0 0 0 0-1.414z"/>
            </svg>
          {:else}
            <svg class="w-4 h-4 text-gray-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="10" stroke-width="2"/>
            </svg>
          {/if}
        </div>
        {#if i !== states.length - 1}
          <div class="w-0.5 h-6 bg-gray-200 dark:bg-gray-700"></div>
        {/if}
      </div>
      <div class="ml-4">
        <h5 class="text-sm font-semibold text-gray-900 dark:text-white">{state.name}</h5>
      </div>
    </li>
  {/each}
</ol>
