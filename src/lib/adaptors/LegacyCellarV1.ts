import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "Assets to deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "oracle",
          label: "Oracle",
          placeholder: PlaceHolder.Address,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "assets",
          label: "Assets to withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "oracle",
          label: "Oracle",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
}

export default LegacyCellarV1;
