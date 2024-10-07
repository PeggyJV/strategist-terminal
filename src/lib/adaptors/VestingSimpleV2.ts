import { type Adaptor, PlaceHolder } from "$lib/type"

const VestingSimpleV2: Adaptor = {
  name: "VestingSimpleV2",
  address: "",
  calls: [
    {
      function: "DepositToVesting",
      action: "Deposit funds into a vesting contract",
      fields: [
        {
          name: "vesting_contract",
          label: "Vesting Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount to Deposit",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawFromVesting",
      action: "Withdraw funds from a specific deposit in the vesting contract",
      fields: [
        {
          name: "vesting_contract",
          label: "Vesting Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "deposit_id",
          label: "Deposit ID",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount to Withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawAnyFromVesting",
      action: "Withdraw any amount from a vesting contract",
      fields: [
        {
          name: "vesting_contract",
          label: "Vesting Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount to Withdraw",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    },
    {
      function: "WithdrawAllFromVesting",
      action: "Withdraw all funds from a vesting contract",
      fields: [
        {
          name: "vesting_contract",
          label: "Vesting Contract Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
};

export default VestingSimpleV2;
