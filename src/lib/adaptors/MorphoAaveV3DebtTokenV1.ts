import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Underlying Asset"
        },
        {
          name: "amount_to_borrow",
          label: "Amount to Borrow",
          placeholder: "Amount to Borrow"
        },
        {
          name: "max_iterations",
          label: "Max Iterations",
          placeholder: "Max Iterations"
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
          placeholder: "Token to Repay"
        },
        {
          name: "amount_to_repay",
          label: "Amount to Repay",
          placeholder: "Amount to Repay"
        }
      ]
    }
  ]
}

export default MorphoAaveV3DebtTokenV1;
