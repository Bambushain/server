use crate::front::homepage;
use crate::legal::{imprint, licenses, privacy};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

mod front;
mod legal;
mod utils;

pub async fn start_server() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    bamboo_common::backend::logging::init();
    log::info!("listening on http://0.0.0.0:4070");
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "public/assets"))
            .service(imprint)
            .service(privacy)
            .service(licenses)
            .default_service(web::to(homepage))
            .wrap(middleware::Compress::default())
    })
    .bind("0.0.0.0:4070")?
    .run()
    .await
}
