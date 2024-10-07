import { type Adaptor, PlaceHolder } from "$lib/type"

const MorphoAaveV3DebtTokenV1: Adaptor = {
  name: "MorphoAaveV3DebtTokenV1",
  address: "",
  calls: [
    {
      function: "BorrowFromAaveV3Morpho",
      action: "Borrow",
      fields: [
        {
          name: "underlying",
          label: "Underlying Asset",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_borrow",
          label: "Amount to Borrow",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "max_iterations",
          label: "Max Iterations",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "RepayAaveV3MorphoDebt",
      action: "Repay",
      fields: [
        {
          name: "token_to_repay",
          label: "Token to Repay",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_repay",
          label: "Amount to Repay",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoAaveV3DebtTokenV1;
