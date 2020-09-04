table! {
    articles (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
    }
}

table! {
    articles_labels (article_id, label_id) {
        article_id -> Nullable<Integer>,
        label_id -> Nullable<Integer>,
    }
}

table! {
    labels (id) {
        id -> Nullable<Integer>,
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
