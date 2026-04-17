mod auth;
mod config;
mod error;
mod models;
mod schema;
mod store;
mod user;

use actix_web::{App, HttpServer, middleware::Logger, web};
use store::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cfg = config::Config::from_env()?;
    if cfg.jwt_secret == "debug-key" {
        log::warn!("===============================================================");
        log::warn!("JWT_SECRET is not configured! DO NOT use this in a deployment!");
        log::warn!("===============================================================");
    }
    let host = cfg.host.clone();
    let port = cfg.port;

    let state = web::Data::new(AppState::new(cfg)?);

    log::info!("Starting API on {host}:{port}");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .app_data(state.clone())
                .wrap(Logger::default())
                .route("/login", web::post().to(auth::login_post))
                .route("/login", web::get().to(auth::login_get))
                .route("/login", web::put().to(auth::login_put))
                .route("/refresh", web::post().to(auth::refresh_post))
                .route("/logout", web::post().to(auth::logout_post))
                .route("/user/{id}", web::get().to(user::user_get))
                .route("/user", web::post().to(user::user_post))
                .route("/user", web::delete().to(user::user_delete)),
        )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
