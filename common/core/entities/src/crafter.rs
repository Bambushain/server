#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use bamboo_common_backend_macros::*;
#[cfg(feature = "frontend")]
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug, Eq, PartialEq, Clone, Default, Copy, Hash)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveActiveEnum),
    sea_orm(
        rs_type = "String",
        db_type = "Enum",
        enum_name = "final_fantasy\".\"crafter_job"
    )
)]
pub enum CrafterJob {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "carpenter"))]
    #[serde(rename = "carpenter")]
    Carpenter,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "blacksmith"))]
    #[serde(rename = "blacksmith")]
    Blacksmith,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "armorer"))]
    #[serde(rename = "armorer")]
    Armorer,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "goldsmith"))]
    #[serde(rename = "goldsmith")]
    Goldsmith,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "leatherworker"))]
    #[serde(rename = "leatherworker")]
    Leatherworker,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "weaver"))]
    #[serde(rename = "weaver")]
    Weaver,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "alchemist"))]
    #[serde(rename = "alchemist")]
    Alchemist,
}

impl CrafterJob {
    pub fn get_file_name(self) -> String {
        match self {
            CrafterJob::Carpenter => "carpenter.webp",
            CrafterJob::Blacksmith => "blacksmith.webp",
            CrafterJob::Armorer => "armorer.webp",
            CrafterJob::Goldsmith => "goldsmith.webp",
            CrafterJob::Leatherworker => "leatherworker.webp",
            CrafterJob::Weaver => "weaver.webp",
            CrafterJob::Alchemist => "alchemist.webp",
        }
        .to_string()
    }

    pub fn get_job_name(self) -> String {
        match self {
            CrafterJob::Carpenter => "Carpenter",
            CrafterJob::Blacksmith => "Blacksmith",
            CrafterJob::Armorer => "Armorer",
            CrafterJob::Goldsmith => "Goldsmith",
            CrafterJob::Leatherworker => "Leatherworker",
            CrafterJob::Weaver => "Weaver",
            CrafterJob::Alchemist => "Alchemist",
        }
        .to_string()
    }
}

impl PartialOrd for CrafterJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrafterJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_job_name().cmp(&other.get_job_name())
    }
}

impl Display for CrafterJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CrafterJob::Carpenter => "Zimmerer",
            CrafterJob::Blacksmith => "Grobschmied",
            CrafterJob::Armorer => "Plattner",
            CrafterJob::Goldsmith => "Goldschmied",
            CrafterJob::Leatherworker => "Gerber",
            CrafterJob::Weaver => "Weber",
            CrafterJob::Alchemist => "Alchemist",
        })
    }
}

impl From<String> for CrafterJob {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "carpenter" => CrafterJob::Carpenter,
            "blacksmith" => CrafterJob::Blacksmith,
            "armorer" => CrafterJob::Armorer,
            "goldsmith" => CrafterJob::Goldsmith,
            "leatherworker" => CrafterJob::Leatherworker,
            "weaver" => CrafterJob::Weaver,
            "alchemist" => CrafterJob::Alchemist,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "crafter", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub job: CrafterJob,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(skip)]
    pub character_id: i32,
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.job.cmp(&other.job)
    }
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Character,
}

#[cfg(feature = "backend")]
impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(character_id: i32, job: CrafterJob, level: String) -> Self {
        Self {
            id: i32::default(),
            job,
            level: Some(level),
            character_id,
        }
    }
}
