use crate::worker;

pub async fn start() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Start check for event reminder");
    worker::start_work()
        .await
        .map_err(std::io::Error::other)
        .map(|_| ())
}
