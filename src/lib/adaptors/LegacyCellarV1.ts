import type { Adaptor } from "$lib/adaptorList"

const LegacyCellarV1: Adaptor = {
  name: "LegacyCellarV1",
  address: "",
  calls: [
    {
      function: "DepositToCellar",
      action: "Deposit To Cellar",
      fields: [
        {
          name: "cellar",
          label: "Cellar",
          placeholder: "Cellar"
        },
        {
          name: "assets",
          label: "Assets to deposit",
          placeholder: "Assets"
        },
        {
          name: "oracle",
          label: "Oracle",
          placeholder: "Oracle"
        }
      ]
    },
    {
      function: "WithdrawFromCellar",
      action: "Withdraw From Cellar",
      fields: [
        {
          name: "cellar",
          label: "Cellar",
          placeholder: "Cellar"
        },
        {
          name: "assets",
          label: "Assets to withdraw",
          placeholder: "Assets"
        },
        {
          name: "oracle",
          label: "Oracle",
          placeholder: "Oracle"
        }
      ]
    }
  ]
}

export default LegacyCellarV1;
