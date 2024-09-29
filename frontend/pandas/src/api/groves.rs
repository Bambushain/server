use bamboo_common::core::entities::user::GroveUser;
use bamboo_common::core::entities::Grove;
use leptos::{server, ServerFnError};

#[server(GetGrovesAction, "/pandas/groves")]
pub async fn get_all_groves() -> Result<Vec<Grove>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::get_groves(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(GetGroveAction, "/pandas/grove")]
pub async fn get_grove(id: i32) -> Result<Grove, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::get_grove(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(EnableInvitesAction, "/pandas/grove/invite/enable")]
pub async fn enable_invites(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::enable_grove_invite(id, &db)
            .await
            .map_err(ServerFnError::new)
    } else {
        Err(ServerFnError::new("You need to be mod"))
    }
}

#[server(DisableInvitesAction, "/pandas/grove/invite/disable")]
pub async fn disable_invites(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::disable_grove_invite(id, &db)
            .await
            .map_err(ServerFnError::new)
    } else {
        Err(ServerFnError::new("You need to be mod"))
    }
}

#[server(IsGroveMod, "/pandas/grove/is_mod")]
pub async fn is_grove_mod(id: i32) -> Result<bool, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    dbal::is_grove_mod(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteGroveAction, "/pandas/grove/delete")]
pub async fn delete_grove(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::delete_grove(id, &db)
            .await
            .map_err(ServerFnError::new)
    } else {
        Err(ServerFnError::new("You need to be mod"))
    }
}

#[server(UpdateModsAction, "/pandas/grove/mods")]
pub async fn update_mods(id: i32, mods: Vec<i32>) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::update_grove_mods(id, auth_state.user.id, mods, &db)
            .await
            .map_err(ServerFnError::new)
            .map(|_| ())
    } else {
        Err(ServerFnError::new("You need to be mod"))
    }
}

#[server(GetBannedPandasAction, "/pandas/grove/banned")]
pub async fn get_banned_pandas(grove_id: i32) -> Result<Vec<GroveUser>, ServerFnError> {
    use crate::authentication::AuthState;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::dbal::BannedStatus;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    dbal::get_users_by_grove(auth_state.user.id, grove_id, BannedStatus::Banned, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(UnbanPandaAction, "/pandas/grove/unban")]
pub async fn unban_panda(grove_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use crate::authentication::AuthState;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(grove_id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::unban_user_from_grove(grove_id, user_id, &db)
            .await
            .map_err(ServerFnError::new)
            .map(|_| ())
    } else {
        Err(ServerFnError::new("Not allowed"))
    }
}
