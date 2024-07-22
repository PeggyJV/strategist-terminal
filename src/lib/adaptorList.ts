import AaveV3ATokenV1 from "$lib/adaptors/AaveV3ATokenV1"
import AaveATokenV1 from "$lib/adaptors/AaveATokenV1"
import AaveATokenV2 from "$lib/adaptors/AaveATokenV2"
import AaveDebtTokenV1 from "$lib/adaptors/AaveDebtTokenV1"
import AaveDebtTokenV2 from "$lib/adaptors/AaveDebtTokenV2"
import AaveV2EnableAssetAsCollateralV1 from "$lib/adaptors/AaveV2EnableAssetAsCollateralV1"
import AaveV3DebtTokenV1 from "$lib/adaptors/AaveV3DebtTokenV1"
import AaveATokenAdaptorV1 from "$lib/adaptors/AaveATokenAdaptorV1"
import AuraErc4626V1 from "$lib/adaptors/AuraErc4626V1"
import BalancerPoolV1 from "$lib/adaptors/BalancerPoolV1"
import CollateralFTokenV1 from "$lib/adaptors/CollateralFTokenV1"
import CompoundCTokenV2 from "$lib/adaptors/CompoundCTokenV2"
import ConvexCurveV1 from "$lib/adaptors/ConvexCurveV1"
import CurveV1 from "$lib/adaptors/CurveV1"
import DebtFTokenV1 from "$lib/adaptors/DebtFTokenV1"
import Erc4626V1V1 from "$lib/adaptors/Erc4626V1"

export interface Adaptor {
  name: string
  address: string
  calls: AdaptorCall[]
}

export interface AdaptorCall {
  function: string
  action: string
  fields: Field[]

}
export interface Field {
  name: string
  label: string
  placeholder: string

}
const adaptors: Adaptor[] = [
  AaveV3ATokenV1,
  AaveATokenV1,
  AaveATokenV2,
  AaveDebtTokenV1,
  AaveDebtTokenV2,
  AaveV2EnableAssetAsCollateralV1,
  AaveV3DebtTokenV1,
  AaveATokenAdaptorV1,
  AuraErc4626V1,
  BalancerPoolV1,
  CollateralFTokenV1,
  CompoundCTokenV2,
  ConvexCurveV1,
  CurveV1,
  DebtFTokenV1,
  Erc4626V1V1
]

export default adaptors;