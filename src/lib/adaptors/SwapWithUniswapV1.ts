import type { Adaptor } from "$lib/adaptorList";

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
          placeholder: "Address"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: "Amount"
        },
        {
          name: "amount_out_min",
          label: "Amount min",
          placeholder: "Amount out min"
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
          placeholder: "Address"
        },
        {
          name: "pool_fees",
          label: "Pool fees",
          placeholder: "Pool fees"
        },
        {
          name: "amount",
          label: "Amount to Swap",
          placeholder: "Amount"
        },
        {
          name: "amount_out_min",
          label: "Amount min",
          placeholder: "Amount out min"
        }
      ]
    }
  ]
};

export default SwapWithUniswapV1;
