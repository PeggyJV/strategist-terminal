import { type Adaptor, PlaceHolder } from "$lib/type"

const SwapWithUniswapV1: Adaptor = {
  name: "SwapWithUniswapV1",
  address: "",
  calls: [
    {
      function: "SwapWithUniV2",
      action: "Swap with Uniswap V2",
      fields: [
        {
          name: "path",
          label: "Address",
          placeholder: PlaceHolder.ArrayOfAddress,
          type: "array"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_out_min",
          label: "Amount min",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "SwapWithUniV3",
      action: "Swap with Uniswap V3",
      fields: [
        {
          name: "path",
          label: "Address",
          placeholder: PlaceHolder.ArrayOfAddress,
          type: "array"
        },
        {
          name: "pool_fees",
          label: "Pool fees",
          placeholder: PlaceHolder.ArrayOfNumber,
          type: "array"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_out_min",
          label: "Amount min",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
};

export default SwapWithUniswapV1;
