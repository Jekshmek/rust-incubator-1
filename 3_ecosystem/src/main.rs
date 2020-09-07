mod auth;
mod config;
mod db;
mod graphql;
mod model;

use std::io;

use actix_files::NamedFile;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, Result};
use sqlx::PgPool;

use crate::auth::handlers::{get_logged_user, login_user, register_user};
use crate::config::CONFIG;
use crate::db::UserRepo;
use graphql::model::schema;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("3_ecosystem/static/index.html")?)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let pool = PgPool::builder()
        .max_size(CONFIG.database.max_connections)
        .build(CONFIG.database.url.as_str())
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let repo = UserRepo::new(pool);

    HttpServer::new(move || {
        App::new()
            .data(repo.clone())
            .data(schema())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(CONFIG.server.secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(CONFIG.server.domain.as_str())
                    .secure(false),
            ))
            .route("/", web::get().to(index))
            .route("/info", web::get().to(get_logged_user))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/graphiql", web::get().to(graphql::handler::graphiql))
            .service(
                web::resource("/api")
                    .route(web::post().to(graphql::handler::graphql))
                    .route(web::get().to(graphql::handler::graphql)),
            )
    })
    .bind(CONFIG.server.url.as_str())?
    .run()
    .await
}
