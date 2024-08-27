use crate::adaptors::*;
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};
use steward_proto::proto::{
    aave_v3_debt_token_adaptor_v1_flash_loan::adaptor_call_for_aave_v3_flash_loan,
    aave_v3_debt_token_adaptor_v1_flash_loan::AdaptorCallForAaveV3FlashLoan,
    adaptor_call,
    balancer_pool_adaptor_v1_flash_loan::adaptor_call_for_balancer_pool_flash_loan,
    balancer_pool_adaptor_v1_flash_loan::AdaptorCallForBalancerPoolFlashLoan,
    cellar_v2_5::{self},
    schedule_request::CallData,
    AdaptorCall, CellarV25,
};

// for now this is more of an "adaptor call" than a "cellar call" as it assumes
// the function being called is CallOnAdaptor. In the future we'll need to generalize
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct CellarCall {
    pub adaptor: String,
    pub name: Adaptors,
    pub fields: String,
}

impl std::fmt::Display for CellarCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CellarCall{{adaptor: {}, name: {}, fields: {}}}",
            self.adaptor,
            self.name.to_string(),
            self.fields
        )
    }
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
            Adaptors::AaveV3DebtTokenV1FlashLoan => get_aave_v3_debt_token_flash_loan_adaptor_call(
                &self.adaptor,
                &self.fields,
                Vec::new(),
            ),
            Adaptors::BalancerPoolV1FlashLoan => {
                get_balancer_pool_flash_loan_adaptor_call(&self.adaptor, &self.fields, Vec::new())
            }
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

pub(crate) fn convert_to_aave_v3_flash_loan_adaptor(
    call: &AdaptorCall,
) -> Result<AdaptorCallForAaveV3FlashLoan> {
    Ok(AdaptorCallForAaveV3FlashLoan {
        adaptor: call.adaptor.clone(),
        call_data: convert_call_data_aave_flash_loan(&call.call_data)?,
    })
}
pub(crate) fn convert_to_balancer_pool_flash_loan_adaptor(
    call: &AdaptorCall,
) -> Result<AdaptorCallForBalancerPoolFlashLoan> {
    Ok(AdaptorCallForBalancerPoolFlashLoan {
        adaptor: call.adaptor.clone(),
        call_data: convert_call_data_balancer_pool_flash_loan(&call.call_data)?,
    })
}
fn convert_call_data_aave_flash_loan(
    call_data: &Option<adaptor_call::CallData>,
) -> Result<Option<adaptor_call_for_aave_v3_flash_loan::CallData>> {
    match call_data {
        Some(adaptor_call::CallData::AaveATokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveATokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AaveDebtTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveDebtTokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::CompoundCTokenV2Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::CompoundCTokenV2Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AaveATokenV2Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveATokenV2Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AaveDebtTokenV2Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveDebtTokenV2Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AaveV3ATokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveV3ATokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::OneInchV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::OneInchV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::FeesAndReservesV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::FeesAndReservesV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::ZeroXV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::ZeroXV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::SwapWithUniswapV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::SwapWithUniswapV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::VestingSimpleV2Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::VestingSimpleV2Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::CellarV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::CellarV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::UniswapV3V2Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::UniswapV3V2Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AaveV2EnableAssetAsCollateralV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AaveV2EnableAssetAsCollateralV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::FTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::FTokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::MorphoAaveV2ATokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoAaveV2ATokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::MorphoAaveV2DebtTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoAaveV2DebtTokenV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::MorphoAaveV3ATokenCollateralV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoAaveV3ATokenCollateralV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::MorphoAaveV3ATokenP2pV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoAaveV3ATokenP2pV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::MorphoAaveV3DebtTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoAaveV3DebtTokenV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::BalancerPoolV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::BalancerPoolV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::LegacyCellarV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::LegacyCellarV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::DebtFTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::DebtFTokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::CollateralFTokenV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::CollateralFTokenV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::ConvexCurveV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::ConvexCurveV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::CurveV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::CurveV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::AuraErc4626V1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::AuraErc4626V1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::MorphoBlueCollateralV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoBlueCollateralV1Calls(
                data.clone(),
            ),
        )),
        Some(adaptor_call::CallData::MorphoBlueDebtV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoBlueDebtV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::MorphoBlueSupplyV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::MorphoBlueSupplyV1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::Erc4626V1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::Erc4626V1Calls(data.clone()),
        )),
        Some(adaptor_call::CallData::StakingV1Calls(data)) => Ok(Some(
            adaptor_call_for_aave_v3_flash_loan::CallData::StakingV1Calls(data.clone()),
        )),
        _ => Ok(None),
    }
}

fn convert_call_data_balancer_pool_flash_loan(
    call_data: &Option<adaptor_call::CallData>,
) -> Result<Option<adaptor_call_for_balancer_pool_flash_loan::CallData>> {
    match call_data {
        Some(adaptor_call::CallData::AaveATokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveATokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AaveDebtTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveDebtTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::CompoundCTokenV2Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::CompoundCTokenV2Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AaveATokenV2Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveATokenV2Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AaveDebtTokenV2Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveDebtTokenV2Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AaveV3ATokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveV3ATokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::OneInchV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::OneInchV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::FeesAndReservesV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::FeesAndReservesV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::ZeroXV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::ZeroXV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::SwapWithUniswapV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::SwapWithUniswapV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::VestingSimpleV2Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::VestingSimpleV2Calls(data.clone())))
        }
        Some(adaptor_call::CallData::CellarV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::CellarV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::UniswapV3V2Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::UniswapV3V2Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AaveV2EnableAssetAsCollateralV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AaveV2EnableAssetAsCollateralV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::FTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::FTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoAaveV2ATokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoAaveV2ATokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoAaveV2DebtTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoAaveV2DebtTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoAaveV3ATokenCollateralV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoAaveV3ATokenCollateralV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoAaveV3ATokenP2pV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoAaveV3ATokenP2pV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoAaveV3DebtTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoAaveV3DebtTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::LegacyCellarV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::LegacyCellarV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::DebtFTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::DebtFTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::CollateralFTokenV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::CollateralFTokenV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::ConvexCurveV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::ConvexCurveV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::CurveV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::CurveV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::AuraErc4626V1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::AuraErc4626V1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoBlueCollateralV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoBlueCollateralV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoBlueDebtV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoBlueDebtV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::MorphoBlueSupplyV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::MorphoBlueSupplyV1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::Erc4626V1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::Erc4626V1Calls(data.clone())))
        }
        Some(adaptor_call::CallData::StakingV1Calls(data)) => {
            Ok(Some(adaptor_call_for_balancer_pool_flash_loan::CallData::StakingV1Calls(data.clone())))
        }
        _ => {
            Ok(None)
        }
    }
}
