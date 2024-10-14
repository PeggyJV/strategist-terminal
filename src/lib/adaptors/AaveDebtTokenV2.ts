import { type Adaptor, PlaceHolder } from "$lib/type"

const AaveDebtTokenV2: Adaptor = {
  name: "AaveDebtTokenV2",
  address: "",
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
      action: "Repay",
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
    }
  ],
}
export default AaveDebtTokenV2;