use std::fmt::{Display, Formatter};
use bamboo_common_core_entities::event::GroveEventNotification;
use bamboo_common_core_entities::{user, Grove, GroveEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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
    UserPasswordChange(user::BambooUser),
    #[serde(rename = "uad")]
    UserAccountDelete(user::BambooUser),
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Notification::EventReminder(_, _) => "EventReminder",
            Notification::GroveJoin(_, _) => "GroveJoin",
            Notification::GroveBan(_, _) => "GroveBan",
            Notification::GroveUnban(_, _) => "GroveUnban",
            Notification::GroveInviteEnable(_) => "GroveInviteEnable",
            Notification::GroveInviteDisable(_) => "GroveInviteDisable",
            Notification::GroveModChange(_) => "GroveModChange",
            Notification::GroveDelete(_, _) => "GroveDelete",
            Notification::UserPasswordChange(_) => "UserPasswordChange",
            Notification::UserAccountDelete(_) => "UserAccountDelete"
        })
    }
}

#[cfg(feature = "backend")]
crate::impl_nats!(Notification);
