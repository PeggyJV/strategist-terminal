import type { Adaptor } from "$lib/adaptorList";

const OneInchV1: Adaptor = {
  name: "OneInchV1",
  address: "",
  calls: [
    {
      function: "SwapWithOneInch",
      action: "Swap",
      fields: [
        {
          name: "token_in",
          label: "The Address of the Token to Swap From",
          placeholder: "0xtoken"
        },
        {
          name: "token_out",
          label: "The Address of the Token to Swap To",
          placeholder: "0xtoken"
        },
        {
          name: "amount",
          label: "The Amount to Swap",
          placeholder: "Amount"
        },
        {
          name: "swap_call_data",
          label: "Swap Calldata",
          placeholder: "Swap Calldata"
        }
      ]
    }
  ]
};

export default OneInchV1;
