-- Add up migration script here

ALTER TABLE channel_commands DROP CONSTRAINT channel_commands_channel_id_fkey;

ALTER TABLE channel_commands ADD CONSTRAINT channel_commands_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES users(user_id) ON DELETE CASCADE;