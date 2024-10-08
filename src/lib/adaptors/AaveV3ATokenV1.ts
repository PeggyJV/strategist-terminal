import { type Adaptor, PlaceHolder } from "$lib/type"

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
      function: "WithdrawFromAave",
      action: "Withdraw",
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
      function: "AdjustIsolationModeAssetAsCollateral",
      action: "Adjust",
      fields: [
        {
          name: "asset",
          label: "ERC-20 Token Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "use_as_collateral",
          label: "Use as Collateral",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        }
      ]
    },
    {
      function: "ChangeEmode",
      action: "Change",
      fields: [
        {
          name: "category_id",
          label: "Category ID",
          placeholder: PlaceHolder.Empty,
          type: "number"
        }

      ]
    },
  ]
}
export default AaveV3ATokenV1;