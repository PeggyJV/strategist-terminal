import type { Adaptor } from "$lib/adaptorList"

const AaveV2EnableAssetAsCollateralV1: Adaptor = {
  name: "AaveV2EnableAssetAsCollateralV1",
  address: "",
  calls: [
    {
      function: "SetUserUseReserveAsCollateral",
      action: "Deposit",
      fields: [
        {
          name: "asset",
          label: "The address of the asset to set as collateral",
          placeholder: "Address"
        },
        {
          name: "use_as_collateral",
          label: "Use the asset as collateral",
          placeholder: "Boolean"
        }
      ]
    }
  ]
}
export default AaveV2EnableAssetAsCollateralV1;