-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name  VARCHAR
)