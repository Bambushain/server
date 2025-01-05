use crate::extension::postgres::Type;
use crate::m20220101_000001_create_schemas::Schemas;
use crate::m20230724_121111_create_table_character::Character;
use crate::sea_orm::{EnumIter, Iterable, Statement, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        let manager = SchemaManager::new(&txn);
        manager
            .create_type(
                Type::create()
                    .as_enum((Schemas::FinalFantasy, Alias::new("gatherer_job")))
                    .values(GathererJob::iter().collect::<Vec<GathererJob>>())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Schemas::FinalFantasy, Gatherer::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Gatherer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Gatherer::Job)
                            .custom(Alias::new("final_fantasy.gatherer_job"))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Gatherer::Level).string())
                    .col(ColumnDef::new(Gatherer::CharacterId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                (Schemas::FinalFantasy, Gatherer::Table),
                                Gatherer::CharacterId,
                            )
                            .to((Schemas::FinalFantasy, Character::Table), Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(Gatherer::Job)
                            .col(Gatherer::CharacterId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        txn
            .execute(Statement::from_string(manager.get_database_backend(), "INSERT INTO final_fantasy.gatherer SELECT id, job::text::final_fantasy.gatherer_job, level, character_id FROM final_fantasy.crafter WHERE job IN ('culinarian', 'miner', 'botanist', 'fisher')"))
            .await?;
        txn
            .execute(Statement::from_string(manager.get_database_backend(), "delete from final_fantasy.crafter where job in ('culinarian', 'miner', 'botanist', 'fisher')"))
            .await?;

        txn.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TYPE final_fantasy.crafter_job RENAME TO crafter_job_old",
        ))
        .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum((Schemas::FinalFantasy, Alias::new("crafter_job")))
                    .values(CrafterJob::iter().collect::<Vec<CrafterJob>>())
                    .to_owned(),
            )
            .await?;

        txn
            .execute(Statement::from_string(manager.get_database_backend(), "ALTER TABLE final_fantasy.crafter ALTER COLUMN job TYPE final_fantasy.crafter_job USING job::text::final_fantasy.crafter_job"))
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name((Schemas::FinalFantasy, CrafterJobOld::CrafterJobOld))
                    .to_owned(),
            )
            .await?;

        txn.commit().await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(Iden)]
enum CrafterJobOld {
    CrafterJobOld,
}

#[derive(Iden, EnumIter)]
enum CrafterJob {
    Carpenter,
    Blacksmith,
    Armorer,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
}

#[derive(Iden, EnumIter)]
enum GathererJob {
    Culinarian,
    Miner,
    Botanist,
    Fisher,
}

#[derive(Iden)]
pub enum Gatherer {
    Table,
    Id,
    Job,
    Level,
    CharacterId,
}
