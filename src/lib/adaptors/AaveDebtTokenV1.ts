import type { Adaptor } from "$lib/adaptorList"

const AaveDebtTokenV1: Adaptor = {
  name: "AaveDebtTokenV1",
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
    },
    {
      function: "SwapAndRepay",
      action: "Swap And Repay",
      fields: [
        {
          name: "token_in",
          label: "The address of the token to swap from",
          placeholder: "0xtoken"
        },
        {
          name: "token_to_repay",
          label: "The address of the token to swap to and repay with",
          placeholder: "0xtoken"
        },
        {
          name: "amount",
          label: "The amount to swap",
          placeholder: "Amount"
        },
        {
          name: "exchange",
          label: "The exchange to make the swap on",
          placeholder: "Exchange"
        }
      ]
    },
  ]
}

export default AaveDebtTokenV1;