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
      label: "registry_id",
      placeholder: "index"
    },
    {
      name: "position_id",
      label: "position_id",
      placeholder: "position_id"
    },
    {
      name: "configuration_data",
      label: "configuration_data",
      placeholder: "configuration_data"
    },
    {
      name: "in_debt_array",
      label: "in_debt_array",
      placeholder: "in_debt_array"
    }
  ]
}

export const administrativeFunctions : CellarCallInputs[] = [
  SetSharePriceOracle,
  AddPosition
]
