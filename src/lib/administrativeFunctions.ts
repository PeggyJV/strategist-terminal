import { type CellarCallInputs, Functions } from "$lib/type"


const SetSharePriceOracle: CellarCallInputs = {
  function: Functions.SetSharePriceOracle,
  action: "Set",
  fields: [
    {
      name: "registry_id",
      label: "registry_id",
      placeholder: "registry_id"
    },
    {
      name: "share_price_oracle",
      label: "share_price_oracle",
      placeholder: "share_price_oracle"
    }
  ]
}

const AddPosition: CellarCallInputs = {
  function: Functions.AddPosition,
  action: "Add",
  fields: [
    {
      name: "index",
      label: "index",
      placeholder: "index",
      type: "number"
    },
    {
      name: "position_id",
      label: "position_id",
      placeholder: "position_id",
      type: "number"
    },
    {
      name: "configuration_data",
      label: "configuration_data",
      placeholder: "configuration_data",
      type: "array"
    },
    {
      name: "in_debt_array",
      label: "in_debt_array",
      placeholder: "in_debt_array",
      type: "checkbox"
    }
  ]
}

const RemovePosition: CellarCallInputs = {
  function: Functions.RemovePosition,
  action: "Remove",
  fields: [
    {
      name: "index",
      label: "index",
      placeholder: "index"
    },
    {
      name: "in_debt_array",
      label: "in_debt_array",
      placeholder: "in_debt_array"
    },
  ]
}

const SetHoldingPosition: CellarCallInputs = {
  function: Functions.SetHoldingPosition,
  action: "Set",
  fields: [
    {
      name: "position_id",
      label: "position_id",
      placeholder: "position_id"
    }
  ]
}

const SetStrategistPayoutAddress: CellarCallInputs = {
  function: Functions.SetStrategistPayoutAddress,
  action: "Set",
  fields: [
    {
      name: "payout",
      label: "payout",
      placeholder: "payout"
    }
  ]
}

const SwapPositions: CellarCallInputs = {
  function: Functions.SwapPositions,
  action: "Swap",
  fields: [
    {
      name: "index_1",
      label: "index_1",
      placeholder: "index_1"
    },
    {
      name: "index_2",
      label: "index_2",
      placeholder: "index_2"
    },
    {
      name: "in_debt_array",
      label: "in_debt_array",
      placeholder: "in_debt_array"
    },
  ]
}

const SetShareLockPeriod: CellarCallInputs = {
  function: Functions.SetShareLockPeriod,
  action: "Set",
  fields: [
    {
      name: "new_lock",
      label: "new_lock",
      placeholder: "new_lock"
    }
  ]
}

const InitiateShutdown: CellarCallInputs = {
  function: Functions.InitiateShutdown,
  action: "Initiate",
  fields: []
}

const LiftShutdown: CellarCallInputs = {
  function: Functions.LiftShutdown,
  action: "Lift",
  fields: []
}

const RemoveAdaptorFromCatalogue: CellarCallInputs = {
  function: Functions.RemoveAdaptorFromCatalogue,
  action: "Remove",
  fields: [
    {
      name: "adaptor",
      label: "adaptor",
      placeholder: "adaptor"
    }
  ]
}

const RemovePositionFromCatalogue: CellarCallInputs = {
  function: Functions.RemovePositionFromCatalogue,
  action: "Remove",
  fields: [
    {
      name: "position_id",
      label: "position_id",
      placeholder: "position_id"
    }
  ]
}

const DecreaseShareSupplyCap: CellarCallInputs = {
  function: Functions.DecreaseShareSupplyCap,
  action: "Decrease",
  fields: [
    {
      name: "new_cap",
      label: "new_cap",
      placeholder: "new_cap"
    }
  ]
}

const SetAlternativeAssetData: CellarCallInputs = {
  function: Functions.SetAlternativeAssetData,
  action: "Set",
  fields: [
    {
      name: "alternative_asset",
      label: "alternative_asset",
      placeholder: "alternative_asset"
    },
    {
      name: "alternative_holding_position",
      label: "alternative_holding_position",
      placeholder: "alternative_holding_position"
    },
    {
      name: "alternative_asset_fee",
      label: "alternative_asset_fee",
      placeholder: "alternative_asset_fee"
    },
  ]
}

const DropAlternativeAssetData: CellarCallInputs = {
  function: Functions.DropAlternativeAssetData,
  action: "Drop",
  fields: [
    {
      name: "alternative_asset",
      label: "alternative_asset",
      placeholder: "alternative_asset"
    }
  ]
}

const AddAdaptorToCatalogue: CellarCallInputs = {
  function: Functions.AddAdaptorToCatalogue,
  action: "Add",
  fields: [
    {
      name: "adaptor",
      label: "adaptor",
      placeholder: "adaptor"
    }
  ]
}

const AddPositionToCatalogue: CellarCallInputs = {
  function: Functions.AddPositionToCatalogue,
  action: "Add",
  fields: [
    {
      name: "position_id",
      label: "position_id",
      placeholder: "position_id"
    }
  ]
}

const SetRebalanceDeviation: CellarCallInputs = {
  function: Functions.SetRebalanceDeviation,
  action: "Set",
  fields: [
    {
      name: "new_deviation",
      label: "new_deviation",
      placeholder: "new_deviation"
    }
  ]
}

const SetStrategistPlatformCut: CellarCallInputs = {
  function: Functions.SetStrategistPlatformCut,
  action: "Set",
  fields: [
    {
      name: "new_cut",
      label: "new_cut",
      placeholder: "new_cut"
    }
  ]
}

const IncreaseShareSupplyCap: CellarCallInputs = {
  function: Functions.IncreaseShareSupplyCap,
  action: "Increase",
  fields: [
    {
      name: "new_cap",
      label: "new_cap",
      placeholder: "new_cap"
    }
  ]
}

const CachePriceRouter: CellarCallInputs = {
  function: Functions.CachePriceRouter,
  action: "Cache",
  fields: [
    {
      name: "check_total_assets",
      label: "check_total_assets",
      placeholder: "check_total_assets"
    },
    {
      name: "allowable_range",
      label: "allowable_range",
      placeholder: "allowable_range"
    },
    {
      name: "expected_price_router",
      label: "expected_price_router",
      placeholder: "expected_price_router"
    }
  ]
}

export const administrativeFunctions : CellarCallInputs[] = [
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
