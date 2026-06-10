use crate::m20220101_000001_create_schemas::Schemas;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .add_column(string_null(Character::Datacenter))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::FinalFantasy, Character::Table))
                    .drop_column(Character::Datacenter)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Character {
    Table,
    Datacenter,
}
