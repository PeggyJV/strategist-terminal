import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Market"
        },
        {
          name: "assets",
          label: "Amount of Loan Token to Lend",
          placeholder: "Amount"
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
          placeholder: "Market"
        },
        {
          name: "assets",
          label: "Amount of Loan Token to Withdraw",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default MorphoBlueSupplyV1;
