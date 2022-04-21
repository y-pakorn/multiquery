use std::collections::HashMap;

use cosmwasm_std::{to_binary, ContractResult, QuerierResult, SystemError, SystemResult, Uint128};
use cw20::{BalanceResponse, Cw20QueryMsg};

#[derive(Default)]
pub(super) struct Cw20Querier {
    /// Mapping token address and user address to the user's token balance
    pub balances: HashMap<String, u128>,
}

impl Cw20Querier {
    pub fn handle_query(&self, contract_addr: &str, query: Cw20QueryMsg) -> QuerierResult {
        match &query {
            Cw20QueryMsg::Balance {
                address,
            } => match self.balances.get(address) {
                Some(balance) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&BalanceResponse {
                        balance: Uint128::new(*balance),
                    })
                    .unwrap(),
                )),
                None => SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("[mock]: balance not set for cw20 `{}` and user `{}`", contract_addr, address),
                    request: Default::default(),
                }),
            },

            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "[mock] unsupported query: {:?}".to_string(),
                request: Default::default(),
            }),
        }
    }
}
