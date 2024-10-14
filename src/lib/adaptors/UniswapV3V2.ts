import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "pool_fee",
          label: "Pool Fee",
          placeholder: PlaceHolder.Empty,
          type: "number"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "tick_lower",
          label: "Tick Lower",
          placeholder: PlaceHolder.Empty,
          type: "number"
        },
        {
          name: "tick_upper",
          label: "Tick Upper",
          placeholder: PlaceHolder.Empty,
          type: "number"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "liquidity",
          label: "Liquidity",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_0",
          label: "Min 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "min_1",
          label: "Min 1",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "take_fees",
          label: "Take Fees",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_0",
          label: "Amount 0",
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "amount_1",
          label: "Amount 1",
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: PlaceHolder.Address,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
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
          placeholder: PlaceHolder.Text,
          type: "text"
        },
        {
          name: "token_0",
          label: "Token 0",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "token_1",
          label: "Token 1",
          placeholder: PlaceHolder.Address,
          type: "text"
        }
      ]
    }
  ]
};

export default UniswapV3V2;
