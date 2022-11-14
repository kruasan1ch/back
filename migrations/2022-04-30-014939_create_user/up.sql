-- Your SQL goes here
CREATE TABLE users (
    id varchar not null primary key,
    user_name varchar not null,
    password varchar not null,
    email varchar not null unique,
    role int not null default 0
)