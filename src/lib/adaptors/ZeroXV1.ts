import type { Adaptor } from "$lib/adaptorList";

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
          placeholder: "0xTokenInAddress"
        },
        {
          name: "token_out",
          label: "Token Out (ERC-20 Contract Address)",
          placeholder: "0xTokenOutAddress"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: "Amount"
        },
        {
          name: "swap_call_data",
          label: "Swap Call Data",
          placeholder: "Swap Call Data"
        }
      ]
    }
  ]
};

export default ZeroXV1;
