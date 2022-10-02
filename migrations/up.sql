-- Setup tables
create table todo_statuses (
    id varchar(26) not null,
    code varchar(255) not null,
    name varchar(255) not null,
    constraint pk_todo_statuses_id primary key (id)
);

insert into todo_statuses (id, code, name) values ('01GE4ZQCSW8QHKSCA172Q5F358', 'new', '新規');
insert into todo_statuses (id, code, name) values ('01GE4ZQPD3V5AYHZ4WFWHV9Y9S', 'working', '着手中');
insert into todo_statuses (id, code, name) values ('01GE4ZQXGH0S8AWEDFXE5903XR', 'waiting', '未着手');
insert into todo_statuses (id, code, name) values ('01GE50C7RJP4X8WEVWKEATRVKS', 'done', '完了');
insert into todo_statuses (id, code, name) values ('01GE50CDE8K0V9NTPAA6V58XV7', 'discontinued', '中止');
insert into todo_statuses (id, code, name) values ('01GE50CK0PADP4ZH7A7BFHDDH9', 'pending', '保留中');
insert into todo_statuses (id, code, name) values ('01GE50F00G30E08VVZ0PR9QT63', 'deleted', '削除');

create table todos (
    id varchar(26) not null,
    title varchar(255) not null,
    description text not null,
    status_id varchar(26) not null default '01GE4ZQCSW8QHKSCA172Q5F358',
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp,
    constraint pk_todos_id primary key (id),
    constraint fk_todos_status_id_todo_statuses_id foreign key (status_id) references todo_statuses (id)
);

-- Insert sample data
insert into todos (id, title, description) values ('01GDT91DZ0FDZ7YJ426PB189V1', 'todo 1', 'init test data.');
insert into todos (id, title, description) values ('01GDT91MB0SGG49T974GX2A5G9', 'todo 2', '');
update todos set status_id = '01GE4ZQPD3V5AYHZ4WFWHV9Y9S', updated_at = current_timestamp where id = '01GDT91DZ0FDZ7YJ426PB189V1';

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
--     description text not null,
--     is_completed boolean not null default false,
--     created_by varchar(26) not null,
--     created_at timestamp with time zone not null default current_timestamp,
--     updated_by varchar(26) not null,
--     updated_at timestamp with time zone not null default current_timestamp,
--     completed_by varchar(26) not null,
--     completed_at timestamp with time zone,
--     constraint fk_todos_created_by foreign key (created_by) references users (id),
--     constraint fk_todos_updated_by foreign key (updated_by) references users (id),
--     constraint fk_todos_completed_by foreign key (completed_by) references users (id),
-- );
--
-- create index idx_todos_created_by on todos (created_by);
--
-- insert into users (id, name) values ('01GDAQ6WGAE67RY9VWDC80FEWZ', 'Taro Tanaka');
