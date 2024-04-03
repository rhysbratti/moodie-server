#[cfg(feature = "ssr")]
use actix_session::{storage::CookieSessionStore, *};
#[cfg(feature = "ssr")]
use actix_web::{cookie::Key, *};
#[cfg(feature = "ssr")]
use moodie_server::redis_helper::start_recommendation_session;
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use moodie_server::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::generate(),
            ))
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            //.service(session)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use moodie_server::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

/*
#[cfg(feature = "ssr")]
#[actix_web::get("/")]
async fn session() -> impl Responder {
    println!("Got request to start session");
    match start_recommendation_session().await {
        Err(err) => HttpResponse::InternalServerError().body("Uh oh"),
        Ok(session_id) => {
            let cookie = cookie::Cookie::build("session_id", session_id)
                .path("/")
                .secure(false) // Set to true in production with HTTPS
                .http_only(true)
                .finish();
            leptos_actix::redirect("/providers");
            HttpResponse::Ok().cookie(cookie).body("")
        }
    }
}
*/
