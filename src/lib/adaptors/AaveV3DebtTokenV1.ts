import type { Adaptor } from "$lib/adaptorList"

const AaveV3DebtTokenV1: Adaptor = {
  name: "AaveV3DebtTokenV1",
  address: "0x1111111111111111111111111111111111111111",
  calls: [
    {
      function: "BorrowFromAave",
      action: "Borrow",
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
      function: "RepayAaveDebt",
      action: "Repay Dept with Underlying",
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
      function: "RepayWithATokens",
      action: "Repay Debt with AToken",
      fields: [
        {
          name: "underlying_token",
          label: "ERC-20 Token Contract Address",
          placeholder: "0xtoken"
        },
        {
          name: "amount",
          label: "Amount of AToken to Repay",
          placeholder: "Amount"
        }
      ]
    }
  ]
}
export default AaveV3DebtTokenV1;