import type { Adaptor } from "$lib/adaptorList"

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
      action: "Repay",
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
    }
  ],
}
export default AaveDebtTokenV2;