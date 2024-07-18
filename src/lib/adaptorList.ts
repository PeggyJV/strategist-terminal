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
  }
]

export default adaptors;