use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use step_3_12::{get_labels, get_labels_for_article, models, schema};

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

    pub fn store(&self, conn: &SqliteConnection) {
        let labels = get_labels(self.labels.as_slice(), conn);
        let new_article = models::NewArticle {
            title: self.title.as_str(),
            body: self.body.as_str(),
        };

        conn.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(schema::articles::table)
                .values(&new_article)
                .execute(conn)
                .expect("Error saving new post");

            let article_id = schema::articles::table
                .order(schema::articles::columns::id.desc())
                .select(schema::articles::columns::id)
                .first(conn)
                .unwrap();

            let new_labels = labels
                .into_iter()
                .map(|label| models::NewArticleLabel {
                    article_id,
                    label_id: label.id,
                })
                .collect::<Vec<_>>();

            let x = diesel::insert_into(schema::articles_labels::table)
                .values(new_labels)
                .execute(conn)
                .expect("Error saving article labels");

            Ok(())
        })
        .expect("Transaction failed");
    }
}

pub struct Label {
    name: String,
}
