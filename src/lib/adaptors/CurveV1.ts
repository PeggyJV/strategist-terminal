import type { Adaptor } from "$lib/adaptorList"

const CurveV1: Adaptor = {
  name: "CurveV1",
  address: "",
  calls: [
    {
      function: "AddLiquidity",
      action: "Add Liquidity",
      fields: [
        {
          name: "pool",
          label: "Enter Curve Pool Address",
          placeholder: "Address"
        },
        {
          name: "lp_token",
          label: "Enter LP Token Address",
          placeholder: "Address"
        },
        {
          name: "ordered_underlying_token_amounts",
          label: "Enter the minimum amount of each underlying token to receive, as an array of strings",
          placeholder: "e.g., [50, 100]"
        },
        {
          name: "min_lp_amount",
          label: "Enter the minimum amount of LP tokens to receive, as a string",
          placeholder: "Amount"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: "Address"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: "Selector"
        }
      ]
    },
    {
      function: "AddLiquidityETH",
      action: "Add Liquidity with ETH",
      fields: [
        {
          name: "pool",
          label: "Enter Curve Pool Address",
          placeholder: "Address"
        },
        {
          name: "lp_token",
          label: "Enter LP Token Address",
          placeholder: "Address"
        },
        {
          name: "ordered_underlying_token_amounts",
          label: "Enter the minimum amount of each underlying token to receive, as an array of strings",
          placeholder: "e.g., [50, 100]"
        },
        {
          name: "min_lp_amount",
          label: "Enter the minimum amount of LP tokens to receive",
          placeholder: "Amount"
        },
        {
          name: "use_underlying",
          label: "Enter a bool for whether to use the underlying asset or the wrapped asset",
          placeholder: "Boolean"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: "Address"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: "Selector"
        }
      ]
    },
    {
      function: "RemoveLiquidity",
      action: "Remove Liquidity",
      fields: [
        {
          name: "pool",
          label: "Enter the Curve Pool address",
          placeholder: "Address"
        },
        {
          name: "lp_token",
          label: "Enter the curve pool token address",
          placeholder: "Address"
        },
        {
          name: "lp_token_amount",
          label: "Enter the LP Token Amount",
          placeholder: "Amount"
        },
        {
          name: "ordered_minimum_underlying_token_amounts_out",
          label: "Enter the minimum amount of each underlying token to receive, as an array of strings",
          placeholder: "e.g., [50, 100]"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: "Address"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: "Selector"
        }
      ]
    },
    {
      function: "RemoveLiquidityETH",
      action: "Remove Liquidity with ETH",
      fields: [
        {
          name: "pool",
          label: "Enter the Curve Pool address",
          placeholder: "Address"
        },
        {
          name: "lp_token",
          label: "Enter the curve pool token address",
          placeholder: "Address"
        },
        {
          name: "lp_token_amount",
          label: "Enter the LP Token Amount",
          placeholder: "Amount"
        },
        {
          name: "ordered_minimum_underlying_token_amounts_out",
          label: "Enter the minimum amount of each underlying token to receive, as an array of strings",
          placeholder: "e.g., [50, 100]"
        },
        {
          name: "use_underlying",
          label: "Use Underlying",
          placeholder: "Boolean"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: "Address"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: "Selector"
        }
      ]
    },
    {
      function: "StakeInGauge",
      action: "Stake in Gauge",
      fields: [
        {
          name: "lp_token",
          label: "Enter the address of the LP token",
          placeholder: "Address"
        },
        {
          name: "gauge",
          label: "Enter the Curve Pool Gauge address",
          placeholder: "Address"
        },
        {
          name: "amount",
          label: "Enter the amount of LP tokens to stake",
          placeholder: "Amount"
        },
        {
          name: "pool",
          label: "Enter the address of the Curve Pool",
          placeholder: "Address"
        },
        {
          name: "selector",
          label: "Enter the selector for the function to call",
          placeholder: "Selector"
        }
      ]
    },
    {
      function: "UnstakeFromGauge",
      action: "Unstake from Gauge",
      fields: [
        {
          name: "gauge",
          label: "Enter the Curve Pool Gauge address",
          placeholder: "Address"
        },
        {
          name: "amount",
          label: "Enter the amount of LP tokens to unstake",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "ClaimRewards",
      action: "Claim Rewards",
      fields: [
        {
          name: "gauge",
          label: "Enter the address of the Curve Gauge",
          placeholder: "Address"
        }
      ]
    }
  ]
}
export default CurveV1;