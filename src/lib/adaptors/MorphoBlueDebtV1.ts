import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_borrow",
          label: "Amount of Debt Token to Borrow",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "debt_token_repay_amount",
          label: "Amount of Debt Token to Repay",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoBlueDebtV1;
