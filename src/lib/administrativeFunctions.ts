import { type CellarFunction, Functions } from "$lib/type"

const AddPosition: CellarFunction = {
  function: Functions.AddPosition,
  action: "Add",
  info: "Insert a trusted position to the list of positions used by the cellar at a given index. " +
    "Represents function addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)",
  fields: [
    {
      name: "index",
      label: "Index at which to add the position",
      placeholder: "e.g 1",
      type: "number"
    },
    {
      name: "position_id",
      label: "The position's ID in the cellar registry",
      placeholder: "e.g 1",
      type: "number"
    },
    {
      name: "configuration_data",
      label: "Data used to configure how the position behaves",
      placeholder: "e.g [1,2,3]",
      type: "array"
    },
    {
      name: "in_debt_array",
      label: "Whether to add position in the debt array, or the credit array",
      placeholder: "",
      type: "checkbox"
    }
  ]
}

const RemovePosition: CellarFunction = {
  function: Functions.RemovePosition,
  action: "Remove",
  info: "Remove the position at a given index from the list of positions used by the cellar. " +
    "Represents function removePosition(uint32 index, bool inDebtArray)",
  fields: [
    {
      name: "index",
      label: "Index at which to remove the position",
      placeholder: "e.g 1",
      type: "number"
    },
    {
      name: "in_debt_array",
      label: "Whether to add position in the debt array, or the credit array",
      placeholder: "",
      type: "checkbox"
    },
  ]
}

const SetHoldingPosition: CellarFunction = {
  function: Functions.SetHoldingPosition,
  action: "Set",
  info: "Set the holding position used of the cellar. " +
    "Represents function setHoldingIndex(uint8 index)",
  fields: [
    {
      name: "position_id",
      label: "ID (index) of the new holding position to use",
      placeholder: "e.g 1",
      type: "number"
    }
  ]
}

const SetStrategistPayoutAddress: CellarFunction = {
  function: Functions.SetStrategistPayoutAddress,
  action: "Set",
  info: "Sets the Strategists payout address. " +
    "Represents function setStrategistPayoutAddress(address payout)",
  fields: [
    {
      name: "payout",
      label: "Payout address",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    }
  ]
}

const SwapPositions: CellarFunction = {
  function: Functions.SwapPositions,
  action: "Swap",
  info: "Swap the positions at two given indeces. " +
    "Represents function swapPositions(uint32 index1, uint32 index2)",
  fields: [
    {
      name: "index_1",
      label: "Index of the first position",
      placeholder: "e.g 1",
      type: "number"
    },
    {
      name: "index_2",
      label: "Index of the second position",
      placeholder: "e.g 2",
      type: "number"
    },
    {
      name: "in_debt_array",
      label: "Whether to switch positions in the debt array, or the credit array",
      placeholder: "",
      type: "checkbox"
    },
  ]
}

const SetShareLockPeriod: CellarFunction = {
  function: Functions.SetShareLockPeriod,
  action: "Set",
  info: "Allows share lock period to be updated. " +
    "Represents function setShareLockPeriod()",
  fields: [
    {
      name: "new_lock",
      label: "New lock",
      placeholder: "",
      type: "text"
    }
  ]
}

const InitiateShutdown: CellarFunction = {
  function: Functions.InitiateShutdown,
  action: "Initiate",
  info: "Shutdown the cellar. Used in an emergency or if the cellar has been deprecated. " +
    "Represents function initiateShutdown()",
  fields: []
}

const LiftShutdown: CellarFunction = {
  function: Functions.LiftShutdown,
  action: "Lift",
  info: "Allows the owner to restart a shut down Cellar. " +
    "Represents function liftShutdown()",
  fields: []
}

const RemoveAdaptorFromCatalogue: CellarFunction = {
  function: Functions.RemoveAdaptorFromCatalogue,
  action: "Remove",
  fields: [
    {
      name: "adaptor",
      label: "Adaptor address",
      placeholder: "adaptor",
      type: "text"
    }
  ]
}

const RemovePositionFromCatalogue: CellarFunction = {
  function: Functions.RemovePositionFromCatalogue,
  action: "Remove",
  info: "Allows caller to remove positions from this cellar's catalogue. " +
    "Represents function removePositionFromCatalogue(uint32 positionId)",
  fields: [
    {
      name: "position_id",
      label: "Position ID",
      placeholder: "e.g 1",
      type: "number"
    }
  ]
}

const DecreaseShareSupplyCap: CellarFunction = {
  function: Functions.DecreaseShareSupplyCap,
  action: "Decrease",
  info: "Allows strategist to decrease the share supply cap. " +
    "Represents function decreaseShareSupplyCap(uint192)",
  fields: [
    {
      name: "new_cap",
      label: "New share supply cap",
      placeholder: "",
      type: "text"
    }
  ]
}

