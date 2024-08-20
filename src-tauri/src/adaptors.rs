use std::{fmt::write, marker::PhantomData};

use eyre::Result;
use log::kv::ToValue;
use serde::{Deserialize, Serialize};
use steward_proto::proto::*;

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
    // AaveV3DebtTokenV1FlashLoan,
    // BalancerPoolV1FlashLoan,
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
            // Adaptors::AaveV3DebtTokenV1FlashLoan => write!(f, "AaveV3DebtTokenV1FlashLoan"),
            // Adaptors::BalancerPoolV1FlashLoan => write!(f, "BalancerPoolV1FlashLoan"),
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

impl<'s> ToValue for &'s Adaptors {
    fn to_value(&self) -> log::kv::Value<'s> {
        log::kv::Value::from_display(&self.to_string())
    }
}

pub(crate) fn get_aave_a_token_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_a_token_adaptor_v1::Function>(fields)?;
    let call = AaveATokenAdaptorV1 {
        function: Some(function),
    };
    let calls = AaveATokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveATokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_debt_token_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_debt_token_adaptor_v1::Function>(fields)?;
    let call = AaveDebtTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = AaveDebtTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveDebtTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_compound_c_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<compound_c_token_adaptor_v2::Function>(fields)?;
    let call = CompoundCTokenAdaptorV2 {
        function: Some(function),
    };
    let calls = CompoundCTokenAdaptorV2Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::CompoundCTokenV2Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_a_token_v2_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_a_token_adaptor_v2::Function>(fields)?;
    let call = AaveATokenAdaptorV2 {
        function: Some(function),
    };
    let calls = AaveATokenAdaptorV2Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveATokenV2Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_debt_token_v2_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_debt_token_adaptor_v2::Function>(fields)?;
    let call = AaveDebtTokenAdaptorV2 {
        function: Some(function),
    };
    let calls = AaveDebtTokenAdaptorV2Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveDebtTokenV2Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_v3_a_token_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_v3a_token_adaptor_v1::Function>(fields)?;
    let call = AaveV3aTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = AaveV3aTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveV3ATokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_v3_debt_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aave_v3_debt_token_adaptor_v1::Function>(fields)?;
    let call = AaveV3DebtTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = AaveV3DebtTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveV3DebtTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_one_inch_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<one_inch_adaptor_v1::Function>(fields)?;
    let call = OneInchAdaptorV1 {
        function: Some(function),
    };
    let calls = OneInchAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::OneInchV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_fees_and_reserves_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<fees_and_reserves_adaptor_v1::Function>(fields)?;
    let call = FeesAndReservesAdaptorV1 {
        function: Some(function),
    };
    let calls = FeesAndReservesAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::FeesAndReservesV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_zero_x_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<zero_x_adaptor_v1::Function>(fields)?;
    let call = ZeroXAdaptorV1 {
        function: Some(function),
    };
    let calls = ZeroXAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::ZeroXV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_swap_with_uniswap_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<swap_with_uniswap_adaptor_v1::Function>(fields)?;
    let call = SwapWithUniswapAdaptorV1 {
        function: Some(function),
    };
    let calls = SwapWithUniswapAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::SwapWithUniswapV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_vesting_simple_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<vesting_simple_adaptor_v2::Function>(fields)?;
    let call = VestingSimpleAdaptorV2 {
        function: Some(function),
    };
    let calls = VestingSimpleAdaptorV2Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::VestingSimpleV2Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_cellar_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<cellar_adaptor_v1::Function>(fields)?;
    let call = CellarAdaptorV1 {
        function: Some(function),
    };
    let calls = CellarAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::CellarV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_uniswap_v3_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<uniswap_v3_adaptor_v2::Function>(fields)?;
    let call = UniswapV3AdaptorV2 {
        function: Some(function),
    };
    let calls = UniswapV3AdaptorV2Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::UniswapV3V2Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aave_v2_enable_asset_as_collateral_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function =
        serde_json::from_str::<aave_v2_enable_asset_as_collateral_adaptor_v1::Function>(fields)?;
    let call = AaveV2EnableAssetAsCollateralAdaptorV1 {
        function: Some(function),
    };
    let calls = AaveV2EnableAssetAsCollateralAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AaveV2EnableAssetAsCollateralV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_f_token_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<f_token_adaptor_v1::Function>(fields)?;
    let call = FTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = FTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::FTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_aave_v2_a_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_aave_v2a_token_adaptor_v1::Function>(fields)?;
    let call = MorphoAaveV2aTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoAaveV2aTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoAaveV2ATokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_aave_v2_debt_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_aave_v2_debt_token_adaptor_v1::Function>(fields)?;
    let call = MorphoAaveV2DebtTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoAaveV2DebtTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoAaveV2DebtTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_aave_v3_a_token_collateral_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function =
        serde_json::from_str::<morpho_aave_v3a_token_collateral_adaptor_v1::Function>(fields)?;
    let call = MorphoAaveV3aTokenCollateralAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoAaveV3aTokenCollateralAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoAaveV3ATokenCollateralV1Calls(
            calls,
        )),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_aave_v3_a_token_p2p_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_aave_v3a_token_p2p_adaptor_v1::Function>(fields)?;
    let call = MorphoAaveV3aTokenP2pAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoAaveV3aTokenP2pAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoAaveV3ATokenP2pV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_aave_v3_debt_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_aave_v3_debt_token_adaptor_v1::Function>(fields)?;
    let call = MorphoAaveV3DebtTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoAaveV3DebtTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoAaveV3DebtTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_balancer_pool_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<balancer_pool_adaptor_v1::Function>(fields)?;
    let call = BalancerPoolAdaptorV1 {
        function: Some(function),
    };
    let calls = BalancerPoolAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::BalancerPoolV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_legacy_cellar_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<legacy_cellar_adaptor_v1::Function>(fields)?;
    let call = LegacyCellarAdaptorV1 {
        function: Some(function),
    };
    let calls = LegacyCellarAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::LegacyCellarV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_debt_f_token_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<debt_f_token_adaptor_v1::Function>(fields)?;
    let call = DebtFTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = DebtFTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::DebtFTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_collateral_f_token_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<collateral_f_token_adaptor_v1::Function>(fields)?;
    let call = CollateralFTokenAdaptorV1 {
        function: Some(function),
    };
    let calls = CollateralFTokenAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::CollateralFTokenV1Calls(calls)),
    };

    Ok(adaptor_call)
}

