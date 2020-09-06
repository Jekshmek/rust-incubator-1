CREATE TABLE labels (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);


CREATE TABLE articles (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE articles_labels (
    article_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    PRIMARY KEY (article_id, label_id),
    FOREIGN KEY (article_id)
        REFERENCES articles (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION,
    FOREIGN KEY (label_id)
        REFERENCES labels (id)
            ON DELETE CASCADE
            ON UPDATE NO ACTION
)
