use eyre::Result;
use log::kv::ToValue;
use serde::{Deserialize, Serialize};
use steward_proto::proto::aave_v3_debt_token_adaptor_v1_flash_loan::*;
use steward_proto::proto::balancer_pool_adaptor_v1_flash_loan::*;
use steward_proto::proto::*;

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

impl std::fmt::Display for Adaptors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adaptors::AaveATokenV1 => write!(f, "AaveATokenV1"),
            Adaptors::AaveDebtTokenV1 => write!(f, "AaveDebtTokenV1"),
            Adaptors::CompoundCTokenV2 => write!(f, "CompoundCTokenV2"),
            Adaptors::AaveATokenV2 => write!(f, "AaveATokenV2"),
            Adaptors::AaveDebtTokenV2 => write!(f, "AaveDebtTokenV2"),
            Adaptors::AaveV3ATokenV1 => write!(f, "AaveV3ATokenV1"),
            Adaptors::AaveV3DebtTokenV1 => write!(f, "AaveV3DebtTokenV1"),
            Adaptors::OneInchV1 => write!(f, "OneInchV1"),
            Adaptors::FeesAndReservesV1 => write!(f, "FeesAndReservesV1"),
            Adaptors::ZeroXV1 => write!(f, "ZeroXV1"),
            Adaptors::SwapWithUniswapV1 => write!(f, "SwapWithUniswapV1"),
            Adaptors::VestingSimpleV2 => write!(f, "VestingSimpleV2"),
            Adaptors::CellarV1 => write!(f, "CellarV1"),
            Adaptors::UniswapV3V2 => write!(f, "UniswapV3V2"),
            Adaptors::AaveV2EnableAssetAsCollateralV1 => {
                write!(f, "AaveV2EnableAssetAsCollateralV1")
            }
            Adaptors::FTokenV1 => write!(f, "FTokenV1"),
            Adaptors::MorphoAaveV2ATokenV1 => write!(f, "MorphoAaveV2ATokenV1"),
            Adaptors::MorphoAaveV2DebtTokenV1 => write!(f, "MorphoAaveV2DebtTokenV1"),
            Adaptors::MorphoAaveV3ATokenCollateralV1 => write!(f, "MorphoAaveV3ATokenCollateralV1"),
            Adaptors::MorphoAaveV3ATokenP2pV1 => write!(f, "MorphoAaveV3ATokenP2pV1"),
            Adaptors::MorphoAaveV3DebtTokenV1 => write!(f, "MorphoAaveV3DebtTokenV1"),
            Adaptors::BalancerPoolV1 => write!(f, "BalancerPoolV1"),
            Adaptors::LegacyCellarV1 => write!(f, "LegacyCellarV1"),
            Adaptors::DebtFTokenV1 => write!(f, "DebtFTokenV1"),
            Adaptors::CollateralFTokenV1 => write!(f, "CollateralFTokenV1"),
            Adaptors::AaveV3DebtTokenV1FlashLoan => write!(f, "AaveV3DebtTokenV1FlashLoan"),
            Adaptors::BalancerPoolV1FlashLoan => write!(f, "BalancerPoolV1FlashLoan"),
            Adaptors::ConvexCurveV1 => write!(f, "ConvexCurveV1"),
            Adaptors::CurveV1 => write!(f, "CurveV1"),
            Adaptors::AuraErc4626V1 => write!(f, "AuraErc4626V1"),
            Adaptors::MorphoBlueCollateralV1 => write!(f, "MorphoBlueCollateralV1"),
            Adaptors::MorphoBlueDebtV1 => write!(f, "MorphoBlueDebtV1"),
            Adaptors::MorphoBlueSupplyV1 => write!(f, "MorphoBlueSupplyV1"),
            Adaptors::Erc4626V1 => write!(f, "Erc4626V1"),
            Adaptors::StakingV1 => write!(f, "StakingV1"),
            Adaptors::Invalid => write!(f, "Invalid"),
        }
    }
}

impl ToValue for Adaptors {
    fn to_value(&self) -> log::kv::Value<'_> {
        log::kv::Value::from_serde(self)
    }
}

macro_rules! define_adaptor_call {
    ($func_name:ident, $proto_module:ident, $adaptor_type:ident, $call_data_variant:ident) => {
        pub(crate) fn $func_name(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
            let function = serde_json::from_str::<$proto_module::Function>(fields)?;
            let call = $adaptor_type {
                function: Some(function),
            };
            let calls = paste::paste! { [<$adaptor_type Calls>] { calls: vec![call] } };
            let adaptor_call = AdaptorCall {
                adaptor: adaptor.to_owned(),
                call_data: Some(adaptor_call::CallData::$call_data_variant(calls)),
            };

            Ok(adaptor_call)
        }
    };
}

