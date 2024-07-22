import type { Adaptor } from "$lib/adaptorList"

const MorphoAaveV3ATokenCollateralV1: Adaptor = {
  name: "MorphoAaveV3ATokenCollateralV1",
  address: "",
  calls: [
    {
      function: "DepositToAaveV3Morpho",
      action: "Deposit",
      fields: [
        {
          name: "token_to_deposit",
          label: "ERC-20 Token Contract Address",
          placeholder: "0xTokenAddress"
        },
        {
          name: "amount_to_deposit",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "WithdrawFromAaveV3Morpho",
      action: "Withdraw",
      fields: [
        {
          name: "token_to_withdraw",
          label: "ERC-20 Token Contract Address",
          placeholder: "0xTokenAddress"
        },
        {
          name: "amount_to_withdraw",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default MorphoAaveV3ATokenCollateralV1;
