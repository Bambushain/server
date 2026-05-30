#![allow(clippy::too_many_arguments)]
use bamboo_common::core::entities::GroveEvent;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use leptos::prelude::{server, ServerFnError};

#[server(CreateEventAction, "/pandas/calendar")]
pub async fn create_event(
    title: String,
    description: Option<String>,
    color: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    is_private: bool,
    grove: Option<i32>,
    notifications: Vec<DateTime<Utc>>,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::GroveEvent;
    use bamboo_common::core::entities::event::GroveEventReminder;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

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
        start_time,
        end_time,
        color,
        is_private,
        user: None,
        grove,
        reminder: notifications
            .into_iter()
            .map(|notification| GroveEventReminder {
                id: -1,
                when: notification,
            })
            .collect(),
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
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    notifications: Vec<DateTime<Utc>>,
) -> Result<(), ServerFnError> {
    use bamboo_common::backend::dbal;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_common::core::entities::GroveEvent;
    use bamboo_common::core::entities::event::GroveEventReminder;
    use leptos_actix::extract;

    use crate::authentication::AuthState;

    let (db, auth_state) = extract::<(DbConnection, AuthState)>().await?;

    let user = auth_state.user.clone();

    let event = dbal::get_event(id, user.id, &db)
        .await
        .map_err(ServerFnError::new)?;

    let event = GroveEvent {
        id,
        title,
        description: description.unwrap_or("".to_string()),
        start_date,
        end_date,
        start_time,
        end_time,
        color,
        is_private: event.is_private,
        user: None,
        grove: event.grove,
        reminder: notifications
            .into_iter()
            .map(|notification| GroveEventReminder {
                id: -1,
                when: notification,
            })
            .collect(),
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
