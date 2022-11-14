-- This file should undo anything in `up.sql`
ALTER TABLE tasks
DROP CONSTRAINT "FK_tasks_to_user";