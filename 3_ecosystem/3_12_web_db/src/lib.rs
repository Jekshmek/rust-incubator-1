#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use crate::models::{Article, ArticleLabel, Label};
use crate::schema::{articles, articles_labels, labels};

#[must_use]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Can`t connect to {}", &db_url))
}

pub fn get_all_articles(conn: &SqliteConnection) -> Vec<Article> {
    articles::table.load(conn).unwrap()
}

pub fn get_article(id: i32, conn: &SqliteConnection) -> Option<Article> {
    articles::table.find(id).first(conn).ok()
}

pub fn delete_article(id: i32, conn: &SqliteConnection) -> bool {
    let rows_changed = diesel::delete(articles::table.find(id))
        .execute(conn)
        .unwrap();
    
    rows_changed == 1
}

pub fn get_labels_for_article(article: &Article, conn: &SqliteConnection) -> Vec<Label> {
    let article_labels_ids: Vec<i32> = ArticleLabel::belonging_to(article)
        .select(articles_labels::columns::label_id)
        .load::<i32>(conn)
        .unwrap();

    labels::table
        .filter(labels::columns::id.eq_any(article_labels_ids))
        .load::<Label>(conn)
        .unwrap()
}

pub fn get_labels(labels: &[String], conn: &SqliteConnection) -> Vec<Label> {
    labels::table
        .filter(labels::columns::name.eq_any(labels))
        .load::<Label>(conn)
        .unwrap()
}
