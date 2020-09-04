mod dto;
mod endpoints;

use crate::endpoints::get_articles;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("index")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/articles", web::get().to(get_articles)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
