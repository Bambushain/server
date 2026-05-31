use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121011_create_table_user::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FirebaseToken::Table)
                    .if_not_exists()
                    .col(pk_auto(FirebaseToken::Id))
                    .col(string(FirebaseToken::Token))
                    .col(string(FirebaseToken::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from((Schemas::Authentication, User::Table), User::Id)
                            .to((Schemas::Authentication, FirebaseToken::Table), FirebaseToken::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FirebaseToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FirebaseToken {
    Table,
    Id,
    Token,
    UserId,
}
