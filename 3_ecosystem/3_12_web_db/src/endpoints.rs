use crate::dto;

use crate::dto::Article;
use actix_web::{error, web, HttpRequest, HttpResponse, Result};
use step_3_12 as db;

pub async fn get_articles() -> Result<HttpResponse> {
    let connection = db::establish_connection();

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
    let connection = db::establish_connection();

    web::block(move || -> Result<(), ()> {
        article.store(&connection);
        Ok(())
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_article(req: HttpRequest) -> Result<HttpResponse> {
    let id = req
        .match_info()
        .get("id")
        .ok_or_else(|| error::ErrorBadRequest("No article id"))?
        .parse::<i32>()
        .map_err(|_| error::ErrorBadRequest("Invalid id"))?;

    let connection = db::establish_connection();

    let article = db::get_article(id, &connection)
        .ok_or_else(|| error::ErrorNotFound(format!("No article with id {}", id)))?;

    let article = Article::from_model(article, &connection);

    Ok(HttpResponse::Ok().json(article))
}

pub async fn delete_article(req: HttpRequest) -> Result<HttpResponse> {
    let id = req
        .match_info()
        .get("id")
        .ok_or_else(|| error::ErrorBadRequest("No article id"))?
        .parse::<i32>()
        .map_err(|_| error::ErrorBadRequest("Invalid id"))?;

    let connection = db::establish_connection();

    if db::delete_article(id, &connection) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(error::ErrorNotFound(format!("No article with id {}", id)))
    }
}
