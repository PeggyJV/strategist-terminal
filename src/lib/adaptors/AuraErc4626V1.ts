import { type Adaptor, PlaceHolder } from "$lib/type"

const AuraErc4626V1: Adaptor = {
  name: "AuraErc4626V1",
  address: "",
  calls: [
    {
      function: "GetRewards",
      action: "Get Rewards",
      fields: [
        {
          name: "aura_pool",
          label: "Aura Pool Address",
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "claim_extras",
          label: "Claim Extra Rewards",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        }
      ]
    }
  ]
}
export default AuraErc4626V1;