use crate::front::homepage;
use crate::legal::{legal_notice, licenses, privacy};
use crate::register::{create_account, create_account_page, register_page, send_registration};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use bamboo_common::backend::database::get_database;
use bamboo_common::backend::services::DbConnection;

mod front;
mod legal;
mod register;
mod utils;

pub async fn start_server() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    bamboo_common::backend::logging::init();

    let db = DbConnection::new(get_database().await.map_err(std::io::Error::other)?);

    log::info!("listening on http://0.0.0.0:4070");
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(Files::new(
                "/static",
                std::env::var("STATIC_DIR").unwrap_or("public/assets".to_string()),
            ))
            .service(legal_notice)
            .service(privacy)
            .service(licenses)
            .service(register_page)
            .service(send_registration)
            .service(create_account_page)
            .service(create_account)
            .default_service(web::to(homepage))
            .wrap(middleware::Compress::default())
    })
    .bind("0.0.0.0:4070")?
    .run()
    .await
}
