use bamboo_common_core_entities::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EventAction {
    #[serde(rename = "c")]
    Created(GroveEvent),
    #[serde(rename = "u")]
    Updated(GroveEvent),
    #[serde(rename = "d")]
    Deleted(GroveEvent),
}

#[cfg(feature = "backend")]
crate::impl_nats!(EventAction);
