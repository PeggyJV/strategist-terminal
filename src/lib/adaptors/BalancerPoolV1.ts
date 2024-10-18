import { type Adaptor, PlaceHolder } from "$lib/type"

const BalancerPoolV1: Adaptor = {
  name: "BalancerPoolV1",
  address: "",
  calls: [
    {
      function: "SingleSwap",
      action: "Perform Swap",
      fields: [
        {
          name: "pool_id",
          label: "Pool ID",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "kind",
          label: "Swap Kind",
          placeholder: PlaceHolder.Empty,
          type: "number"
        },
        {
          name: "asset_in",
          label: "The asset in",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "asset_out",
          label: "The asset out",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "The amount",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "user_data",
          label: "The user data",
          placeholder: PlaceHolder.ArrayOfNumber,
          type: "array"
        }
      ]
    },
    {
      function: "SwapData",
      action: "Perform Swap",
      fields: [
        {
          name: "min_amounts_for_swaps",
          label: "The minimum amounts for swaps",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "swap_deadlines",
          label: "The swap deadlines",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        }
      ]
    },
    {
      function: "JoinPool",
      action: "Join Pool",
      fields: [
        {
          name: "target_bpt",
          label: "Target BPT",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "swaps_before_join",
          label: "Swap to execute before joining pool",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "swap_data",
          label: "Data for swaps",
          placeholder: PlaceHolder.ArrayOfString,
          // "{min_amounts_for_swaps: ['1'], swap_deadlines: : ['1']}",
          type: "text"
        },
        {
          name: "minimum_bpt",
          label: "The minimum BPT to mint",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "ExitPoolRequest",
      action: "Submit Exit Pool Request",
      fields: [
        {
          name: "assets",
          label: "Assets",
          placeholder: PlaceHolder.ArrayOfAddress,
          type: "array"
        },
        {
          name: "min_amounts_out",
          label: "Minimum Amounts Out",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "user_data",
          label: "User Data",
          placeholder: PlaceHolder.ArrayOfNumber,
          type: "array"
        },
        {
          name: "to_internal_balance",
          label: "To Internal Balance",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"

        }
      ]
    },
    {
      function: "ExitPool",
      action: "Exit Pool",
      fields: [
        {
          name: "exit_target_bpt",
          label: "Swaps to execute after exiting pool",
          placeholder: PlaceHolder.Empty,
          type: "text"
        },
        {
          name: "swaps_after_exit",
          label: "Target BPT",
          placeholder: PlaceHolder.ArrayOfString,
          type: "array"
        },
        {
          name: "swap_data",
          label: "Exit swap data",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "request",
          label: "Request",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "StakeBPT",
      action: "Stake BPT",
      fields: [
        {
          name: "bpt",
          label: "BPT",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "liquidity_gauge",
          label: "Liquidity Gauge",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_in",
          label: "Amount",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "UnstakeBPT",
      action: "Unstake BPT",
      fields: [
        {
          name: "bpt",
          label: "BPT",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "liquidity_gauge",
          label: "Liquidity Gauge",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_in",
          label: "Amount",
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
          label: "Gauge",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
}
export default BalancerPoolV1;