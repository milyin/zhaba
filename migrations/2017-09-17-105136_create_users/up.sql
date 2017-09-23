create table user (
    id integer not null primary key,
    name varchar not null,
    email varchar not null,
    password_hash bigint not null
);

create view user_info as select id, name, email from user
