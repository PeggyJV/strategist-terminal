import type { Adaptor } from "$lib/adaptorList";

const UniswapV3V2: Adaptor = {
  name: "UniswapV3V2",
  address: "",
  calls: [
    {
      function: "OpenPosition",
      action: "Open a new liquidity position",
      fields: [
        {
          name: "token_0",
          label: "Token 0",
          placeholder: "Enter Token 0 Address"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: "Enter Token 1 Address"
        },
        {
          name: "pool_fee",
          label: "Pool Fee",
          placeholder: "Enter Pool Fee"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: "Enter Amount 0"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: "Enter Amount 1"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: "Enter Min 0"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: "Enter Min 1"
        },
        {
          name: "tick_lower",
          label: "Tick Lower",
          placeholder: "Enter Tick Lower"
        },
        {
          name: "tick_upper",
          label: "Tick Upper",
          placeholder: "Enter Tick Upper"
        }
      ]
    },
    {
      function: "ClosePosition",
      action: "Close an existing liquidity position",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: "Enter Minimum 0"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: "Enter Minimum 1"
        }
      ]
    },
    {
      function: "AddToPosition",
      action: "Add liquidity to an existing position",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: "Enter Amount 0"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: "Enter Amount 1"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: "Enter Min 0"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: "Enter Min 1"
        }
      ]
    },
    {
      function: "TakeFromPosition",
      action: "Remove liquidity from an existing position",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        },
        {
          name: "liquidity",
          label: "Liquidity",
          placeholder: "Enter Liquidity"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: "Enter Min 0"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: "Enter Min 1"
        },
        {
          name: "take_fees",
          label: "Take Fees",
          placeholder: "Check if you want to take fees"
        }
      ]
    },
    {
      function: "CollectFees",
      action: "Collect fees from an existing position",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: "Enter Amount 0"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: "Enter Amount 1"
        }
      ]
    },
    {
      function: "PurgeAllZeroLiquidityPositions",
      action: "Purge all positions with zero liquidity",
      fields: [
        {
          name: "token_0",
          label: "Token 0",
          placeholder: "Enter Token 0 Address"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: "Enter Token 1 Address"
        }
      ]
    },
    {
      function: "PurgeSinglePosition",
      action: "Purge a single position",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        }
      ]
    },
    {
      function: "RemoveUnownedPositionFromTracker",
      action: "Remove an unowned position from the tracker",
      fields: [
        {
          name: "token_id",
          label: "Token ID",
          placeholder: "Enter Token ID"
        },
        {
          name: "token_0",
          label: "Token 0",
          placeholder: "Enter Token 0 Address"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: "Enter Token 1 Address"
        }
      ]
    }
  ]
};

export default UniswapV3V2;
