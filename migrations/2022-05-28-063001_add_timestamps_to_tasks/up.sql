-- Your SQL goes here
ALTER TABLE tasks
ADD COLUMN created_at timestamp not null,
ADD COLUMN updated_at timestamp;

