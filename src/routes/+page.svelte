<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Aavev3Deposit from "$lib/adaptors/aavev3Deposit.svelte";
  import Config from "$lib/config.svelte";
  import AaveV3AToken from "$lib/adaptors/AaveV3ATokenV1.svelte";
  import AaveV3DebtTokenV1 from "$lib/adaptors/AaveV3DebtTokenV1.svelte";
  import ScheduleRequest from "$lib/ScheduleRequest.svelte";
  import AaveATokenV1 from "$lib/adaptors/AaveATokenV1.svelte";
  import AaveDebtTokenV1 from "$lib/adaptors/AaveDebtTokenV1.svelte";
  import CompoundCTokenV2 from "$lib/adaptors/CompoundCTokenV2.svelte";
  import AaveATokenV2 from "$lib/adaptors/AaveATokenV2.svelte";
  import AaveDebtTokenV2 from "$lib/adaptors/AaveDebtTokenV2.svelte";
  import OneInchV1 from "$lib/adaptors/OneInchV1.svelte";
  import FeesAndReservesV1 from "$lib/adaptors/FeesAndReservesV1.svelte";
  import ZeroXV1 from "$lib/adaptors/ZeroXV1.svelte";
  import SwapWithUniswapV1 from "$lib/adaptors/SwapWithUniswapV1.svelte";
  import VestingSimpleV2 from "$lib/adaptors/VestingSimpleV2.svelte";
  import UniswapV3V2 from "$lib/adaptors/UniswapV3V2.svelte";
  import AaveV2EnableAssetAsCollateralV1 from "$lib/adaptors/AaveV2EnableAssetAsCollateralV1.svelte";
  import FTokenV1 from "$lib/adaptors/FTokenV1.svelte";
  import MorphoAaveV2ATokenV1 from "$lib/adaptors/MorphoAaveV2ATokenV1.svelte";
  import MorphoAaveV2DebtTokenV1 from "$lib/adaptors/MorphoAaveV2DebtTokenV1.svelte";
  import MorphoAaveV3ATokenCollateralV1 from "$lib/adaptors/MorphoAaveV3ATokenCollateralV1.svelte";
  import MorphoAaveV3ATokenP2pV1 from "$lib/adaptors/MorphoAaveV3ATokenP2pV1.svelte";
  import MorphoAaveV3DebtTokenV1 from "$lib/adaptors/MorphoAaveV3DebtTokenV1.svelte";
  import BalancerPoolV1 from "$lib/adaptors/BalancerPoolV1.svelte";
  import LegacyCellarV1 from "$lib/adaptors/LegacyCellarV1.svelte";
  import DebtFTokenV1 from "$lib/adaptors/DebtFTokenV1.svelte";
  import CollateralFTokenV1 from "$lib/adaptors/CollateralFTokenV1.svelte";
  import ConvexCurveV1 from "$lib/adaptors/ConvexCurveV1.svelte";
  import CurveV1 from "$lib/adaptors/CurveV1.svelte";
  import AuraErc4626V1 from "$lib/adaptors/AuraErc4626V1.svelte";
  import MorphoBlueCollateralV1 from "$lib/adaptors/MorphoBlueCollateralV1.svelte";
  import MorphoBlueDebtV1 from "$lib/adaptors/MorphoBlueDebtV1.svelte";
  import MorphoBlueSupplyV1 from "$lib/adaptors/MorphoBlueSupplyV1.svelte";
  import Erc4626V1 from "$lib/adaptors/Erc4626V1.svelte";
  import StakingV1 from "$lib/adaptors/StakingV1.svelte";
  import Queue from "$lib/Queue.svelte";
  import { queue } from "$stores/AdapterQueue";

  let version = "";

  let config = true;
  let aavev3 = true;
  let aavev3a = true;
  let aavev3debt = true;

  let cellar_id = "";
  let block_height = "";
  let chain_id = "";
  let deadline = "";

  const map: { [key: string]: ConstructorOfATypedSvelteComponent } = {
    "Aavev3Deposit": Aavev3Deposit,
    "AaveV3AToken": AaveV3AToken,
    "AaveV3DebtTokenV1": AaveV3DebtTokenV1,
    "AaveATokenV1": AaveATokenV1,
    "AaveDebtTokenV1": AaveDebtTokenV1,
    "CompoundCTokenV2": CompoundCTokenV2,
    "AaveATokenV2": AaveATokenV2,
    "AaveDebtTokenV2": AaveDebtTokenV2,
    "OneInchV1": OneInchV1,
    "FeesAndReservesV1": FeesAndReservesV1,
    "ZeroXV1": ZeroXV1,
    "SwapWithUniswapV1": SwapWithUniswapV1,
    "VestingSimpleV2": VestingSimpleV2,
    "UniswapV3V2": UniswapV3V2,
    "AaveV2EnableAssetAsCollateralV1": AaveV2EnableAssetAsCollateralV1,
    "FTokenV1": FTokenV1,
    "MorphoAaveV2ATokenV1": MorphoAaveV2ATokenV1,
    "MorphoAaveV2DebtTokenV1": MorphoAaveV2DebtTokenV1,
    "MorphoAaveV3ATokenCollateralV1": MorphoAaveV3ATokenCollateralV1,
    "MorphoAaveV3ATokenP2pV1": MorphoAaveV3ATokenP2pV1,
    "MorphoAaveV3DebtTokenV1": MorphoAaveV3DebtTokenV1,
    "BalancerPoolV1": BalancerPoolV1,
    "LegacyCellarV1": LegacyCellarV1,
    "DebtFTokenV1": DebtFTokenV1,
    "CollateralFTokenV1": CollateralFTokenV1,
    "ConvexCurveV1": ConvexCurveV1,
    "CurveV1": CurveV1,
    "AuraErc4626V1": AuraErc4626V1,
    "MorphoBlueCollateralV1": MorphoBlueCollateralV1,
    "MorphoBlueDebtV1": MorphoBlueDebtV1,
    "MorphoBlueSupplyV1": MorphoBlueSupplyV1,
    "Erc4626V1": Erc4626V1,
    "StakingV1": StakingV1
  };

  let displayedAdaptor = Object.values(map)[0];
  let activeButton = Object.keys(map)[0];

  async function status() {
    version = await invoke("version", {});
  }

  function selectAdaptor(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    activeButton = target.innerText;
    displayedAdaptor = map[target.innerText];
  }
