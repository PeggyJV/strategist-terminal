<script lang="ts">
  import type { RequestState } from "$lib/type"
  import TrackingTimeline from "./TrackingTimeline.svelte"

  export let toggleStatesModal: (event?: MouseEvent) => void;
  export let request: RequestState | undefined;

</script>

<div class="fixed inset-0 z-10 w-screen overflow-y-auto bg-gray-500 bg-opacity-75 transition-opacity" aria-labelledby="modal-title" role="dialog" aria-modal="true">
  <div class="flex min-h-full items-center justify-center p-4 text-center sm:p-0">
    <div class=" absolute transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8">
      <!-- Close button -->
        <div class="absolute top-0 right-0 pt-4 pr-4">
          <button
            class="absolute top-2 right-2 text-xl font-bold text-gray-400 hover:text-gray-500"
            on:click={toggleStatesModal}
          >
            &times;
          </button>
      </div>
      <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
        <span class="h-6 w-6 text-blue-600 text-5xl mx-auto sm:mx-0 sm:h-10 sm:w-10">&#x1F6C8;</span>
        <div class="sm:flex sm:items-start">
          <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">Request: {request?.id ?? ""}</h3>
            <div class="mt-2">
              <!-- Information fields: -->
              <ul class="list-disc pl-5 text-sm text-gray-500">
                <li>Cork ID Hash:
                  <br>
                  <b>{request?.corkId ?? ""}</b>
                </li>
                <li>Gravity invalidation scope:
                  <br>
                  <b>{request?.invalidateScope ?? ""}</b>
                </li>
                <li>The Sommelier transaction hash for the IBC relay request:
                  <br>
                  <b>{request?.relayRequestTxHash ?? ""}</b>
                </li>
                <li>Axelar GMP transaction hash:
                  <br>
                  <b>{request?.gmpTxHash ?? ""}</b>
                </li>
                <li>Transaction hash on the target chain:
                  <br>
                  <b>{request?.targetTxHash ?? ""}</b>
                </li>
              </ul>
            </div>
          </div>

          <TrackingTimeline state={request?.status}/>
        </div>
      </div>
    </div>
  </div>
</div>
