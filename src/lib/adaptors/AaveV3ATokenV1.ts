import type { Adaptor } from "$lib/adaptorList"

const AaveV3ATokenV1: Adaptor = {
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
}
export default AaveV3ATokenV1;