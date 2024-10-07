import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_out",
          label: "The Address of the Token to Swap To",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "The Amount to Swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "swap_call_data",
          label: "Swap Calldata",
          placeholder: PlaceHolder.ArrayOfNumber,
          type: "array"
        }
      ]
    }
  ]
};

export default OneInchV1;
