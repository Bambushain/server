use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_165759_create_table_event::Event;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::Bamboo, EventNotification::Table))
                    .if_not_exists()
                    .col(pk_auto(EventNotification::Id))
                    .col(integer(EventNotification::EventId))
                    .col(timestamp_with_time_zone(EventNotification::Time))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::Bamboo, EventNotification::Table),
                                EventNotification::EventId,
                            )
                            .to((Schemas::Bamboo, Event::Table), Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table((Schemas::Bamboo, EventNotification::Table))
                    .name("event_notification_event_id_fkey")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(EventNotification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventNotification {
    Table,
    Id,
    EventId,
    Time,
}
