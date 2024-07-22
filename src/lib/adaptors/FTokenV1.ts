import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Address"
        },
        {
          name: "amount_to_deposit",
          label: "The amount of the fToken to lend",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default FTokenV1;
