use actix_web::web::Bytes;
use actix_web::{delete, get, post, put, web};
use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::{DbConnection, MinioService, TotpService};
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};
use bamboo_common::core::entities::user::BambooUser;

#[put("/api/my/password", wrap = "authenticate!()")]
pub async fn change_password(
    body: Option<web::Json<ChangeMyPassword>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "user")?;

    dbal::change_my_password(
        authentication.user.id,
        body.old_password.clone(),
        body.new_password.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
    .map_err(|err| err.into())
}

#[put("/api/my/profile", wrap = "authenticate!()")]
pub async fn update_profile(
    body: Option<web::Json<UpdateProfile>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "user")?;

    dbal::update_my_profile(
        authentication.user.id,
        body.email.clone(),
        body.display_name.clone(),
        body.discord_name.clone(),
        &db,
    )
    .await
    .map(|_| no_content!())
}

#[post("/api/my/totp", wrap = "authenticate!()")]
pub async fn enable_totp(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<TotpQrCode> {
    let totp_service = TotpService::new();
    let qr_data = totp_service
        .get_totp_qr(authentication.user.clone(), &db)
        .await?;

    Ok(ok!(qr_data))
}

#[put("/api/my/totp/validate", wrap = "authenticate!()")]
pub async fn validate_totp(
    body: Option<web::Json<ValidateTotp>>,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    if authentication.user.totp_validated.unwrap_or(false) {
        Err(BambooError::invalid_data("user", "Already validated"))
    } else {
        let body = check_missing_fields!(body, "user")?;

        dbal::validate_my_totp(
            authentication.user.id,
            body.password.clone(),
            body.code.clone(),
            &db,
        )
        .await
        .map(|data| {
            if data {
                Ok(no_content!())
            } else {
                Err(BambooError::insufficient_rights(
                    "user",
                    "The code is invalid",
                ))
            }
        })?
    }
}

#[get("/api/my/profile", wrap = "authenticate!()")]
pub async fn get_profile(authentication: Authentication) -> BambooApiResult<BambooUser> {
    Ok(ok!(authentication.user.clone()))
}

#[delete("/api/my/totp", wrap = "authenticate!()")]
pub async fn disable_totp(
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::disable_my_totp(authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

#[delete("/api/my", wrap = "authenticate!()")]
pub async fn leave(authentication: Authentication, db: DbConnection) -> BambooApiResponseResult {
    dbal::delete_user(authentication.user.id, &db)
        .await
        .map(|_| no_content!())
}

#[put("/api/my/picture", wrap = "authenticate!()")]
pub async fn upload_profile_picture(
    authentication: Authentication,
    minio: MinioService,
    body: Bytes,
) -> BambooApiResponseResult {
    minio
        .upload_profile_picture(authentication.user.id, &body)
        .await
        .map(|_| no_content!())
}
