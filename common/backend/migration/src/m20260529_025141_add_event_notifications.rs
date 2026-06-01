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
                    .table((Schemas::Bamboo, EventReminder::Table))
                    .if_not_exists()
                    .col(pk_auto(EventReminder::Id))
                    .col(integer(EventReminder::EventId))
                    .col(timestamp_with_time_zone(EventReminder::Time))
                    .col(boolean(EventReminder::Notified).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::Bamboo, EventReminder::Table),
                                EventReminder::EventId,
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
                    .table((Schemas::Bamboo, EventReminder::Table))
                    .name("event_notification_event_id_fkey")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(EventReminder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventReminder {
    Table,
    Id,
    EventId,
    Time,
    Notified,
}
