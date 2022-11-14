-- Your SQL goes here
ALTER TABLE tasks
ADD CONSTRAINT 
    "FK_tasks_to_user" FOREIGN KEY ("user_id")
    REFERENCES "users"("id");