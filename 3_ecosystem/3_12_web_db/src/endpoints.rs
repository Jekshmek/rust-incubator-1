use actix_web::{error, web, HttpRequest, HttpResponse, Result};
use step_3_12 as db;

use crate::dto;

pub async fn get_articles() -> Result<HttpResponse> {
    let connection = db::get_connection();

    let (articles, connection) = web::block(move || -> Result<_, ()> {
        Ok((db::get_all_articles(&connection), connection))
    })
    .await?;

    let articles = articles
        .into_iter()
        .map(|article| dto::Article::from_model(article, &connection))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(articles))
}

pub async fn add_article(article: web::Json<dto::Article>) -> Result<HttpResponse> {
    let connection = db::get_connection();

    web::block(move || -> Result<(), ()> {
        article.store(&connection);
        Ok(())
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_article(req: HttpRequest) -> Result<HttpResponse> {
    let id = get_id(&req)?;

    let connection = db::get_connection();

    let (article, connection) = web::block(move || -> Result<_, ()> {
        Ok((db::get_article(id, &connection).map(|article| (article, connection))).ok_or(()))
    })
    .await?
    .map_err(|_| error::ErrorNotFound(format!("No article with id {}", id)))?;

    let article =
        web::block(move || -> Result<_, ()> { Ok(dto::Article::from_model(article, &connection)) })
            .await?;

    Ok(HttpResponse::Ok().json(article))
}

pub async fn delete_article(req: HttpRequest) -> Result<HttpResponse> {
    let id = get_id(&req)?;

    let connection = db::get_connection();

    let deleted =
        web::block(move || -> Result<_, ()> { Ok(db::delete_article(id, &connection)) }).await?;

    if deleted {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(error::ErrorNotFound(format!("No article with id {}", id)))
    }
}

fn get_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    Ok(req
        .match_info()
        .get("id")
        .ok_or_else(|| error::ErrorBadRequest("No article id"))?
        .parse::<i32>()
        .map_err(|_| error::ErrorBadRequest("Invalid id"))?)
}
