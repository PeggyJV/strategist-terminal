import { type Adaptor, PlaceHolder } from "$lib/type"

const ZeroXV1: Adaptor = {
  name: "ZeroXV1",
  address: "",
  calls: [
    {
      function: "SwapWith0x",
      action: "Swap assets using the 0x protocol",
      fields: [
        {
          name: "token_in",
          label: "Token In (ERC-20 Contract Address)",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_out",
          label: "Token Out (ERC-20 Contract Address)",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "swap_call_data",
          label: "Swap Call Data",
          placeholder: PlaceHolder.ArrayOfNumber,
          type: "array"
        }
      ]
    }
  ]
};

export default ZeroXV1;
