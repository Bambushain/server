use bamboo_common::core::entities::GroveEvent;
use chrono::NaiveDate;
use leptos::prelude::{server, ServerFnError};

#[server(CreateEventAction, "/pandas/calendar")]
pub async fn create_event(
    title: String,
    description: Option<String>,
    color: String,
    start_date: String,
    end_date: String,
    is_private: Option<String>,
    grove: Option<i32>,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::GroveEvent;
    use leptos_actix::extract;
    use leptos_cosmo::prelude::NaiveDate;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

    let start_date =
        NaiveDate::parse_from_str(start_date.as_str(), "%F").map_err(ServerFnError::new)?;
    let end_date =
        NaiveDate::parse_from_str(end_date.as_str(), "%F").map_err(ServerFnError::new)?;

    let grove = if let Some(grove) = grove {
        Some(
            dbal::get_grove(grove, user.id, &db)
                .await
                .map_err(ServerFnError::new)?,
        )
    } else {
        None
    };

    let event = GroveEvent {
        id: -1,
        title,
        description: description.unwrap_or("".to_string()),
        start_date,
        end_date,
        color,
        is_private: is_private
            .map(|val| val.to_lowercase() == "on")
            .unwrap_or(false),
        user: None,
        grove,
    };

    dbal::create_event(event, user.id, &db)
        .await
        .map_err(ServerFnError::new)
        .map(|_| ())
}

#[server(UpdateEventAction, "/pandas/calendar")]
pub async fn update_event(
    id: i32,
    title: String,
    description: Option<String>,
    color: String,
    start_date: String,
    end_date: String,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::GroveEvent;
    use leptos_actix::extract;
    use leptos_cosmo::prelude::NaiveDate;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

    let start_date =
        NaiveDate::parse_from_str(start_date.as_str(), "%F").map_err(ServerFnError::new)?;
    let end_date =
        NaiveDate::parse_from_str(end_date.as_str(), "%F").map_err(ServerFnError::new)?;

    let event = dbal::get_event(id, user.id, &db)
        .await
        .map_err(ServerFnError::new)?;

    let event = GroveEvent {
        id,
        title,
        description: description.unwrap_or("".to_string()),
        start_date,
        end_date,
        color,
        is_private: event.is_private,
        user: None,
        grove: event.grove,
    };

    dbal::update_event(user.id, id, event, &db)
        .await
        .map_err(ServerFnError::new)
        .map(|_| ())
}

#[server(DeleteEventAction, "/pandas/calendar")]
pub async fn delete_event(id: i32) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::delete_event(auth_state.user.id, id, &db)
        .await
        .map_err(ServerFnError::new)
}

#[server(GetEventsAction, "/pandas/calendar")]
pub async fn get_events(
    since: NaiveDate,
    until: NaiveDate,
    grove_id: Option<i32>,
) -> Result<Vec<GroveEvent>, ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use date_range::DateRange;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    dbal::get_events(
        DateRange::new(since, until).map_err(ServerFnError::new)?,
        auth_state.user.id,
        grove_id,
        &db,
    )
    .await
    .map_err(ServerFnError::new)
}
