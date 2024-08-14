#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use bamboo_frontend_pandas::app::App;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{addr}");

    HttpServer::new(move || {
        let mut leptos_options = conf.leptos_options.clone();
        let site_root = &leptos_options.site_root;

        leptos_options.site_pkg_dir = "pandas/pkg".to_string();

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pandas/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/pandas/assets", site_root))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
