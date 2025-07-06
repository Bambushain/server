use crate::error_tag;
use bamboo_common_core::entities::user::{BambooUser, WebUser};
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;
use chrono::{Days, Local, NaiveDate};
use sea_orm::prelude::*;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::{Alias, ColumnRef, Expr, IntoCondition, IntoIden};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult,
    IntoActiveModel, JoinType, NotSet, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};

pub async fn get_user(id: i32, db: &DatabaseConnection) -> BambooResult<BambooUser> {
    user::Entity::find_by_id(id)
        .column_as(
            Expr::val("/api/user/")
                .concatenate(Expr::col(user::Column::Id))
                .concatenate("/picture?time=")
                .concatenate(Expr::current_timestamp()),
            "profile_picture",
        )
        .into_model::<BambooUser>()
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to execute database query"))?
        .ok_or(BambooError::not_found(
            error_tag!(),
            "The user was not found",
        ))
}

pub async fn get_user_by_token(token: &str, db: &DatabaseConnection) -> BambooResult<BambooUser> {
    user::Entity::find()
        .column_as(
            Expr::val("/api/user/")
                .concatenate(Expr::col(ColumnRef::TableColumn(
                    Alias::new("user").into_iden(),
                    user::Column::Id.into_iden(),
                )))
                .concatenate("/picture?time=")
                .concatenate(Expr::current_timestamp()),
            "profile_picture",
        )
        .filter(token::Column::Token.eq(token))
        .join(JoinType::InnerJoin, user::Relation::Token.def())
        .into_model::<BambooUser>()
        .one(db)
        .await
        .map_err(|_| BambooError::unauthorized(error_tag!(), "Token or user not found"))?
        .ok_or(BambooError::unauthorized(
            error_tag!(),
            "Token or user not found",
        ))
}

pub async fn get_user_by_email_or_username(
    username: &str,
    db: &DatabaseConnection,
) -> BambooResult<BambooUser> {
    user::Entity::find()
        .column_as(
            Expr::val("/api/user/")
                .concatenate(Expr::col(user::Column::Id))
                .concatenate("/picture?time=")
                .concatenate(Expr::current_timestamp()),
            "profile_picture",
        )
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(username))
                .add(user::Column::DisplayName.eq(username)),
        )
        .into_model::<BambooUser>()
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to execute database query"))?
        .ok_or(BambooError::not_found(
            error_tag!(),
            "The user was not found",
        ))
}

pub enum BannedStatus {
    Banned,
    Unbanned,
    All,
}

async fn get_users_from_db<U>(
    user_id: i32,
    additional_filter: Option<Condition>,
    banned_status: BannedStatus,
    db: &DatabaseConnection,
) -> BambooResult<Vec<U>>
where
    U: FromQueryResult,
{
    let sub_query = &mut grove_user::Entity::find()
        .select_only()
        .column(grove_user::Column::GroveId)
        .filter(grove_user::Column::UserId.eq(user_id));

    let mut filter = Condition::all()
        .add(grove_user::Column::GroveId.in_subquery(QuerySelect::query(sub_query).to_owned()));

    if let Some(additional_filter) = additional_filter {
        filter = filter.add(additional_filter);
    }
    filter = match banned_status {
        BannedStatus::Banned => filter.add(grove_user::Column::IsBanned.eq(true)),
        BannedStatus::Unbanned => filter.add(grove_user::Column::IsBanned.eq(false)),
        BannedStatus::All => filter,
    };

    user::Entity::find()
        .select_only()
        .distinct_on(vec![Alias::new("display_name")])
        .column_as(user::Column::Id, "id")
        .column_as(user::Column::Email, "email")
        .column_as(user::Column::DiscordName, "discord_name")
        .column_as(user::Column::DisplayName, "display_name")
        .column_as(grove_user::Column::IsMod, "is_mod")
        .column_as(grove_user::Column::IsBanned, "is_banned")
        .column_as(
            Expr::val("/api/user/")
                .concatenate(Expr::col(user::Column::Id))
                .concatenate("/picture?time=")
                .concatenate(Expr::current_timestamp()),
            "profile_picture",
        )
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(user::Entity)
                .from(grove_user::Column::UserId)
                .to(user::Column::Id)
                .into(),
        )
        .filter(filter)
        .order_by_asc(user::Column::DisplayName)
        .into_model::<U>()
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load users"))
}

pub async fn get_users(user_id: i32, db: &DatabaseConnection) -> BambooResult<Vec<WebUser>> {
    get_users_from_db(user_id, None, BannedStatus::Unbanned, db).await
}

pub async fn get_users_by_grove(
    user_id: i32,
    grove_id: i32,
    banned_status: BannedStatus,
    db: &DatabaseConnection,
) -> BambooResult<Vec<user::GroveUser>> {
    get_users_from_db(
        user_id,
        Some(grove_user::Column::GroveId.eq(grove_id).into_condition()),
        banned_status,
        db,
    )
    .await
}

