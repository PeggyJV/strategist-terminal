import { type Adaptor, PlaceHolder } from "$lib/type"

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
      function: "RepayAaveDebt",
      action: "Repay",
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
      function: "SwapAndRepay",
      action: "Swap And Repay",
      fields: [
        {
          name: "token_in",
          label: "The address of the token to swap from",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_to_repay",
          label: "The address of the token to swap to and repay with",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_in",
          label: "The amount to swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "exchange",
          label: "The exchange to make the swap on",
          placeholder: PlaceHolder.Empty,
          type: "number"
        },
        {
          name: "params",
          label: "The parameters for the swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
  ]
}

export default AaveDebtTokenV1;