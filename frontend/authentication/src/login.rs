use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResult {
    pub requires_two_factor: bool,
    pub login_success: bool,
}

#[server(LoginAction, "/authentication/login")]
pub async fn login(
    email: String,
    password: String,
    two_factor_code: Option<String>,
) -> Result<LoginResult, ServerFnError> {
    use actix_web::cookie::Cookie;
    use actix_web::http::header;
    use actix_web::http::header::HeaderValue;
    use bamboo_common::backend::actix::cookie;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::error::BambooError;
    use leptos_actix::{extract, ResponseOptions};

    let db: DbConnection = extract().await?;

    let res = dbal::validate_auth(&email, &password, two_factor_code, &db)
        .await
        .map_err(ServerFnError::new)?;

    if !res.requires_two_factor_code {
        let response = expect_context::<ResponseOptions>();

        let token = dbal::create_token(&email, &db).await.map_err(|err| {
            log::error!("Failed to login {err}");
            BambooError::unauthorized("user", "Login data is invalid")
        })?;

        let cookie = Cookie::build(cookie::BAMBOO_AUTH_COOKIE, token.token.clone())
            .path("/")
            .http_only(true)
            .finish();
        if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
            response.insert_header(header::SET_COOKIE, cookie);
        }

        Ok(LoginResult {
            requires_two_factor: false,
            login_success: true,
        })
    } else {
        Ok(LoginResult {
            requires_two_factor: true,
            login_success: false,
        })
    }
}

#[server(ForgotPasswordAction, "/authentication/forgot-password")]
pub async fn forgot_password(email: String) -> Result<(), ServerFnError> {
    use bamboo_common::backend::mailing::enqueue_forgot_password_mail;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let db: DbConnection = extract().await?;

    enqueue_forgot_password_mail(&email, &db).await;

    Ok(())
}

#[server(ResetPasswordAction, "/authentication/forgot-password")]
pub async fn reset_password(
    email: String,
    token: String,
    password: String,
) -> Result<bool, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let db: DbConnection = extract().await?;

    dbal::reset_password_by_token(&email, &token, &password, &db)
        .await
        .map(|_| true)
        .map_err(ServerFnError::new)
}
