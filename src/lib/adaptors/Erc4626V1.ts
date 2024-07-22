import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Address"
        },
        {
          name: "assets",
          label: "The amount of assets to deposit",
          placeholder: "Amount"
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
          placeholder: "Address"
        },
        {
          name: "assets",
          label: "The amount of assets to deposit",
          placeholder: "Amount"
        }
      ]
    }
  ]
}
export default Erc4626V1V1;