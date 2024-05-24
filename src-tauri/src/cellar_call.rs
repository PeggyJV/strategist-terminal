use eyre::{bail, Result};
use serde::Deserialize;
use steward_proto::proto::{
    cellar_v2_5::{self},
    schedule_request::CallData,
    AdaptorCall, CellarV25,
};

use crate::adaptors::*;

// for now this is more of an "adaptor call" than a "cellar call" as it assumes
// the function being called is CallOnAdaptor. In the future we'll need to generalize
#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct CellarCall {
    pub adaptor: String,
    pub name: Adaptors,
    pub fields: String,
}

impl CellarCall {
    pub fn to_adaptor_call(&self) -> Result<AdaptorCall> {
        match self.name {
            Adaptors::AaveATokenV1 => get_aave_a_token_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::AaveDebtTokenV1 => {
                get_aave_debt_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::CompoundCTokenV2 => {
                get_compound_c_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::AaveATokenV2 => get_aave_a_token_v2_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::AaveDebtTokenV2 => {
                get_aave_debt_token_v2_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::AaveV3ATokenV1 => {
                get_aave_v3_a_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::AaveV3DebtTokenV1 => {
                get_aave_v3_debt_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::OneInchV1 => get_one_inch_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::FeesAndReservesV1 => {
                get_fees_and_reserves_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::ZeroXV1 => get_zero_x_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::SwapWithUniswapV1 => {
                get_swap_with_uniswap_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::VestingSimpleV2 => {
                get_vesting_simple_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::CellarV1 => get_cellar_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::UniswapV3V2 => get_uniswap_v3_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::AaveV2EnableAssetAsCollateralV1 => {
                get_aave_v2_enable_asset_as_collateral_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::FTokenV1 => get_f_token_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::MorphoAaveV2ATokenV1 => {
                get_morpho_aave_v2_a_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoAaveV2DebtTokenV1 => {
                get_morpho_aave_v2_debt_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoAaveV3ATokenCollateralV1 => {
                get_morpho_aave_v3_a_token_collateral_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoAaveV3ATokenP2pV1 => {
                get_morpho_aave_v3_a_token_p2p_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoAaveV3DebtTokenV1 => {
                get_morpho_aave_v3_debt_token_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::BalancerPoolV1 => get_balancer_pool_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::LegacyCellarV1 => get_legacy_cellar_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::DebtFTokenV1 => get_debt_f_token_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::CollateralFTokenV1 => {
                get_collateral_f_token_adaptor_call(&self.adaptor, &self.fields)
            }
            //Adaptors::AaveV3DebtTokenV1FlashLoan => {
            //    get_aave_v3_debt_token_flash_loan_adaptor_call(&self.adaptor, &self.fields)
            //}
            //Adaptors::BalancerPoolV1FlashLoan => {
            //    get_balancer_pool_flash_loan_adaptor_call(&self.adaptor, &self.fields)
            //}
            Adaptors::ConvexCurveV1 => get_convex_curve_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::CurveV1 => get_curve_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::AuraErc4626V1 => get_aura_erc4626_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::MorphoBlueCollateralV1 => {
                get_morpho_blue_collateral_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoBlueDebtV1 => {
                get_morpho_blue_debt_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::MorphoBlueSupplyV1 => {
                get_morpho_blue_supply_adaptor_call(&self.adaptor, &self.fields)
            }
            Adaptors::Erc4626V1 => get_erc4626_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::StakingV1 => get_staking_adaptor_call(&self.adaptor, &self.fields),
            Adaptors::Invalid => bail!("invalid adaptor name: {:?}", &self.name),
        }
    }
}

pub(crate) fn construct_call_data(adaptor_calls: Vec<AdaptorCall>) -> CallData {
    CallData::CellarV25(construct_cellar_v2_5_call(adaptor_calls))
}

fn construct_cellar_v2_5_call(adaptor_calls: Vec<AdaptorCall>) -> CellarV25 {
    CellarV25 {
        call_type: Some(construct_function_call_type(adaptor_calls)),
    }
}

fn construct_function_call_type(adaptor_calls: Vec<AdaptorCall>) -> cellar_v2_5::CallType {
    cellar_v2_5::CallType::FunctionCall(construct_function_call(adaptor_calls))
}

fn construct_function_call(adaptor_calls: Vec<AdaptorCall>) -> cellar_v2_5::FunctionCall {
    cellar_v2_5::FunctionCall {
        function: Some(construct_function(adaptor_calls)),
    }
}

fn construct_function(adaptor_calls: Vec<AdaptorCall>) -> cellar_v2_5::function_call::Function {
    cellar_v2_5::function_call::Function::CallOnAdaptor(construct_call_on_adaptor(adaptor_calls))
}

fn construct_call_on_adaptor(adaptor_calls: Vec<AdaptorCall>) -> cellar_v2_5::CallOnAdaptor {
    cellar_v2_5::CallOnAdaptor {
        data: adaptor_calls,
    }
}
