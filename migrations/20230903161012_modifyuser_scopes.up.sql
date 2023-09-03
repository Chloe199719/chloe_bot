-- Add up migration script here
DROP TABLE IF EXISTS user_scopes;

CREATE TABLE IF NOT EXISTS user_scopes (
    id_user uuid,
    id_scope text,
    FOREIGN KEY (id_user) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (id_scope) REFERENCES scopes(name) ON DELETE CASCADE,
    PRIMARY KEY (id_user, id_scope)
);