const SetAlternativeAssetData: CellarFunction = {
  function: Functions.SetAlternativeAssetData,
  action: "Set",
  info: "Allows the strategist to add, or update an existing alternative asset deposit. " +
    "Represents function setAlternativeAssetData(ERC20 _alternativeAsset, uint32 _alternativeHoldingPosition, uint32 _alternativeAssetFee)",
  fields: [
    {
      name: "alternative_asset",
      label: "The address of the alternative asset",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    },
    {
      name: "alternative_holding_position",
      label: "The holding position to direct alternative asset deposits to",
      placeholder: "",
      type: "number"
    },
    {
      name: "alternative_asset_fee",
      label: "The fee to charge for depositing this alternative asset",
      placeholder: "",
      type: "number"
    },
  ]
}

const DropAlternativeAssetData: CellarFunction = {
  function: Functions.DropAlternativeAssetData,
  action: "Drop",
  info: "Allows the strategist to stop an alternative asset from being deposited. " +
    "Represents function dropAlternativeAssetData(ERC20 _alternativeAsset)",
  fields: [
    {
      name: "alternative_asset",
      label: "The address of the alternative asset",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    }
  ]
}

const AddAdaptorToCatalogue: CellarFunction = {
  function: Functions.AddAdaptorToCatalogue,
  action: "Add",
  info: "Allows the owner to add an adaptor to the Cellar's adaptor catalogue. " +
    "Represents function addAdaptorToCatalogue(address adaptor)",
  fields: [
    {
      name: "adaptor",
      label: "Adaptor address",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    }
  ]
}

const AddPositionToCatalogue: CellarFunction = {
  function: Functions.AddPositionToCatalogue,
  action: "Add",
  info: "Allows the owner to add a position to the Cellar's position catalogue. " +
    "Represents function addPositionToCatalogue(uint32 positionId)",
  fields: [
    {
      name: "position_id",
      label: "Position ID",
      placeholder: "e.g 1",
      type: "number"
    }
  ]
}

const SetRebalanceDeviation: CellarFunction = {
  function: Functions.SetRebalanceDeviation,
  action: "Set",
  info: "Changes the cellar's allowed rebalance deviation, " +
    "which is the percent the total assets of a cellar may deviate during a callOnAdaptor(rebalance) call. " +
    "The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%. " +
    "Represents function setRebalanceDeviation(uint256)",
  fields: [
    {
      name: "new_deviation",
      label: "New deviation",
      placeholder: "",
      type: "text"
    }
  ]
}

const SetStrategistPlatformCut: CellarFunction = {
  function: Functions.SetStrategistPlatformCut,
  action: "Set",
  info: "Allows strategist to set the platform cut for the cellar. " +
    "Represents function setStrategistPlatformCut(uint64 cut)",
  fields: [
    {
      name: "new_cut",
      label: "The new strategist platform cut",
      placeholder: "",
      type: "number"
    }
  ]
}

const IncreaseShareSupplyCap: CellarFunction = {
  function: Functions.IncreaseShareSupplyCap,
  action: "Increase",
  info: "Allows the caller to increase the share supply cap. " +
    "Represents function increaseShareSupplyCap(uint192 _newShareSupplyCap)",
  fields: [
    {
      name: "new_cap",
      label: "New cap",
      placeholder: "",
      type: "text"
    }
  ]
}

const SetSharePriceOracle: CellarFunction = {
  function: Functions.SetSharePriceOracle,
  action: "Set",
  info: "Allows the caller to set the share price oracle contract. " +
    "Represents function setSharePriceOracle(uint256 _registryId, ERC4626SharePriceOracle _sharePriceOracle)",
  fields: [
    {
      name: "registry_id",
      label: "Oracles registry ID",
      placeholder: "e.g 1",
      type: "text"
    },
    {
      name: "share_price_oracle",
      label: "Oracles contract address",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    }
  ]
}

const CachePriceRouter: CellarFunction = {
  function: Functions.CachePriceRouter,
  action: "Cache",
  info: "Updates the cellar to use the latest price router in the registry. " +
    "Represents function cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)",
  fields: [
    {
      name: "check_total_assets",
      label: "Whether to check the total assets of the cellar",
      placeholder: "",
      type: "checkbox"
    },
    {
      name: "allowable_range",
      label: "The allowable range of the cellar's total assets to deviate between old and new routers",
      placeholder: "",
      type: "number"
    },
    {
      name: "expected_price_router",
      label: "The expected price router address",
      placeholder: "e.g 0x1111111111111111111111111111111111111111",
      type: "text"
    }
  ]
}

export const administrativeFunctions : CellarFunction[] = [
  AddPosition,
  RemovePosition,
  SetHoldingPosition,
  SetStrategistPayoutAddress,
  SwapPositions,
  SetShareLockPeriod,
  InitiateShutdown,
  LiftShutdown,
  RemoveAdaptorFromCatalogue,
  RemovePositionFromCatalogue,
  DecreaseShareSupplyCap,
  SetAlternativeAssetData,
  DropAlternativeAssetData,
  AddAdaptorToCatalogue,
  AddPositionToCatalogue,
  SetRebalanceDeviation,
  SetStrategistPlatformCut,
  SetSharePriceOracle,
  IncreaseShareSupplyCap,
  CachePriceRouter
]
