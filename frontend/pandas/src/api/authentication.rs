use bamboo_common::core::entities::user::BambooUser;
use leptos::prelude::{server, ServerFnError};

#[server(GetCurrentUser, "/pandas/current-user")]
pub async fn get_current_user() -> Result<BambooUser, ServerFnError> {
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    extract::<AuthState>()
        .await
        .map(|state| state.user.clone())
        .map_err(ServerFnError::new)
}

#[server(LogoutAction, "/pandas/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::delete_token(auth_state.token.clone(), &db)
        .await
        .map(|_| ())
        .map_err(ServerFnError::new)
}
