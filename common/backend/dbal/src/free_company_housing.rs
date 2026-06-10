use crate::error_tag;
use bamboo_common_core::entities::*;
use bamboo_common_core::entities::{free_company, free_company_housing};
use bamboo_common_core::error::*;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet};

pub async fn get_free_company_housing(
    user_id: i32,
    free_company_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<FreeCompanyHousing> {
    free_company_housing::Entity::find()
        .filter(free_company_housing::Column::FreeCompanyId.eq(free_company_id))
        .filter(free_company::Column::UserId.eq(user_id))
        .inner_join(free_company::Entity)
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load free company housing"))?
        .ok_or(BambooError::not_found(
            error_tag!(),
            "The free_company housing was not found",
        ))
}

pub async fn set_free_company_housing(
    user_id: i32,
    free_company_id: i32,
    housing: FreeCompanyHousing,
    db: &DatabaseConnection,
) -> BambooResult<FreeCompanyHousing> {
    let old_housing = get_free_company_housing(user_id, free_company_id, db)
        .await
        .unwrap_or(housing.clone());

    let mut model = old_housing.into_active_model();
    model.district = Set(housing.district);
    model.ward = Set(housing.ward);
    model.plot = Set(housing.plot);
    model.id = NotSet;

    model
        .insert(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create free company housing"))
}

pub async fn delete_free_company_housing(
    user_id: i32,
    free_company_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    get_free_company_housing(user_id, free_company_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete free_company housing"))
        .map(|_| ())
}
