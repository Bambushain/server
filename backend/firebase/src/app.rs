use crate::listener;
use std::io::ErrorKind;

pub async fn start() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    let fcm_service_account =
        std::env::var("FCM_SERVICE_ACCOUNT_JSON").expect("Firebase fcm api key needs to be set");
    if !tokio::fs::try_exists(&fcm_service_account).await? {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Firebase service account json file not found",
        ));
    }

    log::info!("Listening for new notifications to send");
    listener::start_listening(fcm_service_account)
        .await
        .map_err(std::io::Error::other)
        .map(|_| ())
}
