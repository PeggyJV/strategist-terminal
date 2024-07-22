import type { Adaptor } from "$lib/adaptorList"

const MorphoBlueCollateralV1: Adaptor = {
  name: "MorphoBlueCollateralV1",
  address: "",
  calls: [
    {
      function: "AddCollateral",
      action: "Add",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: "Market"
        },
        {
          name: "collateral_to_deposit",
          label: "Amount of Collateral to Add",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "RemoveCollateral",
      action: "Remove",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: "Market"
        },
        {
          name: "collateral_amount",
          label: "Amount of Collateral to Remove",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default MorphoBlueCollateralV1;
