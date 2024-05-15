use eyre::Result;
use serde::Deserialize;
use steward_proto::proto::*;

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) enum Adaptors {
    AaveATokenV1,
    AaveDebtTokenV1,
    CompoundCTokenV2,
    AaveATokenV2,
    AaveDebtTokenV2,
    AaveV3ATokenV1,
    AaveV3DebtTokenV1,
    OneInchV1,
    FeesAndReservesV1,
    ZeroXV1,
    SwapWithUniswapV1,
    VestingSimpleV2,
    CellarV1,
    UniswapV3V2,
    AaveV2EnableAssetAsCollateralV1,
    FTokenV1,
    MorphoAaveV2ATokenV1,
    MorphoAaveV2DebtTokenV1,
    MorphoAaveV3ATokenCollateralV1,
    MorphoAaveV3ATokenP2pV1,
    MorphoAaveV3DebtTokenV1,
    BalancerPoolV1,
    LegacyCellarV1,
    DebtFTokenV1,
    CollateralFTokenV1,
    AaveV3DebtTokenV1FlashLoan,
    BalancerPoolV1FlashLoan,
    ConvexCurveV1,
    CurveV1,
    AuraErc4626V1,
    MorphoBlueCollateralV1,
    MorphoBlueDebtV1,
    MorphoBlueSupplyV1,
    Erc4626V1,
    StakingV1,
    #[serde(untagged)]
    #[default]
    Invalid,
}

pub(crate) fn get_aave_a_token_adaptor_call(fields: &str) -> Result<AaveATokenAdaptorV1> {
    let function = serde_json::from_str::<aave_a_token_adaptor_v1::Function>(fields)?;

    Ok(AaveATokenAdaptorV1 {
        function: Some(function),
    })
}

#[cfg(test)]
mod tests {
    use steward_proto::proto::aave_a_token_adaptor_v1::DepositToAave;

    use super::*;

    #[test]
    fn test_get_aave_a_token_adaptor_call() {
        let fields = r#"{"DepositToAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let result = get_aave_a_token_adaptor_call(fields).expect("failed to parse fields");
        let expected = AaveATokenAdaptorV1 {
            function: Some(aave_a_token_adaptor_v1::Function::DepositToAave(
                DepositToAave {
                    token: "0x0000000000000000000000000000000000000000".to_string(),
                    amount: "1000000".to_string(),
                },
            )),
        };
    }
}
