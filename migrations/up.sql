create table todos (
    id varchar(26) not null primary key,
    title varchar(255) not null,
    description text,
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp
);

-- Next step
-- create table users (
--     id varchar(26) not null primary key,
--     name varchar(255) not null,
--     created_at timestamp with time zone not null default current_timestamp,
--     updated_at timestamp with time zone not null default current_timestamp
-- );
--
-- create table todos (
--     id varchar(26) not null primary key,
--     title varchar(255) not null,
--     description text,
--     created_user_id varchar(26) not null,
--     created_at timestamp with time zone not null default current_timestamp,
--     updated_user_id varchar(26) not null,
--     updated_at timestamp with time zone not null default current_timestamp,
--     constraint fk_todos_created_user_id foreign key (created_user_id) references users (id)
--     constraint fk_todos_updated_user_id foreign key (updated_user_id) references users (id)
-- );
--
-- create index idx_todos_created_user_id on todos (created_user_id);
-- create index idx_todos_updated_user_id on todos (updated_user_id);
--
-- insert into users (id, name) values ('01GDAQ6WGAE67RY9VWDC80FEWZ', 'Taro Tanaka');
