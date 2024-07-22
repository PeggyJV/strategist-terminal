import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "0xtoken"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount"
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
          placeholder: "0xtoken"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount"
        }
      ]
    },
  ]
}

export default AaveATokenV2;