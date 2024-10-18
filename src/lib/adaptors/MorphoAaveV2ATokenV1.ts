import { type Adaptor, PlaceHolder } from "$lib/type"

const MorphoAaveV2ATokenV1: Adaptor = {
  name: "MorphoAaveV2ATokenV1",
  address: "",
  calls: [
    {
      function: "DepositToAaveV2Morpho",
      action: "Deposit",
      fields: [
        {
          name: "a_token",
          label: "Aave V2 aToken Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_deposit",
          label: "Amount of Asset to Deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawFromAaveV2Morpho",
      action: "Withdraw",
      fields: [
        {
          name: "a_token",
          label: "Aave V2 aToken Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_withdraw",
          label: "Amount of Asset to Withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoAaveV2ATokenV1;
