use leptos::prelude::*;

#[server(SubmitSupportRequestAction, "/pandas/support")]
pub async fn submit_request(message: String, subject: String) -> Result<(), ServerFnError> {
    use bamboo_common::backend::mailing;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let auth_state = extract::<AuthState>().await?;

    mailing::send_support_request(message, subject, auth_state.user.email.clone()).await;

    Ok(())
}
