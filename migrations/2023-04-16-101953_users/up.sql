-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL
);