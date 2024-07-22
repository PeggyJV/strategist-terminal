import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Address"
        },
        {
          name: "collateral_to_deposit",
          label: "The amount of collateral to add to the cellar position",
          placeholder: "Amount"
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
          placeholder: "Amount"
        },
        {
          name: "fraxlend_pair",
          label: "The FraxLend pair to remove collateral from",
          placeholder: "Address"
        }
      ]
    }
  ]
}
export default CollateralFTokenV1;