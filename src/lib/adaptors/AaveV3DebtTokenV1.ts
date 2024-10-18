import { type Adaptor, PlaceHolder } from "$lib/type"

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
      function: "RepayAaveDebt",
      action: "Repay Dept with Underlying",
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
      function: "RepayWithATokens",
      action: "Repay Debt with AToken",
      fields: [
        {
          name: "underlying_token",
          label: "ERC-20 Token Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount of AToken to Repay",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}
export default AaveV3DebtTokenV1;