define_adaptor_call!(
    get_aave_a_token_adaptor_call,
    aave_a_token_adaptor_v1,
    AaveATokenAdaptorV1,
    AaveATokenV1Calls
);
define_adaptor_call!(
    get_aave_debt_token_adaptor_call,
    aave_debt_token_adaptor_v1,
    AaveDebtTokenAdaptorV1,
    AaveDebtTokenV1Calls
);
define_adaptor_call!(
    get_compound_c_token_adaptor_call,
    compound_c_token_adaptor_v2,
    CompoundCTokenAdaptorV2,
    CompoundCTokenV2Calls
);
define_adaptor_call!(
    get_aave_a_token_v2_adaptor_call,
    aave_a_token_adaptor_v2,
    AaveATokenAdaptorV2,
    AaveATokenV2Calls
);
define_adaptor_call!(
    get_aave_debt_token_v2_adaptor_call,
    aave_debt_token_adaptor_v2,
    AaveDebtTokenAdaptorV2,
    AaveDebtTokenV2Calls
);
define_adaptor_call!(
    get_aave_v3_a_token_adaptor_call,
    aave_v3a_token_adaptor_v1,
    AaveV3aTokenAdaptorV1,
    AaveV3ATokenV1Calls
);
define_adaptor_call!(
    get_aave_v3_debt_token_adaptor_call,
    aave_v3_debt_token_adaptor_v1,
    AaveV3DebtTokenAdaptorV1,
    AaveV3DebtTokenV1Calls
);
define_adaptor_call!(
    get_one_inch_adaptor_call,
    one_inch_adaptor_v1,
    OneInchAdaptorV1,
    OneInchV1Calls
);
define_adaptor_call!(
    get_fees_and_reserves_adaptor_call,
    fees_and_reserves_adaptor_v1,
    FeesAndReservesAdaptorV1,
    FeesAndReservesV1Calls
);
define_adaptor_call!(
    get_zero_x_adaptor_call,
    zero_x_adaptor_v1,
    ZeroXAdaptorV1,
    ZeroXV1Calls
);
define_adaptor_call!(
    get_swap_with_uniswap_adaptor_call,
    swap_with_uniswap_adaptor_v1,
    SwapWithUniswapAdaptorV1,
    SwapWithUniswapV1Calls
);
define_adaptor_call!(
    get_vesting_simple_adaptor_call,
    vesting_simple_adaptor_v2,
    VestingSimpleAdaptorV2,
    VestingSimpleV2Calls
);
define_adaptor_call!(
    get_cellar_adaptor_call,
    cellar_adaptor_v1,
    CellarAdaptorV1,
    CellarV1Calls
);
define_adaptor_call!(
    get_uniswap_v3_adaptor_call,
    uniswap_v3_adaptor_v2,
    UniswapV3AdaptorV2,
    UniswapV3V2Calls
);
define_adaptor_call!(
    get_aave_v2_enable_asset_as_collateral_adaptor_call,
    aave_v2_enable_asset_as_collateral_adaptor_v1,
    AaveV2EnableAssetAsCollateralAdaptorV1,
    AaveV2EnableAssetAsCollateralV1Calls
);
define_adaptor_call!(
    get_f_token_adaptor_call,
    f_token_adaptor_v1,
    FTokenAdaptorV1,
    FTokenV1Calls
);
define_adaptor_call!(
    get_morpho_aave_v2_a_token_adaptor_call,
    morpho_aave_v2a_token_adaptor_v1,
    MorphoAaveV2aTokenAdaptorV1,
    MorphoAaveV2ATokenV1Calls
);
define_adaptor_call!(
    get_morpho_aave_v2_debt_token_adaptor_call,
    morpho_aave_v2_debt_token_adaptor_v1,
    MorphoAaveV2DebtTokenAdaptorV1,
    MorphoAaveV2DebtTokenV1Calls
);
define_adaptor_call!(
    get_morpho_aave_v3_a_token_collateral_adaptor_call,
    morpho_aave_v3a_token_collateral_adaptor_v1,
    MorphoAaveV3aTokenCollateralAdaptorV1,
    MorphoAaveV3ATokenCollateralV1Calls
);
define_adaptor_call!(
    get_morpho_aave_v3_a_token_p2p_adaptor_call,
    morpho_aave_v3a_token_p2p_adaptor_v1,
    MorphoAaveV3aTokenP2pAdaptorV1,
    MorphoAaveV3ATokenP2pV1Calls
);
define_adaptor_call!(
    get_morpho_aave_v3_debt_token_adaptor_call,
    morpho_aave_v3_debt_token_adaptor_v1,
    MorphoAaveV3DebtTokenAdaptorV1,
    MorphoAaveV3DebtTokenV1Calls
);
define_adaptor_call!(
    get_balancer_pool_adaptor_call,
    balancer_pool_adaptor_v1,
    BalancerPoolAdaptorV1,
    BalancerPoolV1Calls
);
define_adaptor_call!(
    get_legacy_cellar_adaptor_call,
    legacy_cellar_adaptor_v1,
    LegacyCellarAdaptorV1,
    LegacyCellarV1Calls
);
define_adaptor_call!(
    get_debt_f_token_adaptor_call,
    debt_f_token_adaptor_v1,
    DebtFTokenAdaptorV1,
    DebtFTokenV1Calls
);
define_adaptor_call!(
    get_collateral_f_token_adaptor_call,
    collateral_f_token_adaptor_v1,
    CollateralFTokenAdaptorV1,
    CollateralFTokenV1Calls
);
define_adaptor_call!(
    get_convex_curve_adaptor_call,
    convex_curve_adaptor_v1,
    ConvexCurveAdaptorV1,
    ConvexCurveV1Calls
);
define_adaptor_call!(
    get_curve_adaptor_call,
    curve_adaptor_v1,
    CurveAdaptorV1,
    CurveV1Calls
);
define_adaptor_call!(
    get_aura_erc4626_adaptor_call,
    aura_erc4626_adaptor_v1,
    AuraErc4626AdaptorV1,
    AuraErc4626V1Calls
);
define_adaptor_call!(
    get_morpho_blue_collateral_adaptor_call,
    morpho_blue_collateral_adaptor_v1,
    MorphoBlueCollateralAdaptorV1,
    MorphoBlueCollateralV1Calls
);
define_adaptor_call!(
    get_morpho_blue_debt_adaptor_call,
    morpho_blue_debt_adaptor_v1,
    MorphoBlueDebtAdaptorV1,
    MorphoBlueDebtV1Calls
);
define_adaptor_call!(
    get_morpho_blue_supply_adaptor_call,
    morpho_blue_supply_adaptor_v1,
    MorphoBlueSupplyAdaptorV1,
    MorphoBlueSupplyV1Calls
);
define_adaptor_call!(
    get_erc4626_adaptor_call,
    erc4626_adaptor_v1,
    Erc4626AdaptorV1,
    Erc4626V1Calls
);
define_adaptor_call!(
    get_staking_adaptor_call,
    staking_adaptor_v1,
    StakingAdaptorV1,
    StakingV1Calls
);

