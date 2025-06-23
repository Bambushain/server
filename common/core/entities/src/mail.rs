use bamboo_common_backend_macros::Responder;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

pub const MAIL_STATUS_OPEN: i32 = 0;
pub const MAIL_STATUS_SENDING: i32 = 1;
pub const MAIL_STATUS_FAILED: i32 = 2;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "mail", schema_name = "mailing")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: Uuid,
    pub subject: String,
    pub to: String,
    pub status: i32,
    pub body: String,
    pub error: Option<String>,
    pub reply_to: Option<String>,
    pub templated: bool,
    pub action_label: Option<String>,
    pub action_link: Option<String>,
}

impl Model {
    pub fn new(
        subject: impl Into<String>,
        to: impl Into<String>,
        body: impl Into<String>,
        reply_to: Option<impl Into<String>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject: subject.into(),
            to: to.into(),
            body: body.into(),
            reply_to: reply_to.map(|reply_to| reply_to.into()),
            templated: false,
            action_label: None,
            action_link: None,
            error: None,
            status: MAIL_STATUS_OPEN,
        }
    }

    pub fn new_templated(
        subject: impl Into<String>,
        to: impl Into<String>,
        body: impl Into<String>,
        reply_to: Option<impl Into<String>>,
        action_label: impl Into<String>,
        action_link: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject: subject.into(),
            to: to.into(),
            body: body.into(),
            reply_to: reply_to.map(|reply_to| reply_to.into()),
            templated: true,
            action_label: Some(action_label.into()),
            action_link: Some(action_link.into()),
            error: None,
            status: MAIL_STATUS_OPEN,
        }
    }
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}
