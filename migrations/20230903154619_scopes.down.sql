-- Add down migration script here
DROP TABLE IF EXISTS user_scopes;
DROP TABLE IF EXISTS scopes;
DROP EXTENSION IF EXISTS "pgcrypto";