use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

use crate::error_tag;
use bamboo_common_core::entities::*;
use bamboo_common_core::entities::{character, gatherer};
use bamboo_common_core::error::*;

pub async fn get_gatherers(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Gatherer>> {
    gatherer::Entity::find()
        .filter(gatherer::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(gatherer::Column::Job)
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load gatherers"))
}

pub async fn get_gatherer(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Gatherer> {
    gatherer::Entity::find_by_id(id)
        .filter(gatherer::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load gatherer"))?
        .ok_or(BambooError::not_found(
            error_tag!(),
            "The gatherer was not found",
        ))
}

async fn gatherer_exists_by_id(
    id: i32,
    user_id: i32,
    character_id: i32,
    job: GathererJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    gatherer::Entity::find()
        .filter(gatherer::Column::Id.ne(id))
        .filter(gatherer::Column::Job.eq(job))
        .filter(gatherer::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load the gatherers"))
}

async fn gatherer_exists_by_job(
    user_id: i32,
    character_id: i32,
    job: GathererJob,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    gatherer::Entity::find()
        .filter(gatherer::Column::Job.eq(job))
        .filter(gatherer::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load the gatherers"))
}

pub async fn create_gatherer(
    user_id: i32,
    character_id: i32,
    gatherer: Gatherer,
    db: &DatabaseConnection,
) -> BambooResult<Gatherer> {
    if gatherer_exists_by_job(user_id, character_id, gatherer.job, db).await? {
        return Err(BambooError::exists_already(
            error_tag!(),
            "A gatherer with that job exists already",
        ));
    }

    let mut model = gatherer.clone().into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;
    if gatherer.level.is_some_and(|level| level.is_empty()) {
        model.level = Set(None);
    }

    model.insert(db).await.map_err(|err| {
        log::error!("Failed to insert gatherer, {}", err);
        BambooError::database(error_tag!(), "Failed to create gatherer")
    })
}

pub async fn update_gatherer(
    id: i32,
    user_id: i32,
    character_id: i32,
    gatherer: Gatherer,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if gatherer_exists_by_id(id, user_id, character_id, gatherer.job, db).await? {
        return Err(BambooError::exists_already(
            "gatherer",
            "A gatherer with that job exists already",
        ));
    }

    let mut active_gatherer = get_gatherer(id, user_id, character_id, db)
        .await?
        .into_active_model();
    active_gatherer.level = Set(gatherer.level);

    active_gatherer
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("gatherer", "Failed to update gatherer")
        })
        .map(|_| ())
}

pub async fn delete_gatherer(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    get_gatherer(id, user_id, character_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("gatherer", "Failed to delete gatherer")
        })
        .map(|_| ())
}
