import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Enter Pool ID"
        },
        {
          name: "kind",
          label: "Swap Kind",
          placeholder: "Kind"
        },
        {
          name: "asset_in",
          label: "The asset in",
          placeholder: "Address"
        },
        {
          name: "asset_out",
          label: "The asset out",
          placeholder: "Address"
        },
        {
          name: "amount",
          label: "The amount",
          placeholder: "Amount"
        },
        {
          name: "user_data",
          label: "The user data",
          placeholder: "User data"
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
          placeholder: "Amounts"
        },
        {
          name: "swap_deadlines",
          label: "The swap deadlines",
          placeholder: "Deadlines"
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
          placeholder: "BPT"
        },
        {
          name: "swaps_before_join",
          label: "Swap to execute before joining pool",
          placeholder: "Swap"
        },
        {
          name: "swap_data",
          label: "Data for swaps",
          placeholder: "Data"
        },
        {
          name: "minimum_bpt",
          label: "The minimum BPT to mint",
          placeholder: "BPT"
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
          placeholder: "Asset Addresses, comma-separated"
        },
        {
          name: "min_amounts_out",
          label: "Minimum Amounts Out",
          placeholder: "Minimum Amounts, comma-separated"
        },
        {
          name: "user_data",
          label: "User Data",
          placeholder: "User Data in bytes"
        },
        {
          name: "to_internal_balance",
          label: "To Internal Balance",
          placeholder: "Boolean"
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
          placeholder: "Swaps"
        },
        {
          name: "swaps_after_exit",
          label: "Target BPT",
          placeholder: "Enter Target BPT"
        },
        {
          name: "swap_data",
          label: "Exit swap data",
          placeholder: "Swap Data"
        },
        {
          name: "request",
          label: "Request",
          placeholder: "Request"
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
          placeholder: "BPT Address"
        },
        {
          name: "liquidity_gauge",
          label: "Liquidity Gauge",
          placeholder: "Liquidity Gauge Address"
        },
        {
          name: "amount_in",
          label: "Amount",
          placeholder: "Amount to Stake"
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
          placeholder: "BPT Address"
        },
        {
          name: "liquidity_gauge",
          label: "Liquidity Gauge",
          placeholder: "Liquidity Gauge Address"
        },
        {
          name: "amount_in",
          label: "Amount",
          placeholder: "Amount to Unstake"
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
          placeholder: "BPT Address"
        },
        {
          name: "liquidity_gauge",
          label: "Liquidity Gauge",
          placeholder: "Liquidity Gauge Address"
        },
        {
          name: "amount_in",
          label: "Amount",
          placeholder: "Amount to Unstake"
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
          placeholder: "Gauge Address"
        }
      ]
    }
  ]
}
export default BalancerPoolV1;