pub async fn user_is_banned_from_grove(
    user_id: i32,
    grove_id: i32,
    db: &DatabaseConnection,
) -> bool {
    grove_user::Entity::find()
        .filter(grove_user::Column::UserId.eq(user_id))
        .filter(grove_user::Column::GroveId.eq(grove_id))
        .filter(grove_user::Column::IsBanned.eq(true))
        .all(db)
        .await
        .map(|x| !x.is_empty())
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load users"))
        .unwrap_or(true)
}

pub(crate) async fn user_exists_by_id(
    id: i32,
    email: &str,
    name: &str,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    user::Entity::find()
        .filter(user::Column::Id.ne(id))
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(email))
                .add(user::Column::DisplayName.eq(name)),
        )
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load users"))
}

async fn user_exists_by_email_and_name(
    email: &str,
    name: &str,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(email))
                .add(user::Column::DisplayName.eq(name)),
        )
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load users"))
}

pub async fn create_user(
    user: User,
    password: &str,
    db: &DatabaseConnection,
) -> BambooResult<User> {
    if user_exists_by_email_and_name(&user.email, &user.display_name, db).await? {
        return Err(BambooError::exists_already(
            error_tag!(),
            "A user with that email or name exists already",
        ));
    }

    let mut model = user.into_active_model();
    model.id = NotSet;
    model
        .set_password(password)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to hash password user"))?;

    model
        .insert(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create user"))
}

pub async fn delete_user(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete user"))
        .map(|_| ())
}

pub async fn set_password(id: i32, password: &str, db: &DatabaseConnection) -> BambooErrorResult {
    let hashed_password = bcrypt::hash(password, 12)
        .map_err(|_| BambooError::unknown(error_tag!(), "Failed to hash the password"))?;

    token::Entity::delete_many()
        .filter(token::Column::UserId.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update user"))
        .map(|_| ())?;

    user::Entity::update_many()
        .col_expr(user::Column::Password, Expr::value(hashed_password))
        .col_expr(
            user::Column::TotpSecret,
            Expr::value::<Option<Vec<u8>>>(None),
        )
        .col_expr(user::Column::TotpSecretEncrypted, Expr::value(false))
        .col_expr(user::Column::TotpValidated, Expr::value(false))
        .filter(user::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update user"))
        .map(|_| ())
}

pub async fn set_forgot_password_token(
    id: i32,
    db: &DatabaseConnection,
) -> BambooResult<(String, NaiveDate)> {
    let user = get_user(id, db).await?;
    let valid_until =
        Local::now()
            .checked_add_days(Days::new(7))
            .ok_or(BambooError::invalid_data(
                error_tag!(),
                "Failed to add a week to the date",
            ))?;
    let mut token = [0u8; 32];
    getrandom::getrandom(&mut token)
        .map_err(|_| BambooError::crypto(error_tag!(), "Failed to generate secure random code"))?;

    let token = hex::encode(token);
    let hashed_token = bcrypt::hash(token.clone(), 12)
        .map_err(|_| BambooError::crypto(error_tag!(), "Failed to generate secure random code"))?;

    let mut active_user = user.into_user_active_model();
    active_user.forgot_password_valid_until = Set(Some(valid_until.date_naive()));
    active_user.forgot_password_code = Set(Some(hashed_token));
    active_user
        .update(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update user"))?;

    Ok((token, valid_until.date_naive()))
}

pub async fn reset_password_by_token(
    email: &str,
    token: &str,
    password: &str,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let user = get_user_by_email_or_username(email, db).await?;
    if let (Some(code), Some(until)) = (
        user.forgot_password_code.clone(),
        user.forgot_password_valid_until,
    ) {
        if until >= Local::now().date_naive()
            && bcrypt::verify(token, code.as_str()).unwrap_or(false)
        {
            let mut active_user = user.clone().into_user_active_model();
            active_user
                .set_password(password)
                .map_err(|_| BambooError::crypto(error_tag!(), "Failed to hash password"))?;
            active_user.forgot_password_code = Set(None);
            active_user.forgot_password_valid_until = Set(None);
            active_user.totp_secret = Set(None);
            active_user.totp_validated = Set(Some(false));
            active_user.totp_secret_encrypted = Set(false);

            active_user
                .update(db)
                .await
                .map_err(|_| BambooError::database(error_tag!(), "Failed to save the user"))
                .map(|_| ())?;

            token::Entity::delete_many()
                .filter(token::Column::UserId.eq(user.id))
                .exec(db)
                .await
                .map_err(|_| BambooError::database(error_tag!(), "Failed to delete auth tokens"))
                .map(|_| ())
        } else {
            Err(BambooError::insufficient_rights(
                error_tag!(),
                "The token is either invalid or expired",
            ))
        }
    } else {
        Err(BambooError::insufficient_rights(
            error_tag!(),
            "No data set for forgot password",
        ))
    }
}

pub async fn user_exists(email: &str, db: &DatabaseConnection) -> bool {
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .select_only()
        .column(user::Column::Id)
        .count(db)
        .await
        .is_ok_and(|c| c > 0)
}
