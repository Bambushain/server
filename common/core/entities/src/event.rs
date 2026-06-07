use std::str::FromStr;

use chrono::NaiveDate;
use chrono::NaiveTime as Time;
use color_art::{color, Color};
#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::user::WebUser;
use crate::{Event, Grove};
#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

fn set_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "event", schema_name = "bamboo")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub color: String,
    #[serde(default = "set_false")]
    pub is_private: bool,
    #[serde(skip)]
    pub user_id: Option<i32>,
    #[serde(skip)]
    pub grove_id: Option<i32>,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::grove::Entity",
        from = "Column::GroveId",
        to = "super::grove::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Grove,
    #[sea_orm(has_many = "super::event_reminder::Entity")]
    Reminder,
}

#[cfg(feature = "backend")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::grove::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grove.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::event_reminder::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reminder.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn set_color(&mut self, color: Color) {
        self.color = color.hex();
    }

    pub fn color(&self) -> Color {
        Color::from_str(self.color.as_str()).unwrap_or(color!(#9f2637))
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
#[cfg_attr(feature = "backend", derive(Responder))]
#[serde(rename_all = "camelCase")]
pub struct GroveEventReminder {
    #[serde(default)]
    pub id: i32,
    pub when: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
#[cfg_attr(feature = "backend", derive(Responder))]
#[serde(rename_all = "camelCase")]
pub struct GroveEvent {
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    #[serde(default)]
    pub start_time: Option<Time>,
    #[serde(default)]
    pub end_time: Option<Time>,
    pub color: String,
    pub is_private: bool,
    #[serde(default)]
    pub user: Option<WebUser>,
    #[serde(default)]
    pub grove: Option<Grove>,
    #[serde(default)]
    pub reminder: Vec<GroveEventReminder>,
}

impl GroveEvent {
    pub fn to_event(&self) -> Event {
        Event {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            start_time: self.start_time,
            end_time: self.end_time,
            color: self.color.clone(),
            is_private: self.is_private,
            user_id: self.user.clone().map(|user| user.id),
            grove_id: self.grove.clone().map(|grove| grove.id),
        }
    }
}
