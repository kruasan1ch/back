-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN created_at,
DROP COLUMN updated_at;
