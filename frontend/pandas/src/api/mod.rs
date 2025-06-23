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
use server_fn::ServerFnError;
use std::error::Error;
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct BambooCodeError {
    pub code: BambooErrorCode,
}

impl Display for BambooCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.code))
    }
}

impl Error for BambooCodeError {}

impl From<BambooError> for BambooCodeError {
    fn from(value: BambooError) -> Self {
        Self {
            code: value.error_type,
        }
    }
}

pub fn bamboo_error_to_serverfn_error(err: BambooError) -> ServerFnError<BambooCodeError> {
    ServerFnError::WrappedServerError(err.into())
}

impl FromStr for BambooCodeError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { code: s.into() })
    }
}
