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
        enum_name = "final_fantasy.gatherer_job"
    )
)]
pub enum GathererJob {
    #[default]
    #[cfg_attr(feature = "backend", sea_orm(string_value = "culinarian"))]
    Culinarian,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "miner"))]
    Miner,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "botanist"))]
    Botanist,
    #[cfg_attr(feature = "backend", sea_orm(string_value = "fisher"))]
    Fisher,
}

impl GathererJob {
    pub fn get_file_name(self) -> String {
        match self {
            GathererJob::Culinarian => "culinarian.webp",
            GathererJob::Miner => "miner.webp",
            GathererJob::Botanist => "botanist.webp",
            GathererJob::Fisher => "fisher.webp",
        }
        .to_string()
    }

    pub fn get_job_name(self) -> String {
        match self {
            GathererJob::Culinarian => "Culinarian",
            GathererJob::Miner => "Miner",
            GathererJob::Botanist => "Botanist",
            GathererJob::Fisher => "Fisher",
        }
        .to_string()
    }
}

impl PartialOrd for GathererJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GathererJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_job_name().cmp(&other.get_job_name())
    }
}

impl Display for GathererJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GathererJob::Culinarian => "Gourmet",
            GathererJob::Miner => "Minenarbeiter",
            GathererJob::Botanist => "Gärtner",
            GathererJob::Fisher => "Fischer",
        })
    }
}

impl From<String> for GathererJob {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "culinarian" => GathererJob::Culinarian,
            "miner" => GathererJob::Miner,
            "botanist" => GathererJob::Botanist,
            "fisher" => GathererJob::Fisher,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(
    feature = "backend",
    derive(DeriveEntityModel, Responder),
    sea_orm(table_name = "gatherer", schema_name = "final_fantasy")
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    #[serde(default)]
    pub id: i32,
    pub job: GathererJob,
    #[serde(default)]
    pub level: Option<String>,
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
    pub fn new(character_id: i32, job: GathererJob, level: String) -> Self {
        Self {
            id: i32::default(),
            job,
            level: Some(level),
            character_id,
        }
    }
}
