-- Your SQL goes here
CREATE TABLE tasks(
    id varchar not null primary key,
    title varchar not null,
    body varchar not null,
    done boolean not null default FALSE
)