use crate::api::BambooCodeError;
use bamboo_common::core::entities::CustomCharacterField;
use leptos::server;
use server_fn::ServerFnError;
use std::collections::BTreeSet;

#[server(GetCustomFieldsAction, "/pandas/custom-field")]
pub async fn get_custom_fields() -> Result<Vec<CustomCharacterField>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_custom_fields(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(SaveCustomFieldPositionAction, "/pandas/custom-field")]
pub async fn save_custom_field_position(id: i32, new_position: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::move_custom_field(auth_state.user.id, id, new_position, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(DeleteCustomFieldAction, "/pandas/custom-field")]
pub async fn delete_custom_field(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_custom_field(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(CreateCustomFieldAction, "/pandas/custom-field")]
pub async fn create_custom_field(
    position: usize,
    label: String,
    values: BTreeSet<String>,
) -> Result<(), BambooCodeError> {
    use crate::api::bamboo_error_to_serverfn_error;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::CustomField;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>()
        .await
        .map_err(|_| BambooCodeError::Unknown)?;

    dbal::create_custom_field(
        auth_state.user.id,
        CustomField {
            values,
            label,
            position,
        },
        &db,
    )
        .await
        .map_err(bamboo_error_to_serverfn_error)
        .map(|_| ())
}

#[server(UpdateCustomFieldAction, "/pandas/custom-field")]
pub async fn update_custom_field(
    id: i32,
    position: usize,
    label: String,
    values: Option<BTreeSet<(i32, String)>>,
    deleted_values: Option<BTreeSet<i32>>,
) -> Result<(), BambooCodeError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::CustomField;
    use leptos_actix::extract;

    use crate::api::bamboo_error_to_serverfn_error;
    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>()
        .await
        .map_err(|_| BambooCodeError::Unknown)?;

    dbal::update_custom_field_with_options(
        id,
        auth_state.user.id,
        CustomField {
            values: BTreeSet::new(),
            label,
            position,
        },
        values.unwrap_or_default(),
        deleted_values.unwrap_or_default(),
        &db,
    )
        .await
        .map_err(|err| {
            log::error!("Failed to update custom field: {}", err);
            bamboo_error_to_serverfn_error(err)
        })
}
