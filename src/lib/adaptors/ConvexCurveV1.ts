import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "base_reward_pool",
          label: "Base Reward Pool",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "lpt",
          label: "lpt",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "pool",
          label: "pool",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "selector",
          label: "selector",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_to_deposit",
          label: "Enter amount to deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_withdraw",
          label: "Enter amount to withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "claim",
          label: "Enter whether or not to rewards associated to respective ConvexCurve market position, as a bool",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "claim_extras",
          label: "Claim Extras",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        }
      ]
    }
  ]
}
export default ConvexCurveV1;