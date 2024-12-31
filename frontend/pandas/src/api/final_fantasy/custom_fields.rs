use bamboo_common::core::entities::CustomCharacterField;
use leptos::server;
use server_fn::ServerFnError;

#[server(GetCustomFieldsAction, "/pandas/custom-field")]
pub async fn get_custom_fields() -> Result<Vec<CustomCharacterField>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_custom_fields(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}
