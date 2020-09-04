mod dto;
mod endpoints;

use crate::endpoints::{add_article, get_articles};
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("index")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/articles", web::get().to(get_articles))
            .route("/article", web::post().to(add_article))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
