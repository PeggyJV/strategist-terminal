import type { Adaptor } from "$lib/adaptorList";

const StakingV1: Adaptor = {
  name: "StakingV1",
  address: "",
  calls: [
    {
      function: "Mint",
      action: "Mint",
      fields: [
        {
          name: "amount",
          label: "Amount",
          placeholder: "Amount"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: "Min Amount Out"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Wildcard Data"
        }
      ]
    },
    {
      function: "RequestBurn",
      action: "Request Burn",
      fields: [
        {
          name: "amount",
          label: "Amount",
          placeholder: "Amount"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Wildcard Data"
        }
      ]
    },
    {
      function: "CompleteBurn",
      action: "Complete Burn",
      fields: [
        {
          name: "id",
          label: "Burn Request ID",
          placeholder: "Burn Request ID"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: "Min Amount Out"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Wildcard Data"
        }
      ]
    },
    {
      function: "CancelBurn",
      action: "Cancel Burn",
      fields: [
        {
          name: "id",
          label: "Burn Request ID",
          placeholder: "Burn Request ID"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Wildcard Data"
        }
      ]
    },
    {
      function: "Wrap",
      action: "Wrap",
      fields: [
        {
          name: "amount",
          label: "Amount to Wrap",
          placeholder: "Enter amount"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: "Enter minimum amount out"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Enter wildcard data"
        }
      ]
    },
    {
      function: "Unwrap",
      action: "Unwrap",
      fields: [
        {
          name: "amount",
          label: "Amount to Unwrap",
          placeholder: "Enter amount"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: "Enter minimum amount out"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Enter wildcard data"
        }
      ]
    },
    {
      function: "MintErc20",
      action: "Mint with ERC20",
      fields: [
        {
          name: "deposit_asset",
          label: "Deposit Asset Address",
          placeholder: "Enter deposit asset address"
        },
        {
          name: "amount",
          label: "Amount",
          placeholder: "Enter amount"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: "Enter minimum amount out"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Enter wildcard data"
        }
      ]
    },
    {
      function: "RemoveClaimedRequest",
      action: "Remove Claimed Request",
      fields: [
        {
          name: "id",
          label: "Request ID",
          placeholder: "Enter request ID"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: "Enter wildcard data"
        }
      ]
    }
  ]
};

export default StakingV1;
