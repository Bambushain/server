use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_schemas::Schemas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("CREATE SCHEMA grove").await?;

        manager
            .create_table(
                Table::create()
                    .table((Schemas::Grove, Grove::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grove::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Grove::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Grove::IsSuspended)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Grove::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP SCHEMA grove").await?;

        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::Grove, Grove::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Grove {
    Table,
    Id,
    Name,
    IsSuspended,
    IsEnabled,
}
