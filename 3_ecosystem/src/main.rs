mod auth;
mod config;
mod db;
mod graphql;
mod model;

use std::{env, io};

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;

use crate::config::CONFIG;
use crate::db::UserRepo;
use crate::graphql::model::schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let pool = PgPool::builder()
        .max_size(CONFIG.database.max_connections)
        .build(CONFIG.database.url.as_str())
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let repo = UserRepo::new(pool);

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(repo.clone())
            .data(schema())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(CONFIG.server.secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(CONFIG.server.domain.as_str())
                    .secure(false),
            ))
            .route("/graphiql", web::get().to(graphql::handlers::graphiql))
            .route("/playground", web::get().to(graphql::handlers::playground))
            .service(
                web::resource("/api")
                    .route(web::post().to(graphql::handlers::graphql))
                    .route(web::get().to(graphql::handlers::graphql)),
            )
    })
    .bind(CONFIG.server.url.as_str())?
    .run()
    .await
}
