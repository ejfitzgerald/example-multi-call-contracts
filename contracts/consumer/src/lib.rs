pub mod contract;
mod error;
pub mod msg;

pub use crate::error::ContractError;
use cosmwasm_std::Event;

pub fn extract_value_from_events(events: &[Event], event_type: &str, key: &str) -> Option<String> {
    for item in events {
        if item.ty != event_type {
            continue;
        }

        for attr in &item.attributes {
            if attr.key == key {
                return Some(attr.value.clone());
            }
        }
    }

    None
}
