import type { Adaptor } from "$lib/adaptorList";

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
          placeholder: "Vesting contract address"
        },
        {
          name: "amount",
          label: "Amount to Deposit",
          placeholder: "Amount"
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
          placeholder: "Vesting contract address"
        },
        {
          name: "deposit_id",
          label: "Deposit ID",
          placeholder: "Deposit ID"
        },
        {
          name: "amount",
          label: "Amount to Withdraw",
          placeholder: "Amount"
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
          placeholder: "Vesting contract address"
        },
        {
          name: "amount",
          label: "Amount to Withdraw",
          placeholder: "Amount"
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
          placeholder: "Vesting contract address"
        }
      ]
    }
  ]
};

export default VestingSimpleV2;
