import type { Adaptor } from "$lib/adaptorList"

const MorphoBlueDebtV1: Adaptor = {
  name: "MorphoBlueDebtV1",
  address: "",
  calls: [
    {
      function: "BorrowFromMorphoBlue",
      action: "Borrow",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: "Market"
        },
        {
          name: "amount_to_borrow",
          label: "Amount of Debt Token to Borrow",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "RepayMorphoBlueDebt",
      action: "Repay",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: "Market"
        },
        {
          name: "debt_token_repay_amount",
          label: "Amount of Debt Token to Repay",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default MorphoBlueDebtV1;
