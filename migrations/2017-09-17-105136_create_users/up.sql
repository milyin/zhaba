create table users (
    id integer not null primary key,
    name varchar not null,
    email varchar not null,
    password_hash bigint not null
);

create view user_infos as select id, name, email from users;

create table posts (
    id integer not null primary key,
    user_id integer not null,
    created bigint not null,
    edited bigint not null,
    title varchar not null,
    body varchar not null
);
