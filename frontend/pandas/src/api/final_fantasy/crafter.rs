use bamboo_common::core::entities::{Crafter, CrafterJob};
use leptos::server;
use server_fn::ServerFnError;

#[server(GetCraftersAction, "/pandas/crafter")]
pub async fn get_crafters(character_id: i32) -> Result<Vec<Crafter>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_crafters(auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteCrafterAction, "/pandas/crafter")]
pub async fn delete_crafter(character_id: i32, crafter_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_crafter(crafter_id, auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(CreateCrafterAction, "/pandas/crafter")]
pub async fn create_crafter(
    character_id: i32,
    crafter_job: CrafterJob,
    level: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_crafter(
        auth_state.user.id,
        character_id,
        Crafter::new(character_id, crafter_job, level),
        &db,
    )
    .await
    .map_err(ServerFnError::new)
    .map(|_| ())
}
