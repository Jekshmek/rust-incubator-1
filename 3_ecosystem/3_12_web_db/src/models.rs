use super::schema::{articles, articles_labels, labels};

#[derive(Identifiable, Queryable, Associations)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "articles"]
pub struct NewArticle<'title, 'body> {
    pub title: &'title str,
    pub body: &'body str,
}

#[derive(Identifiable, Queryable, Associations)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "labels"]
pub struct NewLabel<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Article)]
#[belongs_to(Label)]
#[primary_key(article_id, label_id)]
#[table_name = "articles_labels"]
pub struct ArticleLabel {
    pub article_id: i32,
    pub label_id: i32,
}

#[derive(Insertable)]
#[table_name = "articles_labels"]
pub struct NewArticleLabel {
    pub article_id: i32,
    pub label_id: i32,
}
