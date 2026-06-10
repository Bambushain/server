use bamboo_common::core::entities::{BambooUser, TotpQrCode};
use leptos::prelude::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[server(GetProfileAction, "/pandas/profile")]
pub async fn get_profile() -> Result<BambooUser, ServerFnError> {
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let auth_state = extract::<AuthState>().await?;
    let user = auth_state.user.clone();

    Ok(user)
}

#[server(DeleteProfileAction, "/pandas/profile/delete")]
pub async fn delete_profile() -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::delete_user(auth_state.user.id, &db)
        .await
        .map(|_| ())
        .map_err(ServerFnError::new)
}

#[server(DisableTotpAction, "/pandas/profile/totp/disable")]
pub async fn disable_totp() -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::disable_my_totp(auth_state.user.id, &db)
        .await
        .map(|_| ())
        .map_err(ServerFnError::new)
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct UpdateProfileResult {
    pub success: bool,
    pub message: String,
    pub header: String,
    pub user: Option<BambooUser>,
}

#[server(UpdateProfileAction, "/pandas/profile/update")]
pub async fn update_profile(
    display_name: String,
    email: String,
    discord_name: String,
) -> Result<UpdateProfileResult, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::error::BambooErrorCode;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    Ok(
        match dbal::update_my_profile(
            auth_state.user.id,
            &email,
            &display_name,
            &discord_name,
            &db,
        )
            .await
        {
            Ok(_) => UpdateProfileResult {
                success: true,
                user: dbal::get_user(auth_state.user.id, &db).await.ok(),
                ..Default::default()
            },
            Err(err) => match err.error_type {
                BambooErrorCode::ExistsAlready => UpdateProfileResult {
                    success: false,
                    message: "Die Email oder der Name ist leider schon vergeben".to_string(),
                    header: "Leider schon vergeben".to_string(),
                    user: None,
                },
                BambooErrorCode::NotFound => UpdateProfileResult {
                    success: false,
                    message: "Bitte versuch es erneut um einen Fehler auszuschließen".to_string(),
                    header: "Du wurdest scheinbar gelöscht".to_string(),
                    user: None,
                },
                _ => UpdateProfileResult {
                    success: false,
                    message: "Dein Profil konnte leider nicht gespeichert werden".to_string(),
                    header: "Fehler beim Speichern".to_string(),
                    user: None,
                },
            },
        },
    )
}

#[server(UpdateProfilePictureAction, "/pandas/profile/update")]
pub async fn update_profile_picture(data: Vec<u8>) -> Result<(), ServerFnError> {
    use bamboo_common::backend::services::MinioClient;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let auth_state = extract::<AuthState>().await?;
    let minio_client = MinioClient::new().map_err(ServerFnError::new)?;
    minio_client
        .upload_profile_picture(auth_state.user.id, &data)
        .await
        .map_err(ServerFnError::from)
}

#[server(GetQrCodeAction, "/pandas/profile/qr")]
pub async fn get_qr_code() -> Result<TotpQrCode, ServerFnError> {
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::backend::services::TotpService;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    let totp_service = TotpService::new();

    totp_service
        .get_totp_qr(auth_state.user.clone(), &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(ValidateTotpAction, "/pandas/profile/totp")]
pub async fn validate_totp(password: String, totp_code: String) -> Result<bool, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::validate_my_totp(auth_state.user.id, &password, &totp_code, &db)
        .await
        .map_err(ServerFnError::new)
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PasswordResponse {
    WrongPassword,
    UserNotFound,
    Success,
}

#[server(ChangePasswordAction, "/pandas/profile/password")]
pub async fn change_password(
    old_password: String,
    new_password: String,
) -> Result<PasswordResponse, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::error::PasswordError;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    if let Err(err) =
        dbal::change_my_password(auth_state.user.id, &old_password, &new_password, &db).await
    {
        match err {
            PasswordError::WrongPassword => Ok(PasswordResponse::WrongPassword),
            PasswordError::UserNotFound => Ok(PasswordResponse::UserNotFound),
            PasswordError::Unknown => Err(ServerFnError::new("Error changing password")),
        }
    } else {
        Ok(PasswordResponse::Success)
    }
}
