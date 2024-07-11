<script lang="ts">
  import { queue, CellarCall } from "$stores/AdapterQueue";

  let performance_fee_5: string = "";
  let performance_fee_1: string = "";
  let management_fee_5: string = "";
  let management_fee_2: string = "";
  let new_frequency: string = "";
  let new_max_gas: string = "";
  let amount_6 = "";
  let amount_7 = "";
  let amount_8 = "";

  const FeesAndReservesV1Address = "";

  function scheduleUpdatePerformanceFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          UpdatePerformanceFees: {
            performance_fee_5: performance_fee_1,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleUpdateManagementFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          UpdateManagementFees: {
            management_fee_5: management_fee_2,
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleChangeUpkeepFrequency() {
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

  function scheduleChangeUpkeepMaxGas() {
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

  function scheduleSetupMetaData() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          SetupMetaData: {
            management_fee_5,
            performance_fee_5
          },
        }),
      );
      return callQueue;
    });
  }

  function scheduleAddAssetsToReserves() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          AddAssetsToReserves: {
            amount: amount_6
           },
        }),
      );
      return callQueue;
    });
  }

  function scheduleWithdrawAssetsFromReserves() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          WithdrawAssetsFromReserves: {
            amount: amount_7
          },
        }),
      );
      return callQueue;
    });
  }

  function schedulePrepareFees() {
    queue.update((callQueue) => {
      callQueue.push(
        new CellarCall(FeesAndReservesV1Address, "FeesAndReservesV1", {
          PrepareFees: {
            amount: amount_8
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
    for="performance_fee_1"
    title="Enter Cellar's new performance fee."
  >New performance fee:</label
  >
  <input type="text" id="performance_fee_1" bind:value={performance_fee_1} placeholder="Amount" />
</div>

<button on:click={scheduleUpdatePerformanceFees}>Update</button>

<!-- 2. UpdateManagementFees -->

<h1>2. UpdateManagementFees</h1>
<div>
  <label
    for="management_fee_2"
    title="Enter Cellar's new management fee."
  >New management fee:</label
  >
  <input type="text" id="management_fee_2" bind:value={management_fee_2} placeholder="Amount" />
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
  <input type="text" id="new_frequency" bind:value={new_frequency} placeholder="Amount" />
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
  <input type="text" id="new_max_gas" bind:value={new_max_gas} placeholder="Amount" />
</div>

<button on:click={scheduleChangeUpkeepMaxGas}>Update</button>

<!--5. SetupMetaData-->

<h1>5. SetupMetaData</h1>
<div>
  <div>
    <label
      for="management_fee_5"
      title="Enter Cellar's new management fee."
    >Management fee:</label
    >
    <input type="text" id="management_fee_5" bind:value={management_fee_5} placeholder="Amount" />
  </div>

  <div>
    <label
      for="performance_fee_5"
      title="Enter Cellar's new performance fee."
    >Performance fee:</label
    >
    <input type="text" id="performance_fee_5" bind:value={performance_fee_5} placeholder="Amount" />
  </div>

</div>

<button on:click={scheduleSetupMetaData}>Update</button>

<h1>6. Add Assets To Reserves</h1>
<div>
  <div>
    <label
      for="amount_6"
      title="Amount"
    >Amount:</label
    >
    <input type="text" id="amount_6" bind:value={amount_6} placeholder="Amount" />
  </div>

</div>

<h1>7. Withdraw assets from the Cellar's reserves</h1>
<div>
  <div>
    <label
      for="amount_7"
      title="Amount"
    >Amount:</label
    >
    <input type="text" id="amount_7" bind:value={amount_7} placeholder="Amount" />
  </div>

</div>

<button on:click={scheduleWithdrawAssetsFromReserves}>Update</button>

<h1>8. Prepare fees to be split between the platform, strategist, and protocol</h1>
<div>
  <div>
    <label
      for="amount_8"
      title="Amount"
    >Amount:</label
    >
    <input type="text" id="amount_8" bind:value={amount_8} placeholder="Amount" />
  </div>

</div>

<button on:click={schedulePrepareFees}>Update</button>

