use leptos::prelude::*;

#[server(SubmitSupportRequestAction, "/pandas/support")]
pub async fn submit_request(message: String, subject: String) -> Result<(), ServerFnError> {
    use bamboo_common::backend::mailing;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let auth_state = extract::<AuthState>().await?;
    let db = extract::<DbConnection>().await?;

    mailing::send_support_request(message, subject, auth_state.user.email.clone(), &db).await;

    Ok(())
}
