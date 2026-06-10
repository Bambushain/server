use bamboo_common_core_entities::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use strum::EnumIter;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EventAction {
    #[serde(rename = "c")]
    Created(GroveEvent),
    #[serde(rename = "u")]
    Updated(GroveEvent),
    #[serde(rename = "d")]
    Deleted(GroveEvent),
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumIter)]
pub enum EventType {
    Created,
    Updated,
    Deleted,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        })
    }
}

impl FromStr for EventType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "created" => Ok(EventType::Created),
            "updated" => Ok(EventType::Updated),
            "deleted" => Ok(EventType::Deleted),
            _ => Err("Failed to parse".into()),
        }
    }
}

#[cfg(feature = "backend")]
crate::impl_nats!(EventAction);
