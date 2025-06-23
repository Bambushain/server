use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mailing_stmt =
            Statement::from_string(manager.get_database_backend(), "CREATE SCHEMA mailing");
        db.execute(mailing_stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table((Schemas::Mailing, Mail::Table))
                    .col(pk_uuid(Mail::Id))
                    .col(string(Mail::Subject))
                    .col(string(Mail::To))
                    .col(string(Mail::Body))
                    .col(boolean(Mail::Templated))
                    .col(integer(Mail::Status))
                    .col(string_null(Mail::ReplyTo))
                    .col(string_null(Mail::ActionLabel))
                    .col(string_null(Mail::ActionLink))
                    .col(string_null(Mail::Error))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mailing_stmt =
            Statement::from_string(manager.get_database_backend(), "DROP SCHEMA mailing");
        db.execute(mailing_stmt).await.map(|_| ())
    }
}

#[derive(DeriveIden)]
enum Mail {
    Table,
    Id,
    Subject,
    To,
    Body,
    ReplyTo,
    Templated,
    Status,
    Error,
    ActionLabel,
    ActionLink,
}

#[derive(DeriveIden)]
enum Schemas {
    Mailing,
}
