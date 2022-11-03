-- Setup tables
create table if not exists todo_statuses (
    id varchar(26) not null,
    code varchar(255) not null,
    name varchar(255) not null,
    constraint pk_todo_statuses_id primary key (id)
);

insert into todo_statuses (id, code, name) values ('01GE4ZQCSW8QHKSCA172Q5F358', 'new', '新規') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE4ZQPD3V5AYHZ4WFWHV9Y9S', 'working', '着手中') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE4ZQXGH0S8AWEDFXE5903XR', 'waiting', '未着手') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE50C7RJP4X8WEVWKEATRVKS', 'done', '完了') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE50CDE8K0V9NTPAA6V58XV7', 'discontinued', '中止') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE50CK0PADP4ZH7A7BFHDDH9', 'pending', '保留中') on conflict do nothing;
insert into todo_statuses (id, code, name) values ('01GE50F00G30E08VVZ0PR9QT63', 'deleted', '削除') on conflict do nothing;

create table if not exists todos (
    id varchar(26) not null,
    title varchar(255) not null,
    description text not null,
    status_id varchar(26) not null default '01GE4ZQCSW8QHKSCA172Q5F358',
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp,
    constraint pk_todos_id primary key (id),
    constraint fk_todos_status_id_todo_statuses_id foreign key (status_id) references todo_statuses (id)
);
