import type { Adaptor } from "$lib/adaptorList"

const AaveATokenAdaptorV1: Adaptor = {
  name: "AaveATokenAdaptorV1",
  address: "",
  calls: [
    {
      function: "DepositToAave",
      action: "Deposit",
      fields: [
        {
          name: "token_id",
          label: "ERC-20 Token Contract Address",
          placeholder: "0xTokenID"
        },
        {
          name: "amount",
          label: "Amount of ERC-20 Asset",
          placeholder: "Amount"
        }
      ]
    }
  ]
}
export default AaveATokenAdaptorV1;