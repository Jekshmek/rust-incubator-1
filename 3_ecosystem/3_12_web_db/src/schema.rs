table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}

table! {
    articles_labels (article_id, label_id) {
        article_id -> Integer,
        label_id -> Integer,
    }
}

table! {
    labels (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(articles_labels -> articles (article_id));
joinable!(articles_labels -> labels (label_id));

allow_tables_to_appear_in_same_query!(
    articles,
    articles_labels,
    labels,
);
