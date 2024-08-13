<script context="module" lang="ts">

  export interface Request {
    id: string,
    state: string,
    votingPower: string,
    currentHeight: string,
    scheduledHeight: string,
    corkResult: string,
  }

</script>
<script lang="ts">
  import StateModal from "../StateModal.svelte"


  const requests: Request[] = [
    {
      id: "111",
      state: "Relayed",
      votingPower: "xxx",
      currentHeight: "xxx",
      scheduledHeight: "xxx",
      corkResult: "xxx",
    },
    {
      id: "222",
      state: "Scheduled",
      votingPower: "xxx",
      currentHeight: "xxx",
      scheduledHeight: "xxx",
      corkResult: "xxx",
    }
  ]

  let modalVisible = false;
  let activeRequest = requests[0];

  function toggleModal(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;

    activeRequest = requests.find(
        (a: Request) => a.id ===  target.innerText
      )
      ?? requests[0]

    modalVisible = !modalVisible;
  }

</script>
<div class="prose w-screen">

  <div class="flex justify-center">
    <h1>Requests</h1>
  </div>

  <table class="mx-auto text-center">
    <thead>
    <tr>
      <th>ID</th>
      <th>State</th>
      <th>Current Height</th>
    </tr>
    </thead>
    <tbody>
    {#each requests as request}
      <tr>
        <th>
          <button on:click={toggleModal}>
            {request.id}
          </button>
        </th>
        <td>{request.state}</td>
        <td>{request.currentHeight}</td>
      </tr>
    {/each}
    {#if requests.length === 0}
      <tr>
        <td>No requests to show!</td>
      </tr>
    {/if}
    </tbody>
  </table>

  {#if modalVisible}
    <StateModal {toggleModal} request={activeRequest} />
  {/if}
</div>
