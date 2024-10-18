import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Empty,
          type: "number"
        },
        {
          name: "performance_fee",
          label: "Performance fee",
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
}

export default FeesAndReservesV1;
