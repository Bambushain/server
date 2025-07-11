use crate::m20220101_000001_create_schemas::Schemas;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Bamboo, Event::Table))
                    .add_column(ColumnDef::new(Event::StartTime).time())
                    .add_column(ColumnDef::new(Event::EndTime).time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Schemas::Bamboo, Event::Table))
                    .drop_column(Event::StartTime)
                    .drop_column(Event::EndTime)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    StartTime,
    EndTime,
}