// Define the flash loan adaptor call getters separately because of macro complexity
pub(crate) fn get_aave_v3_debt_token_flash_loan_adaptor_call(
    adaptor: &str,
    fields: &str,
    params: Vec<AdaptorCallForAaveV3FlashLoan>,
) -> Result<AdaptorCall> {
    let mut function =
        serde_json::from_str::<aave_v3_debt_token_adaptor_v1_flash_loan::FlashLoan>(fields)?;
    function.params = params;

    let call = AaveV3DebtTokenAdaptorV1FlashLoan {
        flash_loan: Some(function),
    };
    let calls = AaveV3DebtTokenAdaptorV1FlashLoanCalls { calls: vec![call] };

    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveV3DebtTokenV1FlashLoanCalls(
            calls,
        )),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_balancer_pool_flash_loan_adaptor_call(
    adaptor: &str,
    fields: &str,
    params: Vec<AdaptorCallForBalancerPoolFlashLoan>,
) -> Result<AdaptorCall> {
    let mut function =
        serde_json::from_str::<balancer_pool_adaptor_v1_flash_loan::MakeFlashLoan>(fields)?;
    function.data = params;

    let call = BalancerPoolAdaptorV1FlashLoan {
        make_flash_loan: Some(function),
    };
    let calls = BalancerPoolAdaptorV1FlashLoanCalls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::BalancerPoolV1FlashLoanCalls(calls)),
    };

    Ok(adaptor_call)
}

#[cfg(test)]
mod tests {
    use alloy::hex;
    use balancer_pool_adaptor_v1::ClaimRewards;
    use steward_proto::proto::aave_a_token_adaptor_v1::DepositToAave;

    use super::*;

