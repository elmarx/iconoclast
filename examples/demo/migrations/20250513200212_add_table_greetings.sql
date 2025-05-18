-- Add migration script here
create table greetings
(
    id    serial primary key,
    value text not null
);

