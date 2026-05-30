use bamboo_common_core_entities::event::GroveEventNotification;
use bamboo_common_core_entities::{user, Grove, GroveEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Notification {
    #[serde(rename = "er")]
    EventReminder(GroveEvent, GroveEventNotification),
    #[serde(rename = "gj")]
    GroveJoin(Grove, user::GroveUser),
    #[serde(rename = "gb")]
    GroveBan(Grove, user::GroveUser),
    #[serde(rename = "gu")]
    GroveUnban(Grove, user::GroveUser),
    #[serde(rename = "gie")]
    GroveInviteEnable(Grove),
    #[serde(rename = "gid")]
    GroveInviteDisable(Grove),
    #[serde(rename = "gmc")]
    GroveModChange(Grove),
    #[serde(rename = "gd")]
    GroveDelete(Grove, Vec<user::GroveUser>),
    #[serde(rename = "upc")]
    UserPasswordChange(user::WebUser),
    #[serde(rename = "uad")]
    UserAccountDelete(user::WebUser),
}

#[cfg(feature = "backend")]
crate::impl_nats!(Notification);
