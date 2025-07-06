use bamboo_common::backend::dbal;
use bamboo_common::backend::migration::{sea_orm, Migrator, MigratorTrait};

async fn setup_google_playstore_grove(
    user_id: i32,
    db: &sea_orm::DatabaseConnection,
) -> std::io::Result<()> {
    if !dbal::grove_exists_by_name("Google", db)
        .await
        .map_err(std::io::Error::other)?
    {
        dbal::create_grove("Google", false, user_id, db)
            .await
            .map_err(std::io::Error::other)
            .map(|_| ())
    } else {
        Ok(())
    }
}

async fn setup_google_playstore_user(db: &sea_orm::DatabaseConnection) -> std::io::Result<()> {
    let email = "playstore@google.bambushain";
    let password = "NkWHoLDmzg4aVEx";

    if let Ok(user) = dbal::get_user_by_email_or_username(email, db).await {
        dbal::set_password(user.id, password, db)
            .await
            .map_err(std::io::Error::other)
            .map(|_| ())
    } else {
        let user = dbal::create_user(
            bamboo_common::core::entities::User::new(
                email.to_string(),
                "Google Playstore".to_string(),
                "google".to_string(),
            ),
            password,
            db,
        )
        .await
        .map_err(std::io::Error::other)?;
        setup_google_playstore_grove(user.id, db).await
    }
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Open the bamboo grove");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;

    let migrations = Migrator::get_pending_migrations(&db)
        .await
        .map_err(std::io::Error::other)?;
    log::info!("Running {} migrations", migrations.len());

    Migrator::up(&db, None)
        .await
        .map_err(std::io::Error::other)?;
    log::info!("Successfully migrated database");

    setup_google_playstore_user(&db).await
}
