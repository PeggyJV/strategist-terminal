import { type Adaptor, PlaceHolder } from "$lib/type"

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
          placeholder: PlaceHolder.Address,
          type: "text"
        },
        {
          name: "use_as_collateral",
          label: "Use the asset as collateral",
          placeholder: PlaceHolder.Empty,
          type: "checkbox"
        }
      ]
    }
  ]
}
export default AaveV2EnableAssetAsCollateralV1;