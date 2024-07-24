<script lang="ts">
  export let toggleModal: () => void;
</script>

<div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
  <!-- Background backdrop -->
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>

  <!-- Modal panel -->
  <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
    <div class="flex min-h-full items-center justify-center p-4 text-center sm:p-0">
      <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
        <!-- Close button -->
        <div class="absolute top-0 right-0 pt-4 pr-4">
          <button type="button" class="text-gray-400 hover:text-gray-500" on:click={toggleModal}>
            <span class="sr-only">Close</span>
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
          <div class="sm:flex sm:items-start">
            <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-blue-100 sm:mx-0 sm:h-10 sm:w-10">
              <svg class="h-6 w-6 text-blue-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 8.75a.75.75 0 100 1.5.75.75 0 000-1.5zm0 3v6m0-12A9 9 0 103 12a9 9 0 009-9z" />
              </svg>
            </div>
            <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
              <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Informational Message</h3>
              <div class="mt-2">
                <p class="text-sm text-gray-500">This is an informational message about the process you have initiated. Please review the details and confirm to proceed.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
