use bamboo_common::core::entities::Grove;
use leptos::{server, ServerFnError};

#[server(GetGrovesAction, "/pandas/groves")]
pub async fn get_all_groves() -> Result<Vec<Grove>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::get_groves(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(GetGroveAction, "/pandas/grove")]
pub async fn get_grove(id: i32) -> Result<Grove, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::get_grove(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}
