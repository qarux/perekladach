-- Add migration script here
CREATE TABLE users(
   id SERIAL PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   password_hash TEXT NOT NULL
);

CREATE TABLE authorization_tokens(
    token TEXT NOT NULL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE projects(
    slug TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    source_lang TEXT NOT NULL,
    target_lang TEXT NOT NULL
);