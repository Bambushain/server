use bamboo_common::core::entities::FreeCompany;
use leptos::server;
use server_fn::ServerFnError;

#[server(GetFreeCompanies, "/pandas/free-company")]
pub async fn get_free_companies() -> Result<Vec<FreeCompany>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_free_companies(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}
