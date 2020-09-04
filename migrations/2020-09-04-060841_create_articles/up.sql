CREATE TABLE labels (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);


CREATE TABLE articles (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE articles_labels (
    article_id INTEGER,
    label_id INTEGER,
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
