use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(manager.get_database_backend(),"SELECT setval('final_fantasy.gatherer_id_seq', (select id from final_fantasy.gatherer order by id desc limit 1) + 1, true)"))
            .await
            .map(|_| ())?;

        manager
            .get_connection()
            .execute(Statement::from_string(manager.get_database_backend(),"SELECT setval('final_fantasy.crafter_id_seq', (select id from final_fantasy.crafter order by id desc limit 1) + 1, true)"))
            .await
            .map(|_| ())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
