-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id text NOT NULL,
    access_token  text NOT NULL,
    refresh_token text NOT NULL,
    expires_in    int NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);