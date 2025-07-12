#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
#[cfg(feature = "backend")]
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::HousingDistrict;
#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "free_company", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub name: String,
    #[serde(skip)]
    #[cfg(feature = "backend")]
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Clone, Default)]
#[cfg_attr(feature = "backend", derive(Responder, FromQueryResult))]
#[serde(rename_all = "camelCase")]
pub struct FreeCompanyWithCharacterCountAndHousing {
    pub id: i32,
    pub name: String,
    pub character_count: i64,
    pub district: Option<HousingDistrict>,
    pub ward: Option<i16>,
    pub plot: Option<i16>,
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
        belongs_to = "super::character::Entity",
        from = "Column::Id",
        to = "super::character::Column::FreeCompanyId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Character,
    #[sea_orm(
        belongs_to = "super::free_company_housing::Entity",
        from = "Column::Id",
        to = "super::free_company_housing::Column::FreeCompanyId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Housing,
}

#[cfg(feature = "backend")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::free_company_housing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Housing.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            id: i32::default(),
            name,
            #[cfg(feature = "backend")]
            user_id: i32::default(),
        }
    }
}
