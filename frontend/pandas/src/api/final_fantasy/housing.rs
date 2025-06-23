use bamboo_common::core::entities::{CharacterHousing, HousingDistrict, HousingType};
use leptos::server;
use server_fn::ServerFnError;

#[server(GetHousingsAction, "/pandas/housing")]
pub async fn get_housings(character_id: i32) -> Result<Vec<CharacterHousing>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_character_housings(auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteHousingAction, "/pandas/housing")]
pub async fn delete_housing(character_id: i32, housing_id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_character_housing(housing_id, auth_state.user.id, character_id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(CreateHousingAction, "/pandas/housing")]
pub async fn create_housing(
    character_id: i32,
    housing_type: HousingType,
    district: HousingDistrict,
    ward: i16,
    plot: i16,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::create_character_housing(
        auth_state.user.id,
        character_id,
        CharacterHousing::new(character_id, district, housing_type, ward, plot),
        &db,
    )
    .await
    .map_err(ServerFnError::new)
    .map(|_| ())
}
