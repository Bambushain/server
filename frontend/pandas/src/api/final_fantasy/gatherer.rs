use bamboo_common::core::entities::{Gatherer, GathererJob};
use leptos::server;
use server_fn::ServerFnError;

#[server(GetGatherersAction, "/pandas/gatherer")]
pub async fn get_gatherers(character_id: i32) -> Result<Vec<Gatherer>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_gatherers(auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteGathererAction, "/pandas/gatherer")]
pub async fn delete_gatherer(character_id: i32, gatherer_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_gatherer(gatherer_id, auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(CreateGathererAction, "/pandas/gatherer")]
pub async fn create_gatherer(
    character_id: i32,
    gatherer_job: GathererJob,
    level: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_gatherer(
        auth_state.user.id,
        character_id,
        Gatherer::new(character_id, gatherer_job, level),
        &db,
    )
    .await
    .map_err(ServerFnError::new)
    .map(|_| ())
}
