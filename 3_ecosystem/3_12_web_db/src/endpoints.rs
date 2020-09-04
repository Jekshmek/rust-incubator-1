use crate::dto;
use actix_web::{web, HttpResponse, Result};
use step_3_12::{establish_connection, get_all_articles};

pub async fn get_articles() -> Result<HttpResponse> {
    let connection = establish_connection();

    let (articles, connection) =
        web::block(move || -> Result<_, ()> { Ok((get_all_articles(&connection), connection)) })
            .await?;

    let articles = articles
        .into_iter()
        .map(|article| dto::Article::from_model(article, &connection))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(articles))
}

pub async fn add_article(article: web::Json<dto::Article>) -> Result<HttpResponse> {
    let connection = establish_connection();

    web::block(move || -> Result<(), ()> {
        article.store(&connection);
        Ok(())
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}
