use cosmwasm_std::{Binary, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terra_cosmwasm::TerraQueryWrapper;

pub type QueryMsg = Vec<QueryRequest<TerraQueryWrapper>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MultiQueryResponseItem {
    pub success: bool,
    pub data: Binary,
}

pub type MultiQueryResponse = Vec<MultiQueryResponseItem>;
