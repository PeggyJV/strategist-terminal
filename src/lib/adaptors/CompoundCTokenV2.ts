import type { Adaptor } from "$lib/adaptorList"

const CompoundCTokenV2: Adaptor = {
  name: "CompoundCTokenV2",
  address: "",
  calls: [
    {
      function: "DepositToCompound",
      action: "Deposit",
      fields: [
        {
          name: "market",
          label: "Enter the Compound market address",
          placeholder: "Market"
        },
        {
          name: "amount_to_deposit",
          label: "Enter amount to deposit",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "WithdrawFromCompound",
      action: "Withdraw",
      fields: [
        {
          name: "market",
          label: "Enter the Compound market address",
          placeholder: "Market"
        },
        {
          name: "amount_to_withdraw",
          label: "Enter amount to withdraw",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "ClaimComp",
      action: "Claim",
      fields: []
    }
  ]
}
export default CompoundCTokenV2;