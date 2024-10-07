import { type Adaptor, PlaceHolder } from "$lib/type"

const Erc4626V1V1: Adaptor = {
  name: "Erc4626V1V1",
  address: "",
  calls: [
    {
      function: "DepositToVault",
      action: "Deposit",
      fields: [
        {
          name: "erc4626_vault",
          label: "The address of the ERC4626 vault",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "The amount of assets to deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawFromVault",
      action: "Withdraw",
      fields: [
        {
          name: "erc4626_vault",
          label: "The address of the ERC4626 vault",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "The amount of assets to deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}
export default Erc4626V1V1;