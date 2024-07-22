import type { Adaptor } from "$lib/adaptorList"

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
          placeholder: "Enter Aura Pool Address"
        },
        {
          name: "claim_extras",
          label: "Claim Extra Rewards",
          placeholder: "Boolean"
        }
      ]
    }
  ]
}
export default AuraErc4626V1;