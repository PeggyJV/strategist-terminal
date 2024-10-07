import { type Adaptor, PlaceHolder } from "$lib/type"

const DebtFTokenV1: Adaptor = {
  name: "DebtFTokenV1",
  address: "",
  calls: [
    {
      function: "BorrowFromFraxlend",
      action: "Borrow From Fraxlend",
      fields: [
        {
          name: "fraxlend_pair",
          label: "The address of the Frax Pair to borrow from",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_borrow",
          label: "The amount of the asset to borrow",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "RepayFraxlendDebt",
      action: "Repay Fraxlend Debt",
      fields: [
        {
          name: "fraxlend_pair",
          label: "The address of the Frax Pair to repay debt on",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "debt_token_repay_amount",
          label: "The amount of the debt token to repay",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "CallAddInterest",
      action: "Add Interest",
      fields: [
        {
          name: "fraxlend_pair",
          label: "The address of the Frax Pair to call addInterest on",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
}
export default DebtFTokenV1;