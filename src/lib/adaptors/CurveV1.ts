import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token",
          label: "Enter LP Token Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "ordered_underlying_token_amounts",
          label: "Enter the minimum amount of each underlying token to receive",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "min_lp_amount",
          label: "Enter the minimum amount of LP tokens to receive, as a string",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token",
          label: "Enter LP Token Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "ordered_underlying_token_amounts",
          label: "Enter the minimum amount of each underlying token to receive",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "min_lp_amount",
          label: "Enter the minimum amount of LP tokens to receive",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "use_underlying",
          label: "Enter a bool for whether to use the underlying asset or the wrapped asset",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token",
          label: "Enter the curve pool token address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token_amount",
          label: "Enter the LP Token Amount",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "ordered_minimum_underlying_token_amounts_out",
          label: "Enter the minimum amount of each underlying token to receive",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token",
          label: "Enter the curve pool token address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lp_token_amount",
          label: "Enter the LP Token Amount",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "ordered_minimum_underlying_token_amounts_out",
          label: "Enter the minimum amount of each underlying token to receive",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "use_underlying",
          label: "Use Underlying",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        },
        {
          name: "gauge",
          label: "Enter the curve gauge address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "Enter the selector",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "gauge",
          label: "Enter the Curve Pool Gauge address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Enter the amount of LP tokens to stake",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "pool",
          label: "Enter the address of the Curve Pool",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "Enter the selector for the function to call",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Enter the amount of LP tokens to unstake",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
}
export default CurveV1;