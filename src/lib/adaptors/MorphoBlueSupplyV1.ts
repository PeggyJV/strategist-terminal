import { type Adaptor, PlaceHolder } from "$lib/type"

const MorphoBlueSupplyV1: Adaptor = {
  name: "MorphoBlueSupplyV1",
  address: "",
  calls: [
    {
      function: "LendToMorphoBlue",
      action: "Lend",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "Amount of Loan Token to Lend",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawFromMorphoBlue",
      action: "Withdraw",
      fields: [
        {
          name: "market",
          label: "Identifier of a Morpho Blue Market",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "Amount of Loan Token to Withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default MorphoBlueSupplyV1;
