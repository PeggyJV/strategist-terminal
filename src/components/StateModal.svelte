<script lang="ts">
  export let toggleModal: () => void;

  let votingPower = "xxx";
  let currentHeight = "xxx";
  let scheduledHeight = "xxx";
  let corkResult = "xxx";
</script>

<div class="fixed inset-0 z-10 w-screen overflow-y-auto bg-gray-500 bg-opacity-75 transition-opacity" aria-labelledby="modal-title" role="dialog" aria-modal="true">
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
            <span class="h-6 w-6 text-blue-600 text-5xl mx-auto sm:mx-0 sm:h-10 sm:w-10">&#x1F6C8;</span>
          <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Track Schedule ID: 1234</h3>
            <div class="mt-2">
              <!-- Information fields: -->
              <ul class="list-disc pl-5 text-sm text-gray-500">
                <li>Voting power behind a cork: {votingPower}</li>
                <li>Current height: {currentHeight}</li>
                <li>Scheduled height: {scheduledHeight}</li>
                <li>Cork result: {corkResult}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
