use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("CREATE SCHEMA panda_party").await?;
        db.execute_unprepared("CREATE SCHEMA final_fantasy").await?;
        db.execute_unprepared("CREATE SCHEMA authentication").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP SCHEMA panda_party").await?;
        db.execute_unprepared("DROP SCHEMA final_fantasy").await?;
        db.execute_unprepared("DROP SCHEMA authentication").await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Schemas {
    #[deprecated]
    PandaParty,
    Bamboo,
    FinalFantasy,
    Authentication,
    Grove,
}
