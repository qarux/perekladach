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
    source_language TEXT NOT NULL,
    target_language TEXT NOT NULL
);

CREATE TABLE chapters(
    index REAL NOT NULL,
    title TEXT NOT NULL,
    project_slug TEXT NOT NULL REFERENCES projects (slug) ON DELETE CASCADE,
    PRIMARY KEY(index, project_slug)
);