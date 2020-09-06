CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4(),
    name VARCHAR(50) UNIQUE NOT NULL,
    pass TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE user_to_user (
    user_1 uuid NOT NULL,
    user_2 uuid NOT NULL,
    PRIMARY KEY (user_1, user_2),
    UNIQUE (user_2, user_1),
    CONSTRAINT fk_user_1
        FOREIGN KEY (user_1)
        REFERENCES users(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_user_2
        FOREIGN KEY (user_2)
        REFERENCES users(id)
        ON DELETE CASCADE
)
