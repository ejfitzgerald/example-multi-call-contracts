#[cfg(test)]
mod consumer_template;
#[cfg(test)]
mod oracle_template;

#[cfg(test)]
mod tests {
    use crate::consumer_template::{create_consumer_template, ConsumerTemplateContract};
    use crate::oracle_template::{create_oracle_template, OracleTemplateContract};
    use consumer::extract_value_from_events;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cw_multi_test::{App, AppBuilder, Executor};

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, OracleTemplateContract, ConsumerTemplateContract) {
        let mut app = mock_app();

        let oracle_id = app.store_code(create_oracle_template());
        let oracle_address = app
            .instantiate_contract(
                oracle_id,
                Addr::unchecked(ADMIN),
                &oracle::msg::InstantiateMsg {},
                &[],
                "oracle",
                None,
            )
            .unwrap();

        let oracle_contract = OracleTemplateContract(oracle_address);

        let consumer_id = app.store_code(create_consumer_template());
        let consumer_address = app
            .instantiate_contract(
                consumer_id,
                Addr::unchecked(ADMIN),
                &consumer::msg::InstantiateMsg {},
                &[],
                "consumer",
                None,
            )
            .unwrap();

        let consumer_contract = ConsumerTemplateContract(consumer_address);

        (app, oracle_contract, consumer_contract)
    }

    #[test]
    fn basic_e2e_test() {
        let (mut app, oracle_contract, consumer) = proper_instantiate();

        let msg = consumer::msg::ExecuteMsg::Consume {
            oracle_address: oracle_contract.addr(),
        };
        let cosmos_msg = consumer.call(msg).unwrap();
        let result = app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

        let oracle_value = extract_value_from_events(&result.events, "wasm", "oracle-value");
        let reply_id = extract_value_from_events(&result.events, "wasm", "reply-id");

        // for event in &result.events {
        //     for attr in &event.attributes {
        //         println!("- {} {} {}", event.ty, attr.key, attr.value);
        //     }
        // }

        assert_eq!(oracle_value, Some("42".to_string()));
        assert_eq!(reply_id, Some("14".to_string()));
    }
}
