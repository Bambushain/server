use bamboo_common::core::entities::FreeCompanyWithCharacterCountAndHousing;
use leptos::server;
use server_fn::ServerFnError;

#[server(GetFreeCompaniesAction, "/pandas/free-company")]
pub async fn get_free_companies(
) -> Result<Vec<FreeCompanyWithCharacterCountAndHousing>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_free_companies(auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}

#[server(CreateFreeCompanyAction, "/pandas/free-company")]
pub async fn create_free_company(
    name: String,
    district: String,
    ward: String,
    plot: String,
    has_housing: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::FreeCompanyHousing;
    use bamboo_common::core::error::BambooError;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let company = dbal::create_free_company(auth_state.user.id, &name, &db)
        .await
        .map_err::<ServerFnError<BambooError>, _>(ServerFnError::from)?;

    if has_housing == "on" {
        let ward = ward.parse::<i16>().unwrap_or(1);
        let plot = plot.parse::<i16>().unwrap_or(1);

        dbal::set_free_company_housing(
            auth_state.user.id,
            company.id,
            FreeCompanyHousing::new(company.id, district.into(), ward, plot),
            &db,
        )
        .await
        .map(|_| ())
        .map_err(ServerFnError::from)
    } else {
        Ok(())
    }
}

#[server(EditFreeCompanyAction, "/pandas/free-company")]
pub async fn edit_free_company(
    id: i32,
    name: String,
    district: String,
    ward: String,
    plot: String,
    has_housing: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::FreeCompanyHousing;
    use bamboo_common::core::error::BambooError;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::update_free_company(id, auth_state.user.id, &name, &db)
        .await
        .map_err::<ServerFnError<BambooError>, _>(ServerFnError::from)?;

    if has_housing == "on" {
        let ward = ward.parse::<i16>().unwrap_or(1);
        let plot = plot.parse::<i16>().unwrap_or(1);

        dbal::set_free_company_housing(
            auth_state.user.id,
            id,
            FreeCompanyHousing::new(id, district.into(), ward, plot),
            &db,
        )
        .await
        .map(|_| ())
        .map_err(ServerFnError::from)
    } else {
        dbal::delete_free_company_housing(auth_state.user.id, id, &db)
            .await
            .map(|_| ())
            .map_err(ServerFnError::from)
    }
}

#[server(DeleteFreeCompanyAction, "/pandas/free-company")]
pub async fn delete_free_company(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_free_company(id, auth_state.user.id, &db)
        .await
        .map_err(ServerFnError::from)
}
