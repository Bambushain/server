use actix_web::{delete, get, post, put, web};

use bamboo_common::backend::dbal;
use bamboo_common::backend::response::*;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::*;
use bamboo_common::core::error::*;

use crate::middleware::extract_character::{character, CharacterData};
use crate::path;
use bamboo_common::backend::actix::middleware::{authenticate, Authentication};

#[get(
    "/api/final-fantasy/character/{character_id}/gatherer",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn get_gatherers(
    authentication: Authentication,
    character: CharacterData,
    db: DbConnection,
) -> BambooApiResponseResult {
    dbal::get_gatherers(authentication.user.id, character.id, &db)
        .await
        .map(|data| list!(data))
}

#[get(
    "/api/final-fantasy/character/{character_id}/gatherer/{gatherer_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn get_gatherer(
    path: Option<path::GathererPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Gatherer> {
    let path = check_invalid_path!(path, "gatherer")?;

    dbal::get_gatherer(path.gatherer_id, authentication.user.id, character.id, &db)
        .await
        .map(|gatherer| ok!(gatherer))
}

#[post(
    "/api/final-fantasy/character/{character_id}/gatherer",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn create_gatherer(
    body: Option<web::Json<Gatherer>>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResult<Gatherer> {
    let body = check_missing_fields!(body, "gatherer")?;

    dbal::create_gatherer(authentication.user.id, character.id, body.into_inner(), &db)
        .await
        .map(|data| ok!(data))
}

#[put(
    "/api/final-fantasy/character/{character_id}/gatherer/{gatherer_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn update_gatherer(
    body: Option<web::Json<Gatherer>>,
    path: Option<path::GathererPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "gatherer")?;
    let body = check_missing_fields!(body, "gatherer")?;

    dbal::update_gatherer(
        path.gatherer_id,
        authentication.user.id,
        character.id,
        body.into_inner(),
        &db,
    )
        .await
        .map(|_| no_content!())
}

#[delete(
    "/api/final-fantasy/character/{character_id}/gatherer/{gatherer_id}",
    wrap = "authenticate!()",
    wrap = "character!()"
)]
pub async fn delete_gatherer(
    path: Option<path::GathererPath>,
    character: CharacterData,
    authentication: Authentication,
    db: DbConnection,
) -> BambooApiResponseResult {
    let path = check_invalid_path!(path, "gatherer")?;

    dbal::delete_gatherer(path.gatherer_id, authentication.user.id, character.id, &db)
        .await
        .map(|_| no_content!())
}
