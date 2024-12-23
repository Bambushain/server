use bamboo_common::core::entities::{Fighter, FighterJob};
use leptos::server;
use server_fn::ServerFnError;

#[server(GetFightersAction, "/pandas/fighter")]
pub async fn get_fighters(character_id: i32) -> Result<Vec<Fighter>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_fighters(auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteFighterAction, "/pandas/fighter")]
pub async fn delete_fighter(character_id: i32, fighter_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_fighter(fighter_id, auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(CreateFighterAction, "/pandas/fighter")]
pub async fn create_fighter(
    character_id: i32,
    fighter_job: FighterJob,
    level: String,
    gear_score: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_fighter(
        auth_state.user.id,
        character_id,
        Fighter::new(character_id, fighter_job, level, gear_score),
        &db,
    )
    .await
    .map_err(ServerFnError::new)
    .map(|_| ())
}

#[server(EditFighterAction, "/pandas/fighter")]
pub async fn edit_fighter(
    character_id: i32,
    fighter_job: FighterJob,
    level: String,
    gear_score: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_fighter(
        auth_state.user.id,
        character_id,
        Fighter::new(character_id, fighter_job, level, gear_score),
        &db,
    )
    .await
    .map_err(ServerFnError::new)
    .map(|_| ())
}
