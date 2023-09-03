-- Add up migration script here
DROP TABLE IF EXISTS user_scopes;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS scopes;

CREATE TABLE users (
    user_id VARCHAR(255) NOT NULL UNIQUE,
    scopes TEXT NOT NULL,
    refresh_token VARCHAR(255) NOT NULL,
    access_token VARCHAR(255) NOT NULL,
    expires_in INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
);