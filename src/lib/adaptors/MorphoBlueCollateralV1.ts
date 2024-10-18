import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "collateral_to_deposit",
          label: "Amount of Collateral to Add",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "collateral_amount",
          label: "Amount of Collateral to Remove",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoBlueCollateralV1;
