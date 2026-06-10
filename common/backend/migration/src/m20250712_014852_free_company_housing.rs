use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230923_090306_add_field_free_company::FreeCompany;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, FreeCompanyHousing::Table))
                    .if_not_exists()
                    .col(pk_auto(FreeCompanyHousing::Id))
                    .col(custom(
                        FreeCompanyHousing::District,
                        Alias::new("final_fantasy.district"),
                    ))
                    .col(tiny_unsigned(FreeCompanyHousing::Ward))
                    .col(tiny_unsigned(FreeCompanyHousing::Plot))
                    .col(integer_uniq(FreeCompanyHousing::FreeCompanyId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, FreeCompanyHousing::Table),
                                FreeCompanyHousing::FreeCompanyId,
                            )
                            .to((Schemas::FinalFantasy, FreeCompany::Table), FreeCompany::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Schemas::FinalFantasy, FreeCompanyHousing::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FreeCompanyHousing {
    Table,
    Id,
    District,
    Ward,
    Plot,
    FreeCompanyId,
}
