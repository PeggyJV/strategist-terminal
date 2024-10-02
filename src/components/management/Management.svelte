<script lang="ts">
  import { Functions } from "$lib/type"
  import { CellarCall } from "$stores/AdapterQueue";
  import ManagementCall from "./ManagementCall.svelte";
  import { administrativeFunctions } from "$lib/administrativeFunctions";
  import { toast, ToastType } from "$stores/ToastStore"

  let callData: {
    function: Functions | null,
    fields: Record<string, any>
  } = {
    function: null,
    fields: {}
  };

  function handleInput(callFunction: Functions, fieldName: string, event: Event) {
    const target = event.target as HTMLInputElement;
    let value: string | number | boolean | [] | null   = target.value;

    if (target.type === 'number') {
      value = target.value ? Number(target.value) : null;
    } else if (target.type === 'checkbox') {
      value = target.checked;
    }

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
    for (const fieldName of Object.keys(callData.fields)) {
      const field = administrativeFunctions
        .find((fn) => fn.function === callData.function)?.fields
        .find((f) => f.name === fieldName);

      if (field && field.type === 'array') {
        try {
          const value = callData.fields[fieldName];

          const parsedValue = JSON.parse(value);

          if (Array.isArray(parsedValue)) {
            callData.fields[fieldName] = parsedValue;
          } else {
            toast.set({
              type: ToastType.Error,
              description: `Error with array format on ${fieldName}.`
            });
            return;
          }
        } catch (error) {
          console.error("Error parsing array field", fieldName, error);
          toast.set({
            type: ToastType.Error,
            description: `Error parsing array on ${fieldName}. ${error}`
          });
          callData.fields[fieldName] = [];
          return;
        }
      }
    }

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
        fields: selectedCall.fields.reduce((acc: Record<string, any>, field: { name: string, type?: string }) => {
          // Initialize boolean checkboxes as false by default, others as empty strings
          acc[field.name] = field.type === 'checkbox' ? false : '';
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
                type={field.type ?? 'text'}
                value={callData.fields[field.name]}
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
