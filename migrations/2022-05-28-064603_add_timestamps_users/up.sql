-- Your SQL goes here
ALTER TABLE users
ADD COLUMN created_at timestamp not null,
ADD COLUMN updated_at timestamp;