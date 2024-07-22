import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Address"
        },
        {
          name: "amount_to_borrow",
          label: "The amount of the asset to borrow",
          placeholder: "Amount"
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
          placeholder: "Address"
        },
        {
          name: "debt_token_repay_amount",
          label: "The amount of the debt token to repay",
          placeholder: "Amount"
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
          placeholder: "Address"
        }
      ]
    }
  ]
}
export default DebtFTokenV1;