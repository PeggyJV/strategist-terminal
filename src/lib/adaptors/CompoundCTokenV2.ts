import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
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
      function: "WithdrawFromCompound",
      action: "Withdraw",
      fields: [
        {
          name: "market",
          label: "Enter the Compound market address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount_to_withdraw",
          label: "Enter amount to withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
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