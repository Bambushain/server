#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::HousingDistrict;
#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "free_company_housing", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub district: HousingDistrict,
    pub ward: i16,
    pub plot: i16,
    #[serde(skip)]
    pub free_company_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.district
            .cmp(&other.district)
            .then(self.ward.cmp(&other.ward))
            .then(self.plot.cmp(&other.plot))
    }
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::free_company::Entity",
        from = "Column::FreeCompanyId",
        to = "super::free_company::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    FreeCompany,
}

#[cfg(feature = "backend")]
impl Related<super::free_company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FreeCompany.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(free_company_id: i32, district: HousingDistrict, ward: i16, plot: i16) -> Self {
        Self {
            id: i32::default(),
            district,
            ward,
            plot,
            free_company_id,
        }
    }
}
