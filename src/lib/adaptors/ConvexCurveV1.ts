import type { Adaptor } from "$lib/adaptorList"

const ConvexCurveV1: Adaptor = {
  name: "ConvexCurveV1",
  address: "",
  calls: [
    {
      function: "DepositLptInConvexAndStake",
      action: "Deposit",
      fields: [
        {
          name: "pid",
          label: "PID",
          placeholder: "Enter PID"
        },
        {
          name: "base_reward_pool",
          label: "Base Reward Pool",
          placeholder: "Address"
        },
        {
          name: "lpt",
          label: "lpt",
          placeholder: "lpt"
        },
        {
          name: "pool",
          label: "pool",
          placeholder: "pool"
        },
        {
          name: "selector",
          label: "selector",
          placeholder: "selector"
        },
        {
          name: "amount_to_deposit",
          label: "Enter amount to deposit",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "WithdrawFromBaseRewardPoolAsLpt",
      action: "Withdraw",
      fields: [
        {
          name: "base_reward_pool",
          label: "Enter Base Reward Pool Address",
          placeholder: "Address"
        },
        {
          name: "amount_to_withdraw",
          label: "Enter amount to withdraw",
          placeholder: "Amount"
        },
        {
          name: "claim",
          label: "Enter whether or not to rewards associated to respective ConvexCurve market position, as a bool",
          placeholder: "Boolean"
        }
      ]
    },
    {
      function: "GetRewards",
      action: "Get Rewards",
      fields: [
        {
          name: "base_reward_pool",
          label: "Enter Base Reward Pool Address",
          placeholder: "Address"
        },
        {
          name: "claim_extras",
          label: "Claim Extras",
          placeholder: "Boolean"
        }
      ]
    }
  ]
}
export default ConvexCurveV1;