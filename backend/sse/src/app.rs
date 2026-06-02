use crate::listener::start_listening;
use crate::notifier::{
    EventNotifier, EventNotifierState, NotificationsNotifier, NotificationsNotifierState,
};
use crate::routes;
use actix_web::{middleware, App, HttpServer};
use bamboo_common::backend::services::DbConnection;

pub async fn start_server() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Listening for sse connections");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;

    let event_notifier_state = EventNotifierState::new();
    let notifications_notifier_state = NotificationsNotifierState::new();

    start_listening(
        event_notifier_state.clone(),
        notifications_notifier_state.clone(),
    )
    .await
    .map_err(std::io::Error::other)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(DbConnection::new(db.clone()))
            .app_data(EventNotifier::new(event_notifier_state.clone()))
            .app_data(NotificationsNotifier::new(
                notifications_notifier_state.clone(),
            ))
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 4020))?
    .run()
    .await?;

    Ok(())
}
