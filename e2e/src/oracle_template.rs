use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, Empty, StdResult, WasmMsg};
use cw_multi_test::{Contract, ContractWrapper};

use oracle::msg::ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTemplateContract(pub Addr);

impl OracleTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(dead_code)]
    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn create_oracle_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        oracle::contract::execute,
        oracle::contract::instantiate,
        oracle::contract::query,
    );
    Box::new(contract)
}
