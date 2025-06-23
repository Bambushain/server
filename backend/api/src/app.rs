use actix_web::{middleware, App, HttpServer};
use bamboo_common::backend::services::DbConnection;
use bamboo_common::backend::services::MinioClient;

use crate::routes;

pub async fn start_server() -> std::io::Result<()> {
    bamboo_common::backend::logging::init();

    log::info!("Open the bamboo grove");
    let db = bamboo_common::backend::database::get_database()
        .await
        .map_err(std::io::Error::other)?;

    let minio_client = MinioClient::new().map_err(std::io::Error::other)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(bamboo_common::backend::services::MinioService::new(
                minio_client.clone(),
            ))
            .app_data(DbConnection::new(db.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 4010))?
    .run()
    .await
}
