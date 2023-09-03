-- Add down migration script here
ALTER TABLE users DROP CONSTRAINT user_id;
DROP TABLE users;
DROP TABLE user_scopes;
DROP TABLE scopes;

CREATE TABLE users {
    user_id VARCHAR(255) NOT NULL UNIQUE,
    scopes text NOT NULL,
    refresh_token VARCHAR(255) NOT NULL,
    access_token VARCHAR(255) NOT NULL,
    expires_in INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
};
