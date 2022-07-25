use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, Empty, StdResult, WasmMsg};
use cw_multi_test::{Contract, ContractWrapper};

use consumer::msg::ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConsumerTemplateContract(pub Addr);

impl ConsumerTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

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

pub fn create_consumer_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        consumer::contract::execute,
        consumer::contract::instantiate,
        consumer::contract::query,
    )
    .with_reply(consumer::contract::reply);

    Box::new(contract)
}
