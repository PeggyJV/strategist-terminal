<script lang="ts">
  import { Functions } from "$lib/type";
  import { CellarCall } from "$stores/AdapterQueue";
  import ManagementCall from "./ManagementCall.svelte";
  import { administrativeFunctions } from "$lib/administrativeFunctions";

  let callData: {
    function: Functions | null,
    fields: Record<string, string>
  } = {
    function: null,
    fields: {}
  };

  function handleInput(callFunction: Functions, fieldName: string, event: Event) {
    const value = (event.target as HTMLInputElement).value;

    if (callFunction === callData.function) {
      callData.fields = {
        ...callData.fields,
        [fieldName]: value
      };
    }
  }

  let callModalVisible = false;
  function toggleCallModal() {
    callModalVisible = !callModalVisible;
  }

  let call: CellarCall;

  async function callFunction() {
    console.log(callData);

    if (callData.function) {
      call = new CellarCall(callData.function, callData.fields);
    }

    toggleCallModal();
  }

  let openIndex: number | null = null;

  function toggleAccordion(index: number) {
    // If the accordion for the current function is opened, keep it open; otherwise, reset callData
    if (openIndex === index) {
      openIndex = null;
      callData = { function: null, fields: {} };  // Clear callData when closing the accordion
    } else {
      const selectedCall = administrativeFunctions[index];

      // Reset callData when a new call is selected
      callData = {
        function: selectedCall.function,
        fields: selectedCall.fields.reduce((acc: Record<string, string>, field: { name: string }) => {
          acc[field.name] = '';  // Initialize each field with an empty string
          return acc;
        }, {})
      };

      openIndex = index;  // Set the newly selected accordion as open
    }
  }
</script>

<div class="prose w-screen">
  <div class="flex justify-center">
    <h1>Management Calls</h1>
  </div>

  {#if callModalVisible}
    <ManagementCall {toggleCallModal} {call} />
  {/if}

  {#each administrativeFunctions as call, index}
    <div class="accordion-item border-b border-gray-200 mt-4">
      <!-- Accordion Header -->

      <h2>
        <button
          type="button"
          class="accordion-header flex justify-between items-center cursor-pointer bg-gray-100 py-3 px-4 font-semibold w-full"
          on:click={() => toggleAccordion(index)}
        >
          {call.function}
          <span class="ml-auto">{openIndex === index ? '▲' : '▼'}</span>
        </button>
      </h2>


      <!-- Accordion Content -->
      {#if openIndex === index}
        <div class="accordion-content p-4 transition-all duration-300">
          {#each call.fields as field}
            <div class="flex justify-between mt-2">
              <label for="{`${call.function}-${field.name}`}" class="mr-4">{field.label}:</label>
              <input
                value={callData.fields[field.name] || ''}
                on:input={(event) => handleInput(call.function, field.name, event)}
                id="{`${call.function}-${field.name}`}"
                placeholder="{field.placeholder}"
                class="w-full px-2 py-1 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500"
              />
            </div>
          {/each}

          <button
            on:click={callFunction}
            type="button"
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600 mt-3"
          >
            {call.action}
          </button>
        </div>
      {/if}
    </div>
  {/each}
</div>
