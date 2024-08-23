use bamboo_common::core::entities::User;
use leptos::{server, ServerFnError};

#[server(GetCurrentUser, "/pandas/current-user")]
pub async fn get_current_user() -> Result<User, ServerFnError> {
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    extract::<AuthState>().await.map(|state| state.user.clone())
}
