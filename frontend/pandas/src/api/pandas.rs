use bamboo_common::core::entities::user::GroveUser;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[server(GetPandasAction, "/pandas/pandas")]
pub async fn get_pandas(grove_id: Option<i32>) -> Result<Vec<GroveUser>, ServerFnError> {
    use crate::authentication::AuthState;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::dbal::BannedStatus;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;

    if let Some(grove_id) = grove_id {
        dbal::get_users_by_grove(auth_state.user.id, grove_id, BannedStatus::Unbanned, &db)
            .await
            .map_err(ServerFnError::new)
    } else {
        dbal::get_users(auth_state.user.id, &db)
            .await
            .map_err(ServerFnError::new)
            .map(|users| {
                users
                    .iter()
                    .map(|user| GroveUser {
                        id: user.id.clone(),
                        email: user.email.clone(),
                        display_name: user.display_name.clone(),
                        discord_name: user.discord_name.clone(),
                        is_mod: false,
                        is_banned: false,
                    })
                    .collect::<Vec<_>>()
            })
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum BanResultCode {
    Success,
    NotAllowed,
    NotFound,
}

#[server(BanPandaAction, "/pandas/pandas")]
pub async fn ban_panda(grove_id: i32, user_id: i32) -> Result<BanResultCode, ServerFnError> {
    use crate::authentication::AuthState;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    let (auth_state, db) = extract::<(AuthState, DbConnection)>().await?;
    if dbal::is_grove_mod(grove_id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)?
    {
        dbal::ban_user_from_grove(grove_id, user_id, &db)
            .await
            .map_err(ServerFnError::new)
            .map(|_| BanResultCode::Success)
    } else {
        Ok(BanResultCode::NotAllowed)
    }
}
