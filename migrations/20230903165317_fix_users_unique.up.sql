-- Add up migration script here-
ALTER TABLE users ADD CONSTRAINT user_id UNIQUE (user_id);

