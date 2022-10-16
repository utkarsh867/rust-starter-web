create table users (
   id serial primary key,
   username varchar not null unique,
   password varchar not null
)
