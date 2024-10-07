import { type Adaptor, PlaceHolder } from "$lib/type"

const FTokenV1: Adaptor = {
  name: "FTokenV1",
  address: "",
  calls: [
    {
      function: "LendFrax",
      action: "Lend Frax",
      fields: [
        {
          name: "f_token",
          label: "The address of the fToken to lend",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_deposit",
          label: "The amount of the fToken to lend",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default FTokenV1;
