use bamboo_common::core::entities::FreeCompanyWithCharacterCount;
use leptos::server;
use server_fn::ServerFnError;

#[server(GetFreeCompaniesAction, "/pandas/free-company")]
pub async fn get_free_companies() -> Result<Vec<FreeCompanyWithCharacterCount>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_free_companies(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(CreateFreeCompanyAction, "/pandas/free-company")]
pub async fn create_free_company(name: String) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_free_company(auth_state.user.id, &name, &db)
        .await
        .map_err(ServerFnError::from)
        .map(|_| ())
}

#[server(EditFreeCompanyAction, "/pandas/free-company")]
pub async fn edit_free_company(id: i32, name: String) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::update_free_company(id, auth_state.user.id, &name, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(DeleteFreeCompanyAction, "/pandas/free-company")]
pub async fn delete_free_company(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_free_company(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}
