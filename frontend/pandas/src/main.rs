mod authentication;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::authentication::authenticate_user;
    use actix_files::Files;
    use actix_web::*;
    use bamboo_common::backend::database::get_database;
    use bamboo_common::backend::services::DbConnection;
    use bamboo_frontend_pandas::app::App;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;

    let _ = dotenvy::dotenv();

    bamboo_common::backend::logging::init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    log::info!("listening on http://{addr}");

    let db = DbConnection::new(get_database().await.map_err(std::io::Error::other)?);

    HttpServer::new(move || {
        let mut leptos_options = conf.leptos_options.clone();
        let site_root = &leptos_options.site_root;
        leptos_options.site_pkg_dir = "pandas/pkg".into();

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pandas/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/pandas/assets", site_root.as_ref()))
            .leptos_routes(routes.to_owned(), {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="de">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(db.clone())
            .wrap(middleware::from_fn(authenticate_user))
            .wrap(middleware::Compress::default())
    })
        .bind(&addr)?
        .run()
        .await
}
