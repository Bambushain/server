use bamboo_common::core::entities::Character;
use leptos::server;
use server_fn::ServerFnError;

#[server(GetCharacterAction, "/pandas/characters")]
pub async fn get_characters() -> Result<Vec<Character>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

    dbal::get_characters(user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteCharacterAction, "/pandas/characters")]
pub async fn delete_character(character_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_character(character_id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}
