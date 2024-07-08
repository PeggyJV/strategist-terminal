<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  export let performance_fee: number | null = null;
  export let management_fee: number | null = null;
  export let new_frequency: number | null = null;
  export let new_max_gas: number | null = null;

  const FeesAndReservesV1Address = "";

  async function scheduleUpdatePerformanceFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          UpdatePerformanceFees: {
            performance_fee,
          },
        }),
      );
      return callQueue;
    });
  }

  async function scheduleUpdateManagementFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          UpdateManagementFees: {
            management_fee,
          },
        }),
      );
      return callQueue;
    });
  }

  async function scheduleChangeUpkeepFrequency() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          ChangeUpkeepFrequency: {
            new_frequency,
          },
        }),
      );
      return callQueue;
    });
  }

  async function scheduleChangeUpkeepMaxGas() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          ChangeUpkeepMaxGas: {
            new_max_gas,
          },
        }),
      );
      return callQueue;
    });
  }

  async function scheduleSetupMetaData() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          SetupMetaData: {
            management_fee,
            performance_fee
          },
        }),
      );
      return callQueue;
    });
  }

</script>

<!-- 1. UpdatePerformanceFees -->

<h1>1. UpdatePerformanceFees</h1>
<div>
  <label
    for="performance_fee"
    title="Enter Cellar's new performance fee."
  >New performance fee:</label
  >
  <input type="number" id="performance_fee" bind:value={performance_fee} placeholder="Amount" />
</div>

<button on:click={scheduleUpdatePerformanceFees}>Update</button>

<!-- 2. UpdateManagementFees -->

<h1>2. UpdateManagementFees</h1>
<div>
  <label
    for="management_fee"
    title="Enter Cellar's new management fee."
  >New management fee:</label
  >
  <input type="number" id="management_fee" bind:value={management_fee} placeholder="Amount" />
</div>

<button on:click={scheduleUpdateManagementFees}>Update</button>

<!--3. ChangeUpkeepFrequency-->

<h1>3. ChangeUpkeepFrequency</h1>
<div>
  <label
    for="new_frequency"
    title="Enter new frequency."
  >New frequency:</label
  >
  <input type="number" id="new_frequency" bind:value={new_frequency} placeholder="Amount" />
</div>

<button on:click={scheduleChangeUpkeepFrequency}>Update</button>

<!--4. ChangeUpkeepMaxGas-->

<h1>4. ChangeUpkeepMaxGas</h1>
<div>
  <label
    for="new_max_gas"
    title="Enter new max gas."
  >New max gas:</label
  >
  <input type="number" id="new_max_gas" bind:value={new_max_gas} placeholder="Amount" />
</div>

<button on:click={scheduleChangeUpkeepMaxGas}>Update</button>

<!--4. SetupMetaData-->

<h1>4. SetupMetaData</h1>
<div>
  <div>
    <label
      for="management_fee"
      title="Enter Cellar's new management fee."
    >New max gas:</label
    >
    <input type="number" id="management_fee" bind:value={management_fee} placeholder="Amount" />
  </div>

  <div>
    <label
      for="performance_fee"
      title="Enter Cellar's new performance fee."
    >New performance fee:</label
    >
    <input type="number" id="performance_fee" bind:value={performance_fee} placeholder="Amount" />
  </div>

</div>

<button on:click={scheduleChangeUpkeepMaxGas}>Update</button>

<!-- Additional text information at the bottom of the page -->
<div>
  <h2>Specific Strategist Information</h2>
  <p>
    A. Keep in mind regarding isolated markets. If they are isolated, then
    they may not be usable as collateral for other borrow positions.
  </p>
  <p>
    B. See the <a
    href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture"
  >docs</a
  > for more information pertaining to this adaptor.
  </p>
  <!-- Empty lines of spaces -->
  <br /><br /><br />
</div>
