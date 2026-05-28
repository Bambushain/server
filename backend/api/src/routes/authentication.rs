use actix_web::{delete, post, web, HttpResponse};
use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};
use bamboo_common::backend::mailing::enqueue_forgot_password_mail;

#[post("/api/login")]
pub async fn login(body: Option<web::Json<Login>>, db: DbConnection) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?;

    let data = dbal::validate_auth(
        &body.email,
        &body.password,
        body.two_factor_code.clone(),
        &db,
    )
        .await
        .map_err(|err| {
            log::error!("Failed to login {err}");
            BambooError::unauthorized("user", "Login data is invalid")
        })?;

    if data.requires_two_factor_code {
        Ok(no_content!())
    } else {
        dbal::create_token(&body.email, &db)
            .await
            .map_err(|err| {
                log::error!("Failed to login {err}");
                BambooError::unauthorized("user", "Login data is invalid")
            })
            .map(|data| list!(data.clone()))
    }
}

#[post("/api/forgot-password")]
pub async fn forgot_password(
    body: Option<web::Json<ForgotPassword>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?.into_inner();
    enqueue_forgot_password_mail(&body.email, &db).await;

    Ok(no_content!())
}

#[post("/api/reset-password")]
pub async fn reset_password(
    body: Option<web::Json<ResetPassword>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "authentication")?.into_inner();

    dbal::reset_password_by_token(&body.email, &body.token, &body.password, &db)
        .await
        .map(|_| no_content!())
}

#[delete("/api/login", wrap = "authenticate!()")]
pub async fn logout(auth: Authentication, db: DbConnection) -> HttpResponse {
    let _ = dbal::delete_token(auth.token.clone(), &db).await;

    no_content!()
}
