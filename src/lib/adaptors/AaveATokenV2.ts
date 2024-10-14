import { type Adaptor, PlaceHolder } from "$lib/type"

const AaveATokenV2: Adaptor = {
  name: "AaveATokenV2",
  address: "",
  calls: [
    {
      function: "DepositToAave",
      action: "Deposit",
      fields: [
        {
          name: "token",
          label: "ERC-20 Token Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawFromAave",
      action: "Withdraw",
      fields: [
        {
          name: "token",
          label: "ERC-20 Token Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
  ]
}

export default AaveATokenV2;