use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, NotSet, QueryFilter,
    QuerySelect,
};

use bamboo_common_core::entities::user::BambooUser;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;

use crate as dbal;
use crate::{decrypt_string, encrypt_string, error_tag};

pub async fn create_token(username: &str, db: &DatabaseConnection) -> BambooResult<LoginResult> {
    let user = dbal::get_user_by_email_or_username(username, db)
        .await
        .map_err(|_| BambooError::not_found(error_tag!(), "User not found"))?;

    token::ActiveModel {
        id: NotSet,
        token: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
    }
    .insert(db)
    .await
    .map(|token| LoginResult {
        token: token.token,
        user,
    })
    .map_err(|_| BambooError::database(error_tag!(), "Failed to create token"))
}

pub async fn validate_auth(
    username: &str,
    password: &str,
    two_factor_code: Option<String>,
    db: &DatabaseConnection,
) -> BambooResult<TwoFactorResult> {
    let user = dbal::get_user_by_email_or_username(username, db)
        .await
        .map_err(|_| BambooError::not_found(error_tag!(), "User not found"))?;

    let password_valid = user.validate_password(password);
    if !password_valid {
        return Err(BambooError::validation(error_tag!(), "Password is invalid"));
    }

    let mut requires_two_factor_code =
        user.totp_secret.is_some() && user.totp_validated.unwrap_or(false);
    if requires_two_factor_code && let Some(two_factor_code) = two_factor_code {
        validate_two_factor_code(user.id, &two_factor_code, password, false, db).await?;
        requires_two_factor_code = false;
    }

    Ok(TwoFactorResult {
        user,
        requires_two_factor_code,
    })
}

pub async fn delete_token(token: String, db: &DatabaseConnection) -> BambooErrorResult {
    token::Entity::delete_many()
        .filter(token::Column::Token.eq(token))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete the token"))
        .map(|_| ())
}

pub async fn delete_all_token(user_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    token::Entity::delete_many()
        .filter(token::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete the tokens"))
        .map(|_| ())
}

pub async fn validate_two_factor_code(
    id: i32,
    code: &str,
    password: &str,
    initial_validation: bool,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let user = dbal::get_user(id, db).await?;

    let password_valid = user.validate_password(password);
    if !password_valid {
        return Err(BambooError::unauthorized(
            error_tag!(),
            "Invalid login data",
        ));
    }

    if initial_validation || user.totp_validated.unwrap_or(false) {
        validate_totp_token(code, password, user, db).await
    } else {
        Ok(())
    }
}

async fn validate_totp_token(
    code: &str,
    password: &str,
    user: BambooUser,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let totp_secret = if user.totp_secret_encrypted {
        decrypt_string(user.totp_secret.unwrap(), password)?
    } else {
        let decrypted_secret = user.totp_secret.unwrap();
        let encrypted_secret = encrypt_string(&decrypted_secret, password)?;

        user::Entity::update_many()
            .col_expr(user::Column::TotpSecretEncrypted, Expr::value(true))
            .col_expr(user::Column::TotpSecret, Expr::value(encrypted_secret))
            .filter(user::Column::Id.eq(user.id))
            .exec(db)
            .await
            .map_err(|_| BambooError::database(error_tag!(), "Failed to validate"))?;

        decrypted_secret
    };

    totp_rs::TOTP::from_rfc6238(
        totp_rs::Rfc6238::new(
            6,
            totp_secret.clone(),
            Some("Bambushain".to_string()),
            user.display_name.clone(),
        )
        .map_err(|_| BambooError::crypto(error_tag!(), "Failed to validate"))?,
    )
    .map_err(|_| BambooError::crypto(error_tag!(), "Failed to validate"))
    .map(|totp| {
        totp.check_current(code)
            .map_err(|_| BambooError::crypto(error_tag!(), "Failed to validate"))
            .map(|_| ())
    })?
}

pub async fn create_firebase_token(
    user_id: i32,
    token: &str,
    db: &DatabaseConnection,
) -> BambooResult<()> {
    firebase_token::ActiveModel {
        id: NotSet,
        user_id: Set(user_id),
        token: Set(token.to_string()),
    }
    .insert(db)
    .await
    .map_err(|_| BambooError::database(error_tag!(), "Failed to create firebase token"))
    .map(|_| ())
}

pub async fn delete_firebase_token(token: &str, db: &DatabaseConnection) -> BambooErrorResult {
    firebase_token::Entity::delete_many()
        .filter(firebase_token::Column::Token.eq(token))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete the firebase token"))
        .map(|_| ())
}

pub async fn delete_firebase_token_by_user(
    user_id: i32,
    token: &str,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    firebase_token::Entity::delete_many()
        .filter(firebase_token::Column::UserId.eq(user_id))
        .filter(firebase_token::Column::Token.eq(token))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete the firebase token"))
        .map(|_| ())
}

pub async fn delete_all_firebase_token(user_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    firebase_token::Entity::delete_many()
        .filter(firebase_token::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete the firebase tokens"))
        .map(|_| ())
}

pub async fn get_firebase_tokens_by_user(
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<FirebaseToken>> {
    firebase_token::Entity::find()
        .filter(firebase_token::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to fetch firebase tokens"))
}

pub async fn get_firebase_tokens_by_users(
    user_ids: Vec<i32>,
    db: &DatabaseConnection,
) -> BambooResult<Vec<FirebaseToken>> {
    firebase_token::Entity::find()
        .filter(firebase_token::Column::UserId.is_in(user_ids))
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to fetch firebase tokens"))
}

pub async fn get_firebase_tokens_for_grove_mods(
    grove_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<FirebaseToken>> {
    firebase_token::Entity::find()
        .join(
            JoinType::InnerJoin,
            firebase_token::Entity::belongs_to(grove_user::Entity)
                .from(firebase_token::Column::UserId)
                .to(grove_user::Column::UserId)
                .into(),
        )
        .filter(grove_user::Column::IsMod.eq(true))
        .filter(grove_user::Column::IsBanned.eq(false))
        .filter(grove_user::Column::GroveId.eq(grove_id))
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to fetch firebase tokens"))
}
