export interface Adaptor {
  name: string
  address: string
  calls: AdaptorCall[]
}

export interface AdaptorCall {
  function: string
  action: string
  fields: Field[]

}
export interface Field {
  name: string
  label: string
  placeholder: string

}
const adaptors: Adaptor[] = [
  {
    name: "AaveV3ATokenV1",
    address: "0x1111111111111111111111111111111111111111",
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
      {
        function: "AdjustIsolationModeAssetAsCollateral",
        action: "Adjust",
        fields: [
          {
            name: "asset",
            label: "ERC-20 Token Contract Address",
            placeholder: "0xtoken"
          },
          {
            name: "use_as_collateral",
            label: "Use as Collateral",
            placeholder: "boolean"
          }
        ]
      },
      {
        function: "ChangeEMode",
        action: "Change",
        fields: [
          {
            name: "category_id",
            label: "Category ID",
            placeholder: "ID"
          }
          
        ]
      },
    ]
  },
  {
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
  },
  {
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
  },
  {
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
]

export default adaptors;