<script lang="ts">
  import { RequestStatus } from "$lib/type"

  export let state;

  let statesList = Object.keys(RequestStatus)

  let indexOfCurrentState = statesList.indexOf(state);
</script>

<ol class="list-none">
  {#each statesList as state, i}
    <li class="relative flex not-prose">
      <div class="flex flex-col items-center">
        <div class="z-10 flex items-center justify-center w-6 h-6 rounded-full ring-2 shrink-0 {i <= indexOfCurrentState ? 'ring-green-400' : 'ring-gray-400'}">
          {#if i <= indexOfCurrentState}
            <svg class="w-4 h-4 text-green-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
              <path d="M16.707 5.293a1 1 0 0 0-1.414 0L9 11.586 6.707 9.293a1 1 0 0 0-1.414 1.414l3 3a1 1 0 0 0 1.414 0l7-7a1 1 0 0 0 0-1.414z"/>
            </svg>
          {/if}
        </div>
        {#if i !== statesList.length - 1}
          <div class="w-0.5 h-6 dark:bg-gray-700 {i < indexOfCurrentState ? 'bg-green-200' : 'bg-gray-200'}"></div>
        {/if}
      </div>
      <div class="ml-4">
        <h5 class="text-sm font-semibold text-gray-900 dark:text-white">{state}</h5>
      </div>
    </li>
  {/each}
</ol>
