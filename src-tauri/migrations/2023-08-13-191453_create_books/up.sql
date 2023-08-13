-- Your SQL goes here
CREATE TABLE books
(
    id INTEGER NOT NULL PRIMARY KEY,
    title    VARCHAR NOT NULL,
    author   VARCHAR NOT NULL,
    status   VARCHAR,
    language VARCHAR NOT NULL
)