</script>

<div class="min-h-screen flex flex-col justify-center items-center bg-gray-100">

  <h1 class="mb-4 text-3xl font-bold">Welcome to Strategist Terminal</h1>

  <div class="flex mt-8 space-x-12">
    {#if config}
      <div class="prose flex-1">
        <Config />
      </div>
    {/if}

    <div class="prose flex-1">
      <ScheduleRequest />
    </div>

    {#if $queue.length > 0}
        <Queue />
    {/if}
  </div>

  <div class="flex flex-wrap gap-2.5 mt-12 mx-20">
    {#each Object.keys(map) as item}
      <button
        on:click={selectAdaptor}
        class="p-2.5 border rounded focus:outline-none {item === activeButton ? 'bg-blue-500 text-white border-blue-500' : 'bg-gray-100 text-black border-gray-300'}"
      >{item}</button>
    {/each}
  </div>

  <svelte:component this={displayedAdaptor} />

  <!-- Additional text information at the bottom of the page -->
  <div class="mt-8 prose">
    <h2>Specific Strategist Information</h2>
    <p>
      A. Keep in mind regarding isolated markets. If they are isolated, then
      they may not be usable as collateral for other borrow positions.
    </p>
    <p>
      B. See the <a href="https://sommelier-finance.gitbook.io/sommelier-documentation/smart-contracts/protocol-v2-contract-architecture">docs</a> for more information pertaining to this adaptor.
    </p>
    <!-- Empty lines of spaces -->
    <br /><br /><br />
  </div>

</div>
