import type { Adaptor } from "$lib/adaptorList"

const MorphoAaveV2DebtTokenV1: Adaptor = {
  name: "MorphoAaveV2DebtTokenV1",
  address: "",
  calls: [
    {
      function: "BorrowFromAaveV2Morpho",
      action: "Borrow",
      fields: [
        {
          name: "a_token",
          label: "Aave V2 aToken Contract Address",
          placeholder: "0xaTokenAddress"
        },
        {
          name: "amount_to_borrow",
          label: "Amount of Asset to Borrow",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "RepayAaveV2MorphoDebt",
      action: "Repay",
      fields: [
        {
          name: "a_token",
          label: "Aave V2 aToken Contract Address",
          placeholder: "0xaTokenAddress"
        },
        {
          name: "amount_to_repay",
          label: "Amount of Asset to Repay",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default MorphoAaveV2DebtTokenV1;
