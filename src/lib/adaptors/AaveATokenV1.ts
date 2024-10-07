import type { Adaptor } from "$lib/type"

const AaveATokenV1: Adaptor = {
  name: "AaveATokenV1",
  address: "",
  calls: [
    {
      function: "DepositToAave",
      action: "Deposit",
      fields: [
        {
          name: "token",
          label: "ERC-20 Token Contract Address",
          placeholder: "0xtoken",
          type: "text"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount",
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
          placeholder: "0xtoken",
          type: "text"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount",
          type: "text"
        }
      ]
    },
  ]
}
export default AaveATokenV1;