//pub(crate) fn get_aave_v3_debt_token_flash_loan_adaptor_call(
//    adaptor: &str,
//    fields: &str,
//) -> Result<AdaptorCall> {
//    let function = serde_json::from_str::<aave_v3_debt_token_v1::Function>(fields)?;
//    let call = AaveV3DebtTokenV1FlashLoan {
//        function: Some(function),
//    };
//    let calls = AaveV3DebtTokenV1FlashLoanCalls { calls: vec![call] };
//    let adaptor_call = AdaptorCall {
//        adaptor: adaptor.to_owned(),
//        call_data: Some(adaptor_call::CallData::AaveV3DebtTokenV1FlashLoanCalls(calls)),
//    };
//
//    Ok(adaptor_call)
//}
//
//pub(crate) fn get_balancer_pool_flash_loan_adaptor_call(
//    adaptor: &str,
//    fields: &str,
//) -> Result<AdaptorCall> {
//    let function = serde_json::from_str::<balancer_pool_adaptor_v1::Function>(fields)?;
//    let call = BalancerPoolV1FlashLoan {
//        function: Some(function),
//    };
//    let calls = BalancerPoolV1FlashLoanCalls { calls: vec![call] };
//    let adaptor_call = AdaptorCall {
//        adaptor: adaptor.to_owned(),
//        call_data: Some(adaptor_call::CallData::BalancerPoolV1FlashLoanCalls(calls)),
//    };
//
//    Ok(adaptor_call)
//}

pub(crate) fn get_convex_curve_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<convex_curve_adaptor_v1::Function>(fields)?;
    let call = ConvexCurveAdaptorV1 {
        function: Some(function),
    };
    let calls = ConvexCurveAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::ConvexCurveV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_curve_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<curve_adaptor_v1::Function>(fields)?;
    let call = CurveAdaptorV1 {
        function: Some(function),
    };
    let calls = CurveAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::CurveV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_aura_erc4626_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<aura_erc4626_adaptor_v1::Function>(fields)?;
    let call = AuraErc4626AdaptorV1 {
        function: Some(function),
    };
    let calls = AuraErc4626AdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::AuraErc4626V1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_blue_collateral_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_blue_collateral_adaptor_v1::Function>(fields)?;
    let call = MorphoBlueCollateralAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoBlueCollateralAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoBlueCollateralV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_blue_debt_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_blue_debt_adaptor_v1::Function>(fields)?;
    let call = MorphoBlueDebtAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoBlueDebtAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoBlueDebtV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_morpho_blue_supply_adaptor_call(
    adaptor: &str,
    fields: &str,
) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<morpho_blue_supply_adaptor_v1::Function>(fields)?;
    let call = MorphoBlueSupplyAdaptorV1 {
        function: Some(function),
    };
    let calls = MorphoBlueSupplyAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::MorphoBlueSupplyV1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_erc4626_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<erc4626_adaptor_v1::Function>(fields)?;
    let call = Erc4626AdaptorV1 {
        function: Some(function),
    };
    let calls = Erc4626AdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::Erc4626V1Calls(calls)),
    };

    Ok(adaptor_call)
}

pub(crate) fn get_staking_adaptor_call(adaptor: &str, fields: &str) -> Result<AdaptorCall> {
    let function = serde_json::from_str::<staking_adaptor_v1::Function>(fields)?;
    let call = StakingAdaptorV1 {
        function: Some(function),
    };
    let calls = StakingAdaptorV1Calls { calls: vec![call] };
    let adaptor_call = AdaptorCall {
        adaptor: adaptor.to_owned(),
        call_data: Some(adaptor_call::CallData::StakingV1Calls(calls)),
    };

    Ok(adaptor_call)
}

#[cfg(test)]
mod tests {
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
}
