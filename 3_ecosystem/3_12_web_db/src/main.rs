mod dto;
mod endpoints;

use crate::endpoints::{add_article, delete_article, get_article, get_articles};

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/articles", web::get().to(get_articles))
            .service(
                web::scope("/article")
                    .route("", web::post().to(add_article))
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(get_article))
                            .route(web::delete().to(delete_article)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
