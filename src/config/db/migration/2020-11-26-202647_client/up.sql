-- Your SQL goes here
CREATE TABLE clients
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name  VARCHAR
)