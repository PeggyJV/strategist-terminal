import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount",
          label: "Amount",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_amount_out",
          label: "Min Amount Out",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "wildcard",
          label: "Wildcard Data",
          placeholder: PlaceHolder.Text,
          type: "text"
        }
      ]
    }
  ]
};

export default StakingV1;
