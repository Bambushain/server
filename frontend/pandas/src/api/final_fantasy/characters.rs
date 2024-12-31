use crate::api::BambooCodeError;
use bamboo_common::core::entities::Character;
use leptos::server;
use server_fn::ServerFnError;
use std::collections::{BTreeMap, BTreeSet};

#[server(GetCharacterAction, "/pandas/characters")]
pub async fn get_characters() -> Result<Vec<Character>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

    dbal::get_characters(user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteCharacterAction, "/pandas/characters")]
pub async fn delete_character(character_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_character(character_id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(CreateCharacterAction, "/pandas/characters")]
pub async fn create_character(
    race: String,
    name: String,
    world: String,
    datacenter: String,
    free_company: Option<String>,
    custom_fields: Option<BTreeMap<String, BTreeSet<String>>>,
) -> Result<Character, ServerFnError<BambooCodeError>> {
    use crate::api::bamboo_error_to_serverfn_error;
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::{Character, CharacterRace, CustomField};
    use bamboo_common::core::error::BambooErrorCode;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await.map_err(|_| {
        ServerFnError::WrappedServerError(BambooCodeError {
            code: BambooErrorCode::Unknown,
        })
    })?;

    let free_company = if let Some(free_company) = free_company {
        dbal::get_free_company_by_name(free_company, auth_state.user.id, &db)
            .await
            .map_err(bamboo_error_to_serverfn_error)?
    } else {
        None
    };
    let custom_fields = custom_fields
        .unwrap_or_default()
        .into_iter()
        .map(|(label, values)| CustomField {
            label,
            values,
            ..CustomField::default()
        })
        .collect::<Vec<_>>();

    dbal::create_character(
        auth_state.user.id,
        Character::new(
            CharacterRace::from(race),
            name,
            world,
            datacenter,
            custom_fields,
            free_company,
        ),
        &db,
    )
    .await
    .map_err(bamboo_error_to_serverfn_error)
}
