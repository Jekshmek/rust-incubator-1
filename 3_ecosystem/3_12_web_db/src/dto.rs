use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use step_3_12::{get_labels_for_article, models};

#[derive(Serialize, Deserialize)]
pub struct Article {
    title: String,
    body: String,
    labels: Vec<String>,
}

impl Article {
    pub fn from_model(article: models::Article, conn: &SqliteConnection) -> Self {
        let labels = get_labels_for_article(&article, &conn)
            .into_iter()
            .map(|label| label.name)
            .collect::<Vec<_>>();

        Article {
            title: article.title,
            body: article.body,
            labels,
        }
    }
}

pub struct Label {
    name: String,
}
