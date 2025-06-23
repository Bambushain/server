mod authentication;
mod event;
mod final_fantasy;
mod groves;
mod pandas;
mod profile;
mod support;

pub use authentication::*;
use bamboo_common::core::error::{BambooError, BambooErrorCode};
pub use event::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;
use server_fn::error::{FromServerFnError, ServerFnErrorErr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod ff {
    pub use crate::api::final_fantasy::characters::*;
    pub use crate::api::final_fantasy::crafter::*;
    pub use crate::api::final_fantasy::custom_fields::*;
    pub use crate::api::final_fantasy::fighter::*;
    pub use crate::api::final_fantasy::free_companies::*;
    pub use crate::api::final_fantasy::gatherer::*;
    pub use crate::api::final_fantasy::housing::*;
}
pub use groves::*;
pub use pandas::*;
pub use profile::*;
pub use support::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum BambooCodeError {
    ServerFn(ServerFnErrorErr),
    Crypto,
    Database,
    ExistsAlready,
    InsufficientRights,
    InvalidData,
    Io,
    Mailing,
    NotFound,
    Serialization,
    Unauthorized,
    Unknown,
    Validation,
}

impl FromServerFnError for BambooCodeError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        BambooCodeError::ServerFn(value)
    }
}

impl FromStr for BambooCodeError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for BambooCodeError {
    fn from(s: &str) -> Self {
        match s {
            "Crypto" => BambooCodeError::Crypto,
            "Database" => BambooCodeError::Database,
            "ExistsAlready" => BambooCodeError::ExistsAlready,
            "InsufficientRights" => BambooCodeError::InsufficientRights,
            "InvalidData" => BambooCodeError::InvalidData,
            "Io" => BambooCodeError::Io,
            "Mailing" => BambooCodeError::Mailing,
            "NotFound" => BambooCodeError::NotFound,
            "Serialization" => BambooCodeError::Serialization,
            "Unauthorized" => BambooCodeError::Unauthorized,
            "Validation" => BambooCodeError::Validation,
            _ => BambooCodeError::Unknown,
        }
    }
}

impl From<BambooErrorCode> for BambooCodeError {
    fn from(s: BambooErrorCode) -> Self {
        match s {
            BambooErrorCode::Crypto => BambooCodeError::Crypto,
            BambooErrorCode::Database => BambooCodeError::Database,
            BambooErrorCode::ExistsAlready => BambooCodeError::ExistsAlready,
            BambooErrorCode::InsufficientRights => BambooCodeError::InsufficientRights,
            BambooErrorCode::InvalidData => BambooCodeError::InvalidData,
            BambooErrorCode::Io => BambooCodeError::Io,
            BambooErrorCode::Mailing => BambooCodeError::Mailing,
            BambooErrorCode::NotFound => BambooCodeError::NotFound,
            BambooErrorCode::Serialization => BambooCodeError::Serialization,
            BambooErrorCode::Unauthorized => BambooCodeError::Unauthorized,
            BambooErrorCode::Validation => BambooCodeError::Validation,
            _ => BambooCodeError::Unknown,
        }
    }
}

impl Display for BambooCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

pub fn bamboo_error_to_serverfn_error(err: BambooError) -> BambooCodeError {
    err.error_type.into()
}
