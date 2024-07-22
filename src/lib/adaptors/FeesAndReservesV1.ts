import type { Adaptor } from "$lib/adaptorList"

const FeesAndReservesV1: Adaptor = {
  name: "FeesAndReservesV1",
  address: "",
  calls: [
    {
      function: "UpdatePerformanceFees",
      action: "Update Performance Fees",
      fields: [
        {
          name: "performance_fee",
          label: "New performance fee",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "UpdateManagementFees",
      action: "Update Management Fees",
      fields: [
        {
          name: "management_fee",
          label: "New management fee",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "ChangeUpkeepFrequency",
      action: "Change Upkeep Frequency",
      fields: [
        {
          name: "new_frequency",
          label: "New frequency",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "ChangeUpkeepMaxGas",
      action: "Change Upkeep Max Gas",
      fields: [
        {
          name: "new_max_gas",
          label: "New max gas",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "SetupMetaData",
      action: "Setup Meta Data",
      fields: [
        {
          name: "management_fee",
          label: "Management fee",
          placeholder: "Amount"
        },
        {
          name: "performance_fee",
          label: "Performance fee",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "AddAssetsToReserves",
      action: "Add Assets To Reserves",
      fields: [
        {
          name: "amount",
          label: "Amount",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "WithdrawAssetsFromReserves",
      action: "Withdraw Assets From Reserves",
      fields: [
        {
          name: "amount",
          label: "Amount",
          placeholder: "Amount"
        }
      ]
    },
    {
      function: "PrepareFees",
      action: "Prepare Fees",
      fields: [
        {
          name: "amount",
          label: "Amount",
          placeholder: "Amount"
        }
      ]
    }
  ]
}

export default FeesAndReservesV1;