    #[test]
    fn test_get_aave_a_token_adaptor_call() {
        let fields = r#"{"DepositToAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x1111111111111111111111111111111111111111";
        let result =
            get_aave_a_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveATokenV1Calls(
                AaveATokenAdaptorV1Calls {
                    calls: vec![AaveATokenAdaptorV1 {
                        function: Some(aave_a_token_adaptor_v1::Function::DepositToAave(
                            DepositToAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aave_debt_token_adaptor_call() {
        let fields = r#"{"BorrowFromAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x2222222222222222222222222222222222222222";
        let result =
            get_aave_debt_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveDebtTokenV1Calls(
                AaveDebtTokenAdaptorV1Calls {
                    calls: vec![AaveDebtTokenAdaptorV1 {
                        function: Some(aave_debt_token_adaptor_v1::Function::BorrowFromAave(
                            aave_debt_token_adaptor_v1::BorrowFromAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_compound_c_token_adaptor_call() {
        let fields = r#"{"DepositToCompound": {"market":"0x0000000000000000000000000000000000000000", "amount_to_deposit": "1000000"}}"#;
        let adaptor = "0x3333333333333333333333333333333333333333";
        let result =
            get_compound_c_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::CompoundCTokenV2Calls(
                CompoundCTokenAdaptorV2Calls {
                    calls: vec![CompoundCTokenAdaptorV2 {
                        function: Some(compound_c_token_adaptor_v2::Function::DepositToCompound(
                            compound_c_token_adaptor_v2::DepositToCompound {
                                market: "0x0000000000000000000000000000000000000000".to_string(),
                                amount_to_deposit: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aave_a_token_v2_adaptor_call() {
        let fields = r#"{"DepositToAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x4444444444444444444444444444444444444444";
        let result =
            get_aave_a_token_v2_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveATokenV2Calls(
                AaveATokenAdaptorV2Calls {
                    calls: vec![AaveATokenAdaptorV2 {
                        function: Some(aave_a_token_adaptor_v2::Function::DepositToAave(
                            aave_a_token_adaptor_v2::DepositToAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aave_debt_token_v2_adaptor_call() {
        let fields = r#"{"BorrowFromAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x5555555555555555555555555555555555555555";
        let result =
            get_aave_debt_token_v2_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveDebtTokenV2Calls(
                AaveDebtTokenAdaptorV2Calls {
                    calls: vec![AaveDebtTokenAdaptorV2 {
                        function: Some(aave_debt_token_adaptor_v2::Function::BorrowFromAave(
                            aave_debt_token_adaptor_v2::BorrowFromAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aave_v3_a_token_adaptor_call() {
        let fields = r#"{"DepositToAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x6666666666666666666666666666666666666666";
        let result =
            get_aave_v3_a_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveV3ATokenV1Calls(
                AaveV3aTokenAdaptorV1Calls {
                    calls: vec![AaveV3aTokenAdaptorV1 {
                        function: Some(aave_v3a_token_adaptor_v1::Function::DepositToAave(
                            aave_v3a_token_adaptor_v1::DepositToAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aave_v3_debt_token_adaptor_v1_call() {
        let fields = r#"{"BorrowFromAave": {"token":"0x0000000000000000000000000000000000000000", "amount": "1000000"}}"#;
        let adaptor = "0x7777777777777777777777777777777777777777";
        let result =
            get_aave_v3_debt_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AaveV3DebtTokenV1Calls(
                AaveV3DebtTokenAdaptorV1Calls {
                    calls: vec![AaveV3DebtTokenAdaptorV1 {
                        function: Some(aave_v3_debt_token_adaptor_v1::Function::BorrowFromAave(
                            aave_v3_debt_token_adaptor_v1::BorrowFromAave {
                                token: "0x0000000000000000000000000000000000000000".to_string(),
                                amount: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_one_inch_adaptor_call() {
        let fields = r#"{"SwapWithOneInch": {"token_in":"0x1111111111111111111111111111111111111111", "token_out": "0x2222222222222222222222222222222222222222", "amount": "1000000", "swap_call_data": [18, 52]}}"#;
        let adaptor = "0x8888888888888888888888888888888888888888";
        let result = get_one_inch_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::OneInchV1Calls(
                OneInchAdaptorV1Calls {
                    calls: vec![OneInchAdaptorV1 {
                        function: Some(one_inch_adaptor_v1::Function::SwapWithOneInch(
                            one_inch_adaptor_v1::SwapWithOneInch {
                                token_in: "0x1111111111111111111111111111111111111111".to_string(),
                                token_out: "0x2222222222222222222222222222222222222222".to_string(),
                                amount: "1000000".to_string(),
                                swap_call_data: hex::decode("0x1234").unwrap(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_fees_and_reserves_adaptor_call() {
        let fields = r#"{"AddAssetsToReserves": {"amount": "500000"}}"#;
        let adaptor = "0x9999999999999999999999999999999999999999";
        let result =
            get_fees_and_reserves_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::FeesAndReservesV1Calls(
                FeesAndReservesAdaptorV1Calls {
                    calls: vec![FeesAndReservesAdaptorV1 {
                        function: Some(
                            fees_and_reserves_adaptor_v1::Function::AddAssetsToReserves(
                                fees_and_reserves_adaptor_v1::AddAssetsToReserves {
                                    amount: "500000".to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_zero_x_adaptor_call() {
        let fields = r#"{"SwapWith0x": {"token_in":"0x4444444444444444444444444444444444444444", "token_out": "0x5555555555555555555555555555555555555555", "amount": "1000000", "swap_call_data": [86, 120]}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let result = get_zero_x_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::ZeroXV1Calls(ZeroXAdaptorV1Calls {
                calls: vec![ZeroXAdaptorV1 {
                    function: Some(zero_x_adaptor_v1::Function::SwapWith0x(
                        zero_x_adaptor_v1::SwapWith0x {
                            token_in: "0x4444444444444444444444444444444444444444".to_string(),
                            token_out: "0x5555555555555555555555555555555555555555".to_string(),
                            amount: "1000000".to_string(),
                            swap_call_data: hex::decode("0x5678").unwrap(),
                        },
                    )),
                }],
            })),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_swap_with_uniswap_adaptor_call() {
        let fields = r#"{"SwapWithUniV2": {"path": ["0x6666666666666666666666666666666666666666"], "amount": "1000000", "amount_out_min": "950000"}}"#;
        let adaptor = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        let result =
            get_swap_with_uniswap_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::SwapWithUniswapV1Calls(
                SwapWithUniswapAdaptorV1Calls {
                    calls: vec![SwapWithUniswapAdaptorV1 {
                        function: Some(swap_with_uniswap_adaptor_v1::Function::SwapWithUniV2(
                            swap_with_uniswap_adaptor_v1::SwapWithUniV2 {
                                path: vec!["0x6666666666666666666666666666666666666666".to_string()],
                                amount: "1000000".to_string(),
                                amount_out_min: "950000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_cellar_adaptor_call() {
        let fields = r#"{"DepositToCellar": {"cellar": "0x7777777777777777777777777777777777777777", "assets": "1000000"}}"#;
        let adaptor = "0xcccccccccccccccccccccccccccccccccccccccc";
        let result = get_cellar_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::CellarV1Calls(
                CellarAdaptorV1Calls {
                    calls: vec![CellarAdaptorV1 {
                        function: Some(cellar_adaptor_v1::Function::DepositToCellar(
                            cellar_adaptor_v1::DepositToCellar {
                                cellar: "0x7777777777777777777777777777777777777777".to_string(),
                                assets: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_uniswap_v3_adaptor_call() {
        let fields = r#"{"AddToPosition": {"token_id": "123", "amount_0": "1000000", "amount_1": "2000000", "min_0": "950000", "min_1": "1900000"}}"#;
        let adaptor = "0xdddddddddddddddddddddddddddddddddddddddd";
        let result = get_uniswap_v3_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::UniswapV3V2Calls(
                UniswapV3AdaptorV2Calls {
                    calls: vec![UniswapV3AdaptorV2 {
                        function: Some(uniswap_v3_adaptor_v2::Function::AddToPosition(
                            uniswap_v3_adaptor_v2::AddToPosition {
                                token_id: "123".to_string(),
                                amount_0: "1000000".to_string(),
                                amount_1: "2000000".to_string(),
                                min_0: "950000".to_string(),
                                min_1: "1900000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    // get_f_token_adaptor_call, f_token_adaptor_v1, FTokenAdaptorV1, FTokenV1Calls
    // get_morpho_aave_v2_a_token_adaptor_call, morpho_aave_v2a_token_adaptor_v1, MorphoAaveV2aTokenAdaptorV1, MorphoAaveV2ATokenV1Calls
    // get_morpho_aave_v2_debt_token_adaptor_call, morpho_aave_v2_debt_token_adaptor_v1, MorphoAaveV2DebtTokenAdaptorV1, MorphoAaveV2DebtTokenV1Calls
    // get_morpho_aave_v3_a_token_collateral_adaptor_call, morpho_aave_v3a_token_collateral_adaptor_v1, MorphoAaveV3aTokenCollateralAdaptorV1, MorphoAaveV3ATokenCollateralV1Calls
    // get_morpho_aave_v3_a_token_p2p_adaptor_call, morpho_aave_v3a_token_p2p_adaptor_v1, MorphoAaveV3aTokenP2pAdaptorV1, MorphoAaveV3ATokenP2pV1Calls
    // get_morpho_aave_v3_debt_token_adaptor_call, morpho_aave_v3_debt_token_adaptor_v1, MorphoAaveV3DebtTokenAdaptorV1, MorphoAaveV3DebtTokenV1Calls
    // get_balancer_pool_adaptor_call, balancer_pool_adaptor_v1, BalancerPoolAdaptorV1, BalancerPoolV1Calls
    // get_legacy_cellar_adaptor_call, legacy_cellar_adaptor_v1, LegacyCellarAdaptorV1, LegacyCellarV1Calls
    // get_debt_f_token_adaptor_call, debt_f_token_adaptor_v1, DebtFTokenAdaptorV1, DebtFTokenV1Calls
    // get_collateral_f_token_adaptor_call, collateral_f_token_adaptor_v1, CollateralFTokenAdaptorV1, CollateralFTokenV1Calls
    // get_convex_curve_adaptor_call, convex_curve_adaptor_v1, ConvexCurveAdaptorV1, ConvexCurveV1Calls
    // get_curve_adaptor_call, curve_adaptor_v1, CurveAdaptorV1, CurveV1Calls
    // get_aura_erc4626_adaptor_call, aura_erc4626_adaptor_v1, AuraErc4626AdaptorV1, AuraErc4626V1Calls
    // get_morpho_blue_collateral_adaptor_call, morpho_blue_collateral_adaptor_v1, MorphoBlueCollateralAdaptorV1, MorphoBlueCollateralV1Calls
    // get_morpho_blue_debt_adaptor_call, morpho_blue_debt_adaptor_v1, MorphoBlueDebtAdaptorV1, MorphoBlueDebtV1Calls
    // get_morpho_blue_supply_adaptor_call, morpho_blue_supply_adaptor_v1, MorphoBlueSupplyAdaptorV1, MorphoBlueSupplyV1Calls
    // get_erc4626_adaptor_call, erc4626_adaptor_v1, Erc4626AdaptorV1, Erc4626V1Calls
    // get_staking_adaptor_call, staking_adaptor_v1, StakingAdaptorV1, StakingV1Calls

    #[test]
    fn test_get_f_token_adaptor_call() {
        let fields = r#"{"RedeemFraxShare": {"f_token": "0x1111111111111111111111111111111111111111", "amount_to_redeem": "1000000"}}"#;
        let adaptor = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
        let result = get_f_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::FTokenV1Calls(
                FTokenAdaptorV1Calls {
                    calls: vec![FTokenAdaptorV1 {
                        function: Some(f_token_adaptor_v1::Function::RedeemFraxShare(
                            f_token_adaptor_v1::RedeemFraxShare {
                                f_token: "0x1111111111111111111111111111111111111111".to_string(),
                                amount_to_redeem: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_aave_v2_a_token_adaptor_call() {
        let fields = r#"{"DepositToAaveV2Morpho": {"a_token": "0x2222222222222222222222222222222222222222", "amount_to_deposit": "2000000"}}"#;
        let adaptor = "0xffffffffffffffffffffffffffffffffffffffff";
        let result = get_morpho_aave_v2_a_token_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoAaveV2ATokenV1Calls(
                MorphoAaveV2aTokenAdaptorV1Calls {
                    calls: vec![MorphoAaveV2aTokenAdaptorV1 {
                        function: Some(
                            morpho_aave_v2a_token_adaptor_v1::Function::DepositToAaveV2Morpho(
                                morpho_aave_v2a_token_adaptor_v1::DepositToAaveV2Morpho {
                                    a_token: "0x2222222222222222222222222222222222222222"
                                        .to_string(),
                                    amount_to_deposit: "2000000".to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_aave_v2_debt_token_adaptor_call() {
        let fields = r#"{"BorrowFromAaveV2Morpho": {"a_token": "0x3333333333333333333333333333333333333333", "amount_to_borrow": "3000000"}}"#;
        let adaptor = "0x1111111111111111111111111111111111111111";
        let result = get_morpho_aave_v2_debt_token_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoAaveV2DebtTokenV1Calls(
                MorphoAaveV2DebtTokenAdaptorV1Calls {
                    calls: vec![MorphoAaveV2DebtTokenAdaptorV1 {
                        function: Some(
                            morpho_aave_v2_debt_token_adaptor_v1::Function::BorrowFromAaveV2Morpho(
                                morpho_aave_v2_debt_token_adaptor_v1::BorrowFromAaveV2Morpho {
                                    a_token: "0x3333333333333333333333333333333333333333"
                                        .to_string(),
                                    amount_to_borrow: "3000000".to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_aave_v3_a_token_collateral_adaptor_call() {
        let fields = r#"{"DepositToAaveV3Morpho": {"token_to_deposit": "0x4444444444444444444444444444444444444444", "amount_to_deposit": "4000000"}}"#;
        let adaptor = "0x2222222222222222222222222222222222222222";
        let result = get_morpho_aave_v3_a_token_collateral_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoAaveV3ATokenCollateralV1Calls(
                MorphoAaveV3aTokenCollateralAdaptorV1Calls {
                    calls: vec![MorphoAaveV3aTokenCollateralAdaptorV1 {
                        function: Some(morpho_aave_v3a_token_collateral_adaptor_v1::Function::DepositToAaveV3Morpho(
                            morpho_aave_v3a_token_collateral_adaptor_v1::DepositToAaveV3Morpho {
                                token_to_deposit: "0x4444444444444444444444444444444444444444".to_string(),
                                amount_to_deposit: "4000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_aave_v3_a_token_p2p_adaptor_call() {
        let fields = r#"{"DepositToAaveV3Morpho": {"token_to_deposit": "0x5555555555555555555555555555555555555555", "amount_to_deposit": "5000000", "max_iterations": "10"}}"#;
        let adaptor = "0x3333333333333333333333333333333333333333";
        let result = get_morpho_aave_v3_a_token_p2p_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoAaveV3ATokenP2pV1Calls(
                MorphoAaveV3aTokenP2pAdaptorV1Calls {
                    calls: vec![MorphoAaveV3aTokenP2pAdaptorV1 {
                        function: Some(
                            morpho_aave_v3a_token_p2p_adaptor_v1::Function::DepositToAaveV3Morpho(
                                morpho_aave_v3a_token_p2p_adaptor_v1::DepositToAaveV3Morpho {
                                    token_to_deposit: "0x5555555555555555555555555555555555555555"
                                        .to_string(),
                                    amount_to_deposit: "5000000".to_string(),
                                    max_iterations: 10.to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_aave_v3_debt_token_adaptor_call() {
        let fields = r#"{"BorrowFromAaveV3Morpho": {"underlying": "0x6666666666666666666666666666666666666666", "amount_to_borrow": "6000000", "max_iterations": "10"}}"#;
        let adaptor = "0x4444444444444444444444444444444444444444";
        let result = get_morpho_aave_v3_debt_token_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoAaveV3DebtTokenV1Calls(
                MorphoAaveV3DebtTokenAdaptorV1Calls {
                    calls: vec![MorphoAaveV3DebtTokenAdaptorV1 {
                        function: Some(
                            morpho_aave_v3_debt_token_adaptor_v1::Function::BorrowFromAaveV3Morpho(
                                morpho_aave_v3_debt_token_adaptor_v1::BorrowFromAaveV3Morpho {
                                    underlying: "0x6666666666666666666666666666666666666666"
                                        .to_string(),
                                    amount_to_borrow: "6000000".to_string(),
                                    max_iterations: 10.to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_balancer_pool_adaptor_call() {
        let fields = r#"{"JoinPool": {"target_bpt": "0x7777777777777777777777777777777777777777", "swaps_before_join": [], "swap_data": {"min_amounts_for_swaps": [], "swap_deadlines": []}, "minimum_bpt": "7000000"}}"#;
        let adaptor = "0x5555555555555555555555555555555555555555";
        let result =
            get_balancer_pool_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::BalancerPoolV1Calls(
                BalancerPoolAdaptorV1Calls {
                    calls: vec![BalancerPoolAdaptorV1 {
                        function: Some(balancer_pool_adaptor_v1::Function::JoinPool(
                            balancer_pool_adaptor_v1::JoinPool {
                                target_bpt: "0x7777777777777777777777777777777777777777"
                                    .to_string(),
                                swaps_before_join: vec![],
                                swap_data: Some(balancer_pool_adaptor_v1::SwapData {
                                    min_amounts_for_swaps: vec![],
                                    swap_deadlines: vec![],
                                }),
                                minimum_bpt: "7000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_legacy_cellar_adaptor_call() {
        let fields = r#"{"DepositToCellar": {"cellar": "0x8888888888888888888888888888888888888888", "assets": "8000000", "oracle": "0x9999999999999999999999999999999999999999"}}"#;
        let adaptor = "0x6666666666666666666666666666666666666666";
        let result =
            get_legacy_cellar_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::LegacyCellarV1Calls(
                LegacyCellarAdaptorV1Calls {
                    calls: vec![LegacyCellarAdaptorV1 {
                        function: Some(legacy_cellar_adaptor_v1::Function::DepositToCellar(
                            legacy_cellar_adaptor_v1::DepositToCellar {
                                cellar: "0x8888888888888888888888888888888888888888".to_string(),
                                assets: "8000000".to_string(),
                                oracle: "0x9999999999999999999999999999999999999999".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_debt_f_token_adaptor_call() {
        let fields = r#"{"BorrowFromFraxlend": {"fraxlend_pair": "0x9999999999999999999999999999999999999999", "amount_to_borrow": "9000000"}}"#;
        let adaptor = "0x7777777777777777777777777777777777777777";
        let result =
            get_debt_f_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::DebtFTokenV1Calls(
                DebtFTokenAdaptorV1Calls {
                    calls: vec![DebtFTokenAdaptorV1 {
                        function: Some(debt_f_token_adaptor_v1::Function::BorrowFromFraxlend(
                            debt_f_token_adaptor_v1::BorrowFromFraxlend {
                                fraxlend_pair: "0x9999999999999999999999999999999999999999"
                                    .to_string(),
                                amount_to_borrow: "9000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_collateral_f_token_adaptor_call() {
        let fields = r#"{"AddCollateral": {"fraxlend_pair": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "collateral_to_deposit": "10000000"}}"#;
        let adaptor = "0x8888888888888888888888888888888888888888";
        let result =
            get_collateral_f_token_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::CollateralFTokenV1Calls(
                CollateralFTokenAdaptorV1Calls {
                    calls: vec![CollateralFTokenAdaptorV1 {
                        function: Some(collateral_f_token_adaptor_v1::Function::AddCollateral(
                            collateral_f_token_adaptor_v1::AddCollateral {
                                fraxlend_pair: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                                    .to_string(),
                                collateral_to_deposit: "10000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_convex_curve_adaptor_call() {
        let fields = r#"{"DepositLptInConvexAndStake": {"pid": "11", "base_reward_pool": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", "lpt": "0xcccccccccccccccccccccccccccccccccccccccc", "pool": "0xdddddddddddddddddddddddddddddddddddddddd", "selector": "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", "amount_to_deposit": "11000000"}}"#;
        let adaptor = "0x9999999999999999999999999999999999999999";
        let result =
            get_convex_curve_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::ConvexCurveV1Calls(
                ConvexCurveAdaptorV1Calls {
                    calls: vec![ConvexCurveAdaptorV1 {
                        function: Some(
                            convex_curve_adaptor_v1::Function::DepositLptInConvexAndStake(
                                convex_curve_adaptor_v1::DepositLptInConvexAndStake {
                                    pid: 11.to_string(),
                                    base_reward_pool: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                                        .to_string(),
                                    lpt: "0xcccccccccccccccccccccccccccccccccccccccc".to_string(),
                                    pool: "0xdddddddddddddddddddddddddddddddddddddddd".to_string(),
                                    selector: "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                                        .to_string(),
                                    amount_to_deposit: "11000000".to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_curve_adaptor_call() {
        let fields = r#"{"AddLiquidity": {"pool": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "lp_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2", "ordered_underlying_token_amounts": ["1000000", "2000000"], "min_lp_amount": "500000", "gauge": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3", "selector": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5";
        let result = get_curve_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::CurveV1Calls(CurveAdaptorV1Calls {
                calls: vec![CurveAdaptorV1 {
                    function: Some(curve_adaptor_v1::Function::AddLiquidity(
                        curve_adaptor_v1::AddLiquidity {
                            pool: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1".to_string(),
                            lp_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2".to_string(),
                            ordered_underlying_token_amounts: vec![
                                "1000000".to_string(),
                                "2000000".to_string(),
                            ],
                            min_lp_amount: "500000".to_string(),
                            gauge: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3".to_string(),
                            selector: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4".to_string(),
                        },
                    )),
                }],
            })),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_aura_erc4626_adaptor_call() {
        let fields = r#"{"GetRewards": {"aura_pool": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "claim_extras": true}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2";
        let result =
            get_aura_erc4626_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::AuraErc4626V1Calls(
                AuraErc4626AdaptorV1Calls {
                    calls: vec![AuraErc4626AdaptorV1 {
                        function: Some(aura_erc4626_adaptor_v1::Function::GetRewards(
                            aura_erc4626_adaptor_v1::GetRewards {
                                aura_pool: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1".to_string(),
                                claim_extras: true,
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_blue_collateral_adaptor_call() {
        let fields = r#"{"AddCollateral": {"market": {"loan_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "collateral_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2", "oracle": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3", "irm": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4", "lltv": "800000000000000000"}, "collateral_to_deposit": "10000000"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5";
        let result = get_morpho_blue_collateral_adaptor_call(adaptor, fields)
            .expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoBlueCollateralV1Calls(
                MorphoBlueCollateralAdaptorV1Calls {
                    calls: vec![MorphoBlueCollateralAdaptorV1 {
                        function: Some(morpho_blue_collateral_adaptor_v1::Function::AddCollateral(
                            morpho_blue_collateral_adaptor_v1::AddCollateral {
                                market: Some(MarketParams {
                                    loan_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1"
                                        .to_string(),
                                    collateral_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2"
                                        .to_string(),
                                    oracle: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3"
                                        .to_string(),
                                    irm: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4".to_string(),
                                    lltv: "800000000000000000".to_string(),
                                }),
                                collateral_to_deposit: "10000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_blue_debt_adaptor_call() {
        let fields = r#"{"BorrowFromMorphoBlue": {"market": {"loan_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "collateral_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2", "oracle": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3", "irm": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4", "lltv": "800000000000000000"}, "amount_to_borrow": "1000000"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5";
        let result =
            get_morpho_blue_debt_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoBlueDebtV1Calls(
                MorphoBlueDebtAdaptorV1Calls {
                    calls: vec![MorphoBlueDebtAdaptorV1 {
                        function: Some(
                            morpho_blue_debt_adaptor_v1::Function::BorrowFromMorphoBlue(
                                morpho_blue_debt_adaptor_v1::BorrowFromMorphoBlue {
                                    market: Some(MarketParams {
                                        loan_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1"
                                            .to_string(),
                                        collateral_token:
                                            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2".to_string(),
                                        oracle: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3"
                                            .to_string(),
                                        irm: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4"
                                            .to_string(),
                                        lltv: "800000000000000000".to_string(),
                                    }),
                                    amount_to_borrow: "1000000".to_string(),
                                },
                            ),
                        ),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_morpho_blue_supply_adaptor_call() {
        let fields = r#"{"LendToMorphoBlue": {"market": {"loan_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "collateral_token": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2", "oracle": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3", "irm": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4", "lltv": "800000000000000000"}, "assets": "1000000"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5";
        let result =
            get_morpho_blue_supply_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::MorphoBlueSupplyV1Calls(
                MorphoBlueSupplyAdaptorV1Calls {
                    calls: vec![MorphoBlueSupplyAdaptorV1 {
                        function: Some(morpho_blue_supply_adaptor_v1::Function::LendToMorphoBlue(
                            morpho_blue_supply_adaptor_v1::LendToMorphoBlue {
                                market: Some(MarketParams {
                                    loan_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1"
                                        .to_string(),
                                    collateral_token: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2"
                                        .to_string(),
                                    oracle: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3"
                                        .to_string(),
                                    irm: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4".to_string(),
                                    lltv: "800000000000000000".to_string(),
                                }),
                                assets: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_erc4626_adaptor_call() {
        let fields = r#"{"DepositToVault": {"erc4626_vault": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "assets": "1000000"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2";
        let result = get_erc4626_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::Erc4626V1Calls(
                Erc4626AdaptorV1Calls {
                    calls: vec![Erc4626AdaptorV1 {
                        function: Some(erc4626_adaptor_v1::Function::DepositToVault(
                            erc4626_adaptor_v1::DepositToVault {
                                erc4626_vault: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1"
                                    .to_string(),
                                assets: "1000000".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_staking_adaptor_call() {
        let fields = r#"{"Mint": {"staking_contract": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1", "amount": "1000000", "min_amount_out": "1000000", "wildcard": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1"}}"#;
        let adaptor = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2";
        let result = get_staking_adaptor_call(adaptor, fields).expect("failed to parse fields");
        let expected = AdaptorCall {
            adaptor: adaptor.to_string(),
            call_data: Some(adaptor_call::CallData::StakingV1Calls(
                StakingAdaptorV1Calls {
                    calls: vec![StakingAdaptorV1 {
                        function: Some(staking_adaptor_v1::Function::Mint(
                            staking_adaptor_v1::Mint {
                                amount: "1000000".to_string(),
                                min_amount_out: "1000000".to_string(),
                                wildcard: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1".to_string(),
                            },
                        )),
                    }],
                },
            )),
        };

        assert_eq!(expected, result);
    }
}
