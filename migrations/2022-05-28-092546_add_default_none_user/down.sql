-- This file should undo anything in `up.sql`
DELETE FROM users WHERE role = -1;