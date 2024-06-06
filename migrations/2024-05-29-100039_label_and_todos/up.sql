-- Your SQL goes here
CREATE TABLE labels
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL
);

CREATE TABLE todos
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR NOT NULL,
    description TEXT   NULL,
    label_id    INTEGER NULL REFERENCES labels(id),
    is_checked  BOOLEAN NOT NULL DEFAULT FALSE
);