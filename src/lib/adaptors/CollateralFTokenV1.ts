import { type Adaptor, PlaceHolder } from "$lib/type"

const CollateralFTokenV1: Adaptor = {
  name: "CollateralFTokenV1",
  address: "",
  calls: [
    {
      function: "AddCollateral",
      action: "Deposit",
      fields: [
        {
          name: "fraxlend_pair",
          label: "The FraxLend pair to add collateral to",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "collateral_to_deposit",
          label: "The amount of collateral to add to the cellar position",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "RemoveCollateral",
      action: "Deposit",
      fields: [
        {
          name: "collateral_amount",
          label: "The amount of collateral to remove from the cellar position",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "fraxlend_pair",
          label: "The FraxLend pair to remove collateral from",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
}
export default CollateralFTokenV1;