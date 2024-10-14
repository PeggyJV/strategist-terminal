import { type Adaptor, PlaceHolder } from "$lib/type"

const MorphoAaveV3ATokenP2pV1: Adaptor = {
  name: "MorphoAaveV3ATokenP2pV1",
  address: "",
  calls: [
    {
      function: "DepositToAaveV3Morpho",
      action: "Deposit",
      fields: [
        {
          name: "token_to_deposit",
          label: "ERC-20 Token Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_deposit",
          label: "Amount of ERC-20 Asset",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "max_iterations",
          label: "Max Iterations",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_withdraw",
          label: "Amount of ERC-20 Asset",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "max_iterations",
          label: "Max Iterations",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoAaveV3ATokenP2pV1;
