import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_borrow",
          label: "Amount of Asset to Borrow",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_repay",
          label: "Amount of Asset to Repay",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoAaveV2DebtTokenV1;
