use cosmwasm_std::testing::MockQuerier;
use cosmwasm_std::{
    from_binary, from_slice, Coin, Empty, Querier, QuerierResult, QueryRequest, SystemError,
    SystemResult, WasmQuery,
};
use cw20::Cw20QueryMsg;

use super::cw20_querier::Cw20Querier;

pub(super) struct CustomQuerier {
    pub base: MockQuerier,
    pub cw20_querier: Cw20Querier,
}

impl Querier for CustomQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
                .into()
            },
        };
        self.handle_query(&request)
    }
}

impl CustomQuerier {
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        Self {
            base: MockQuerier::new(balances),
            cw20_querier: Cw20Querier::default(),
        }
    }

    pub fn set_cw20_balance(&mut self, user: &str, balance: u128) {
        self.cw20_querier.balances.insert(user.to_string(), balance);
    }

    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match request {
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr,
                msg,
            }) => {
                if let Ok(query) = from_binary::<Cw20QueryMsg>(msg) {
                    return self.cw20_querier.handle_query(&contract_addr, query);
                }

                SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("[mock] unsupported query: {:?}", request),
                    request: Default::default(),
                })
            },

            _ => self.base.handle_query(request),
        }
    }
}
